use std::{fs, path::PathBuf};

use anyhow::Result;
use glob::glob;
use serde::{Deserialize, Serialize};
use tracing::{info, warn};

use crate::markdown;

pub struct Writing {
    pub articles: Vec<Article>,
}

#[derive(Serialize, Deserialize)]
pub struct Article {
    pub date: String,
    pub title: String,
    pub description: String,
    pub tags: Vec<String>,
    pub path: String,
}

pub fn load() -> Result<Writing> {
    info!("Generating writing cache");

    let cache_path = PathBuf::from(".writing_cache");
    if cache_path.exists() {
        fs::remove_dir_all(".writing_cache")?;
    }
    fs::create_dir(".writing_cache")?;

    let mut articles = Vec::new();
    for path in glob("writing/**/*.md")? {
        let path = path?;
        let relative_path = path.strip_prefix("writing/")?;

        let contents = fs::read_to_string(&path)?;
        let rendered = markdown::render(&contents);

        let Some(front_matter) = rendered.front_matter else {
            warn!(
                "Article `{}` is missing its frontmatter, skipping",
                relative_path.to_string_lossy()
            );
            continue;
        };
        let front_matter =
            match serde_yaml::from_str::<Article>(&front_matter[4..front_matter.len() - 6]) {
                Ok(e) => e,
                Err(e) => {
                    warn!(
                        "Error parsing frontmatter for article `{}`, skipping: {e}",
                        relative_path.to_string_lossy()
                    );
                    continue;
                }
            };

        let new_path = cache_path.join(relative_path).with_extension("html");
        fs::create_dir_all(new_path.parent().unwrap())?;
        fs::write(new_path, rendered.html)?;

        articles.push(front_matter);
    }

    Ok(Writing { articles })
}
