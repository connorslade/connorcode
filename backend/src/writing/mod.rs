use std::{
    cmp::Reverse,
    fs,
    ops::Deref,
    path::{Path, PathBuf},
};

use anyhow::{bail, Context, Result};
use chrono::NaiveDate;
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use tracing::{info, warn};

use crate::markdown::{self, RenderedMarkdown};

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

pub enum DocumentType {
    Article(Article),
    Project(Project),
}

impl Writing {
    pub fn find_article(&self, category: &str, article: &str) -> Option<&Article> {
        self.articles.iter().find(|x| {
            x.front_matter.path.category == category
                && matches!(x.front_matter.path.slug, Some(ref slug) if slug == article)
        })
    }

    pub fn find_article_category(&self, category: &str) -> Option<&Article> {
        self.articles.iter().find(|x| {
            x.front_matter.path.category == category && x.front_matter.path.slug.is_none()
        })
    }

    pub fn find_project(&self, path: &str) -> Option<&Project> {
        self.projects.iter().find(|x| x.front_matter.slug == path)
    }

    pub fn get_articles(&self) -> impl Iterator<Item = &Article> {
        self.articles
            .iter()
            .filter(|x| !x.front_matter.hidden.unwrap_or_default())
    }

    pub fn get_projects(&self) -> impl Iterator<Item = &Project> {
        self.projects.iter().filter(|x| !x.front_matter.hidden)
    }

    pub fn add_document(&mut self, document: DocumentType) {
        match document {
            DocumentType::Article(article) => self.articles.push(article),
            DocumentType::Project(project) => self.projects.push(project),
        }
    }
}

pub fn load(raw_path: &Path) -> Result<Writing> {
    let project_path = &raw_path.join("projects");
    let writing_path = &raw_path.join("writing");

    info!("Generating article cache");

    let cache_path = PathBuf::from(".writing_cache");
    if cache_path.exists() {
        for file in fs::read_dir(&cache_path)?.filter_map(|x| x.ok()) {
            let file_type = file.file_type()?;
            if file_type.is_dir() {
                fs::remove_dir_all(file.path())?;
            } else {
                fs::remove_file(file.path())?;
            }
        }
    } else {
        fs::create_dir(&cache_path)?;
    }

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

        let (document, rendered, relative_path) = match load_document(&path, raw_path) {
            Ok(x) => x,
            Err(e) => {
                warn!("Error loading document: {}", e);
                continue;
            }
        };

        this.add_document(document);

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

pub fn load_document(
    path: &PathBuf,
    writing_dir: &Path,
) -> Result<(DocumentType, RenderedMarkdown, PathBuf)> {
    let project_path = &writing_dir.join("projects");
    let writing_path = &writing_dir.join("writing");

    let contents = fs::read_to_string(path)?;
    let rendered = markdown::render(&contents);

    let Some(front_matter) = &rendered.front_matter else {
        bail!(
            "Article `{}` is missing its frontmatter, skipping",
            path.strip_prefix(writing_dir)?.display()
        );
    };

    let end = front_matter.rfind("---").context("Invalid frontmatter?")?;

    let relative_path;
    let res = if path.starts_with(writing_path) {
        relative_path = path.strip_prefix(writing_path)?.to_path_buf();
        DocumentType::Article(article::load(
            &front_matter[3..end],
            relative_path.clone(),
            rendered.word_count,
        )?)
    } else if path.starts_with(project_path) {
        relative_path = PathBuf::from("projects").join(path.strip_prefix(project_path)?);
        DocumentType::Project(project::load(
            &front_matter[3..end],
            relative_path.clone(),
            rendered.word_count,
        )?)
    } else {
        unreachable!("Path is neither a writing nor project path");
    };

    Ok((res, rendered, relative_path))
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
    Ok(NaiveDate::parse_from_str(&str, "%m-%d-%y")
        .or_else(|_| NaiveDate::parse_from_str(&str, "%m-%d-%Y"))
        .unwrap_or_default())
}

fn stringify_naive_date<S>(date: &NaiveDate, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    serializer.serialize_str(&date.format("%m/%d/%Y").to_string())
}
