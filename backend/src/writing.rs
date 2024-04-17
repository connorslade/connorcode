use std::{default, fs};

use anyhow::Result;
use comrak::{markdown_to_html, ExtensionOptions, Options, ParseOptions, RenderOptions};
use glob::glob;

use crate::markdown;

struct Writing {
    articles: Article,
}

struct Article {
    title: String,
    date: String,
    description: String,
    tags: Vec<String>,
    path: String,
}

fn load() -> Result<Writing> {
    for path in glob("writing/**/*.md")? {
        let path = path?;

        let contents = fs::read_to_string(path)?;
        markdown_to_html(&contents, markdown::default_config());
    }

    todo!()
}
