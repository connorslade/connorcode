use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::markdown;

use super::{Date, Document};

// TODO: Store category and article in the struct, rather than the path
#[derive(Deserialize, Serialize)]
pub struct ArticleFrontMatter {
    pub path: String,

    pub title: String,
    pub description: String,

    #[serde(flatten)]
    pub date: Date,
    pub tags: Vec<String>,
}

#[derive(Serialize)]
pub struct ArticleApiResponse<'a> {
    #[serde(flatten)]
    pub front_matter: &'a ArticleFrontMatter,
    pub word_count: u32,
}

pub fn load(
    front_matter: &str,
    filesystem_path: PathBuf,
    word_count: u32,
) -> Result<Document<ArticleFrontMatter>> {
    let mut front_matter = serde_yaml::from_str::<ArticleFrontMatter>(front_matter)
        .context("Error parsing frontmatter")?;
    front_matter.description = markdown::render(&front_matter.description).html;
    Ok(Document {
        front_matter,
        filesystem_path,
        word_count,
    })
}

impl<'a> ArticleApiResponse<'a> {
    pub fn from_document(document: &'a Document<ArticleFrontMatter>) -> Self {
        ArticleApiResponse {
            front_matter: &document.front_matter,
            word_count: document.word_count,
        }
    }
}
