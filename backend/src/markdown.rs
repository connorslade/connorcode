use comrak::{
    format_html_with_plugins, nodes::NodeValue, parse_document, plugins::syntect::SyntectAdapter,
    Arena, ExtensionOptions, Options, ParseOptions, Plugins, RenderOptions,
};
use latex2mathml::{latex_to_mathml, DisplayStyle};

use std::{io::BufWriter, sync::OnceLock};

const CODE_BLOCK_THEME: &str = "base16-eighties.dark";

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
        let syntect = Box::new(SyntectAdapter::new(Some(CODE_BLOCK_THEME)));
        plugins.render.codefence_syntax_highlighter = Some(Box::leak(syntect));
        plugins
    })
}

pub fn render(markdown: &str) -> String {
    let options = &default_config();
    let arena = Arena::new();

    let root = parse_document(&arena, markdown, options);

    let mut children = vec![root];
    while let Some(child) = children.pop() {
        children.extend(child.children());
        let mut node = child.data.borrow_mut();
        if let NodeValue::Math(math) = &node.value {
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
                    node.value = NodeValue::Text(format!("Error rendering LaTeX to MathML: {e}"))
                }
            }
        }
    }

    let mut bw = BufWriter::new(Vec::new());
    format_html_with_plugins(root, options, &mut bw, default_plugins()).unwrap();
    String::from_utf8(bw.into_inner().unwrap()).unwrap()
}
