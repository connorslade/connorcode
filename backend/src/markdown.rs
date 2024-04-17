use comrak::{
    markdown_to_html_with_plugins, plugins::syntect::SyntectAdapter, ExtensionOptions, Options,
    ParseOptions, Plugins, RenderOptions,
};

use std::sync::OnceLock;

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
    markdown_to_html_with_plugins(&markdown, default_config(), default_plugins())
}
