use std::path::PathBuf;

use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};

use crate::markdown;

use super::{Date, Document};

#[derive(Deserialize, Serialize)]
pub struct ProjectFrontMatter {
    pub name: String,
    pub slug: String,

    #[serde(flatten)]
    pub date: Date,
    pub description: String,

    pub github: Option<String>,
    pub link: Option<String>,
}

#[derive(Serialize)]
pub struct ProjectApiResponse<'a> {
    #[serde(flatten)]
    pub front_matter: &'a ProjectFrontMatter,
    pub word_count: u32,
}

pub fn load(
    front_matter: &str,
    filesystem_path: PathBuf,
    word_count: u32,
) -> Result<Document<ProjectFrontMatter>> {
    let mut front_matter = serde_yaml::from_str::<ProjectFrontMatter>(front_matter)
        .context("Error parsing frontmatter")?;
    front_matter.description = markdown::render(&front_matter.description).html;
    Ok(Document {
        front_matter,
        filesystem_path,
        word_count,
    })
}

impl<'a> ProjectApiResponse<'a> {
    pub fn from_document(document: &'a Document<ProjectFrontMatter>) -> ProjectApiResponse<'a> {
        ProjectApiResponse {
            front_matter: &document.front_matter,
            word_count: document.word_count,
        }
    }
}
