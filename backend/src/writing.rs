use std::{default, fs};

use anyhow::Result;
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
    }

    todo!()
}