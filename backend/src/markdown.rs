use std::{cell::RefCell, collections::VecDeque, io::BufWriter, sync::OnceLock};

use comrak::{
    arena_tree::Node,
    format_html_with_plugins,
    nodes::{Ast, NodeValue},
    parse_document,
    plugins::syntect::SyntectAdapterBuilder,
    Arena, ExtensionOptions, Options, ParseOptions, Plugins, RenderOptions,
};
use latex2mathml::{latex_to_mathml, DisplayStyle};
use syntect::{highlighting::ThemeSet, parsing::SyntaxSet};

use crate::regex;

pub struct RenderedMarkdown {
    pub html: String,
    pub front_matter: Option<String>,
    pub word_count: u32,
}

pub fn default_config() -> &'static Options {
    static CELL: OnceLock<Options> = OnceLock::new();

    CELL.get_or_init(|| {
        let mut extension = ExtensionOptions::default();
        extension.strikethrough = true;
        extension.table = true;
        extension.autolink = true;
        extension.tasklist = true;
        extension.superscript = true;
        extension.header_ids = Some(String::new());
        extension.footnotes = true;
        extension.description_lists = true;
        extension.front_matter_delimiter = Some("---".to_owned());
        extension.multiline_block_quotes = true;
        extension.math_dollars = true;

        let mut parse = ParseOptions::default();
        parse.smart = true;

        let mut render = RenderOptions::default();
        render.unsafe_ = true;

        Options {
            extension,
            parse,
            render,
        }
    })
}

pub fn default_plugins() -> &'static Plugins<'static> {
    static CELL: OnceLock<Plugins> = OnceLock::new();

    CELL.get_or_init(|| {
        let mut plugins = Plugins::default();

        let mut theme_set = ThemeSet::new();
        let theme = ThemeSet::get_theme("assets/OneDark.tmTheme").unwrap();
        theme_set.themes.insert("Atom One Dark".to_owned(), theme);

        let mut syntax_set = SyntaxSet::load_defaults_newlines().into_builder();
        syntax_set.add_from_folder("assets", true).unwrap();

        let syntect = Box::new(
            SyntectAdapterBuilder::new()
                .syntax_set(syntax_set.build())
                .theme_set(theme_set)
                .theme("Atom One Dark")
                .build(),
        );

        plugins.render.codefence_syntax_highlighter = Some(Box::leak(syntect));
        plugins
    })
}

pub fn render(markdown: &str) -> RenderedMarkdown {
    let options = &default_config();
    let arena = Arena::new();

    let root = parse_document(&arena, markdown, options);
    let mut front_matter = None;
    let mut word_count = 0;

    let mut count_words = |text: &str| {
        if !text.is_empty() {
            word_count += 1;
        }

        let mut last_whitespace = true;
        for char in text.chars() {
            let is_whitespace = char.is_whitespace();
            word_count += (is_whitespace && !last_whitespace) as u8 as u32;
            last_whitespace = is_whitespace;
        }
    };

    let mut children = VecDeque::from_iter([root]);
    while let Some(child) = children.pop_front() {
        children.extend(child.children());
        let mut node = child.data.borrow_mut();

        match &node.value {
            NodeValue::Text(text) => count_words(text),
            NodeValue::Code(code) => count_words(&code.literal),
            NodeValue::Link(link) => count_words(&link.title),
            _ => {}
        }

        match &node.value {
            NodeValue::FrontMatter(matter) => front_matter = Some(matter.to_owned()),
            NodeValue::Math(math) => {
                let mathml = latex_to_mathml(
                    &math.literal,
                    if math.display_math {
                        DisplayStyle::Block
                    } else {
                        DisplayStyle::Inline
                    },
                );
                match mathml {
                    Ok(mathml) => node.value = NodeValue::HtmlInline(mathml),
                    Err(e) => {
                        node.value =
                            NodeValue::Text(format!("Error rendering LaTeX to MathML: {e}"))
                    }
                }
            }
            NodeValue::BlockQuote => {
                let mut children = VecDeque::from_iter(child.children());
                while let Some(this_child) = children.pop_front() {
                    children.extend(this_child.children());
                    let this_node = this_child.data.borrow();
                    if let NodeValue::Text(text) = &this_node.value {
                        let Some(captures) = regex!(r"\[(.*)\] (.*)").captures(text) else {
                            break;
                        };

                        this_child.detach();
                        node.value = NodeValue::HtmlInline(format!(
                            r#"<div element="admonition" type="{}" title="{}">"#,
                            &captures[1], &captures[2]
                        ));

                        let ast =
                            Ast::new(NodeValue::HtmlInline("</div>".to_owned()), (0, 0).into());
                        let node = Node::new(RefCell::new(ast));
                        child.append(arena.alloc(node));
                        break;
                    }
                }
            }
            _ => {}
        }
    }

    let mut buf = BufWriter::new(Vec::new());
    format_html_with_plugins(root, options, &mut buf, default_plugins()).unwrap();
    let html = String::from_utf8(buf.into_inner().unwrap()).unwrap();

    RenderedMarkdown {
        html,
        front_matter,
        word_count,
    }
}
