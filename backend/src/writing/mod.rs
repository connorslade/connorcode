use std::{
    cmp::Reverse,
    fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::{Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::{info, warn};

use crate::markdown;

use self::{article::ArticleFrontMatter, project::ProjectFrontMatter};
pub mod article;
pub mod project;

pub type Article = Document<ArticleFrontMatter>;
pub type Project = Document<ProjectFrontMatter>;

#[derive(Default)]
pub struct Writing {
    pub articles: Vec<Article>,
    pub projects: Vec<Project>,
}

pub struct Document<FrontMatter> {
    pub front_matter: FrontMatter,
    pub filesystem_path: PathBuf,
    pub word_count: u32,
}

impl Writing {
    pub fn find_article(&self, category: &str, article: &str) -> Option<&Article> {
        self.articles.iter().find(|x| {
            x.front_matter.path.category == category && x.front_matter.path.slug == article
        })
    }

    pub fn find_project(&self, path: &str) -> Option<&Project> {
        self.projects.iter().find(|x| x.front_matter.slug == path)
    }
}

pub fn load(raw_path: &Path) -> Result<Writing> {
    let project_path = &raw_path.join("projects");
    let writing_path = &raw_path.join("writing");

    info!("Generating article cache");

    let cache_path = PathBuf::from(".writing_cache");
    if cache_path.exists() {
        fs::remove_dir_all(".writing_cache")?;
    }
    fs::create_dir(".writing_cache")?;

    let mut this = Writing::default();

    let mut files = Vec::new();
    files.extend(fs::read_dir(writing_path)?.filter_map(Result::ok));
    files.extend(fs::read_dir(project_path)?.filter_map(Result::ok));
    while let Some(file) = files.pop() {
        let path = file.path();
        if path.is_dir() {
            files.extend(fs::read_dir(&path)?.filter_map(Result::ok));
            continue;
        }

        if path.extension().map_or(true, |e| e != "md") {
            continue;
        }

        let contents = fs::read_to_string(&path)?;
        let rendered = markdown::render(&contents);

        let Some(front_matter) = rendered.front_matter else {
            warn!(
                "Article `{}` is missing its frontmatter, skipping",
                path.strip_prefix(raw_path)?.display()
            );
            continue;
        };

        let end = front_matter.rfind("---").context("Invalid frontmatter?")?;

        let relative_path;
        if path.starts_with(writing_path) {
            relative_path = path.strip_prefix(writing_path)?;
            this.articles.push(article::load(
                &front_matter[3..end],
                path.to_path_buf(),
                rendered.word_count,
            )?);
        } else if path.starts_with(project_path) {
            relative_path = &path;
            this.projects.push(project::load(
                &front_matter[3..end],
                path.to_path_buf(),
                rendered.word_count,
            )?);
        } else {
            unreachable!("Path is neither a writing nor project path");
        }

        let new_path = cache_path.join(relative_path).with_extension("html");
        fs::create_dir_all(new_path.parent().unwrap())?;
        fs::write(new_path, rendered.html)?;
    }

    this.articles.sort_by_key(|x| Reverse(*x.front_matter.date));
    this.projects.sort_by_key(|x| Reverse(*x.front_matter.date));

    info!(
        "Article cache generated for {} articles and {} projects",
        this.articles.len(),
        this.projects.len()
    );

    Ok(this)
}

#[derive(Deserialize, Serialize)]
pub struct Date {
    #[serde(
        deserialize_with = "parse_naive_date",
        serialize_with = "stringify_naive_date"
    )]
    date: NaiveDate,
}

impl Deref for Date {
    type Target = NaiveDate;

    fn deref(&self) -> &Self::Target {
        &self.date
    }
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
