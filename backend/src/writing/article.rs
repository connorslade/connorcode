use std::{
    fmt::{self, Display},
    path::PathBuf,
};

use anyhow::{Context, Result};
use serde::{Deserialize, Deserializer, Serialize};

use crate::markdown;

use super::{Date, Document};

#[derive(Deserialize, Serialize)]
pub struct ArticleFrontMatter {
    #[serde(deserialize_with = "parse_path", serialize_with = "serialize_path")]
    pub path: Path,

    pub title: String,
    pub description: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub hidden: Option<bool>,

    #[serde(flatten)]
    pub date: Date,
    pub tags: Vec<String>,
}

pub struct Path {
    pub category: String,
    pub slug: Option<String>,
}

#[derive(Serialize)]
pub struct ArticleApiResponse<'a> {
    #[serde(flatten)]
    pub front_matter: &'a ArticleFrontMatter,
    pub word_count: u32,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub views: Option<u32>,
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
            views: None,
        }
    }

    pub fn from_document_with_views(
        document: &'a Document<ArticleFrontMatter>,
        views: u32,
    ) -> Self {
        ArticleApiResponse {
            front_matter: &document.front_matter,
            word_count: document.word_count,
            views: Some(views),
        }
    }
}

impl Display for Path {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(slug) = &self.slug {
            write!(f, "{}/{}", self.category, slug)
        } else {
            write!(f, "{}", self.category)
        }
    }
}

fn parse_path<'de, D>(from: D) -> Result<Path, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(from)?;
    if let Some((category, slug)) = str.split_once('/') {
        Ok(Path {
            category: category.to_string(),
            slug: Some(slug.to_string()),
        })
    } else {
        Ok(Path {
            category: str,
            slug: None,
        })
    }
}

fn serialize_path<S>(path: &Path, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    if let Some(slug) = &path.slug {
        serializer.serialize_str(&format!("{}/{}", path.category, slug))
    } else {
        serializer.serialize_str(&path.category)
    }
}
