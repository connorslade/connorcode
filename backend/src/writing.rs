use std::{
    cmp::Reverse,
    fs,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::{info, warn};

use crate::markdown;

pub struct Writing {
    pub articles: Vec<Article>,
}

// TODO: Store category and article in the struct, rather than the path
#[derive(Deserialize, Serialize)]
pub struct FrontMatter {
    pub path: String,

    pub title: String,
    pub description: String,

    #[serde(
        deserialize_with = "parse_naive_date",
        serialize_with = "stringify_naive_date"
    )]
    pub date: NaiveDate,
    pub tags: Vec<String>,
}

pub struct Article {
    pub front_matter: FrontMatter,
    pub filesystem_path: PathBuf,
    pub word_count: u32,
}

#[derive(Serialize)]
pub struct ArticleApiResponse<'a> {
    #[serde(flatten)]
    pub front_matter: &'a FrontMatter,
    pub word_count: u32,
}

impl Article {
    pub fn into_api_response(&self) -> ArticleApiResponse {
        ArticleApiResponse {
            front_matter: &self.front_matter,
            word_count: self.word_count,
        }
    }
}

pub fn load(writing_path: &Path) -> Result<Writing> {
    info!("Generating writing cache");

    let cache_path = PathBuf::from(".writing_cache");
    if cache_path.exists() {
        fs::remove_dir_all(".writing_cache")?;
    }
    fs::create_dir(".writing_cache")?;

    let mut articles = Vec::new();

    let mut files = Vec::new();
    files.extend(fs::read_dir(writing_path)?.filter_map(Result::ok));
    while let Some(file) = files.pop() {
        let path = file.path();
        if path.is_dir() {
            files.extend(fs::read_dir(&path)?.filter_map(Result::ok));
            continue;
        }

        if path.extension().map_or(true, |e| e != "md") {
            continue;
        }

        let relative_path = path.strip_prefix(writing_path)?;

        let contents = fs::read_to_string(&path)?;
        let rendered = markdown::render(&contents);

        let Some(front_matter) = rendered.front_matter else {
            warn!(
                "Article `{}` is missing its frontmatter, skipping",
                relative_path.to_string_lossy()
            );
            continue;
        };

        let end = front_matter.rfind("---").context("Invalid frontmatter?")?;
        let front_matter = match serde_yaml::from_str::<FrontMatter>(&front_matter[3..end]) {
            Ok(e) => e,
            Err(e) => {
                warn!(
                    "Error parsing frontmatter for article `{}`, skipping: {e}",
                    relative_path.to_string_lossy()
                );
                continue;
            }
        };

        let article = Article {
            front_matter,
            filesystem_path: relative_path.to_path_buf(),
            word_count: rendered.word_count,
        };

        let new_path = cache_path.join(relative_path).with_extension("html");
        fs::create_dir_all(new_path.parent().unwrap())?;
        fs::write(new_path, rendered.html)?;

        articles.push(article);
    }

    articles.sort_by_key(|x| Reverse(x.front_matter.date));

    Ok(Writing { articles })
}

fn parse_naive_date<'de, D>(from: D) -> Result<NaiveDate, D::Error>
where
    D: Deserializer<'de>,
{
    let str = String::deserialize(from)?;
    Ok(NaiveDate::parse_from_str(&str, "%m-%d-%y").unwrap_or_default())
}

fn stringify_naive_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format("%m/%d/%Y").to_string())
}
