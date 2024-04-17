use std::{
    fs::{self, File},
    path::PathBuf,
    time::UNIX_EPOCH,
};

use afire::{
    extensions::serve_static::{get_type, TYPES},
    extensions::{serve_static::safe_path, RouteShorthands},
    internal::encoding::url,
    Content, HeaderName, Server,
};
use pulldown_cmark::{Options, Parser};
use serde::Serialize;
use serde_json::json;

use crate::app::App;

#[derive(Serialize)]
struct DirResponse {
    children: Vec<DirEntry>,
    readme: Option<String>,
}

#[derive(Serialize)]
struct DirEntry {
    path: String,
    name: String,
    is_dir: bool,
    size: u64,
    last_modified: u64,
}

const MIME_TYPES: &[(&str, &str)] = &[("md", "text/markdown"), ("wasm", "application/wasm")];

pub fn attach(server: &mut Server<App>) {
    server.get("/api/files**", |ctx| {
        let base_path = PathBuf::from("files");
        let mut local_path: &str = &safe_path(&ctx.req.path[10..]);
        if local_path.starts_with('/') {
            local_path = &local_path[1..];
        }
        let local_path = url::decode(local_path);
        let path = base_path.join(&local_path);

        let no_file = ctx.req.query.has("no_file");

        if path.is_dir() {
            let mut children = Vec::new();
            let mut readme = None;

            for file in fs::read_dir(path)?.into_iter().filter_map(|x| x.ok()) {
                let metadata = file.metadata()?;
                let name = file.file_name().to_string_lossy().into_owned();

                if name.eq_ignore_ascii_case("readme.md") {
                    let contents = fs::read_to_string(file.path())?;

                    let mut options = Options::empty();
                    options.insert(Options::ENABLE_STRIKETHROUGH);
                    let parser = Parser::new_ext(&contents, options);

                    let mut html = String::new();
                    pulldown_cmark::html::push_html(&mut html, parser);
                    readme = Some(html);
                }

                children.push(DirEntry {
                    path: if local_path.is_empty() {
                        name.to_owned()
                    } else {
                        format!("{local_path}/{name}")
                    },
                    name,
                    is_dir: metadata.is_dir(),
                    size: metadata.len(),
                    last_modified: metadata
                        .modified()?
                        .duration_since(UNIX_EPOCH)
                        .unwrap()
                        .as_secs(),
                });
            }

            children.sort_by(|a, b| a.name.cmp(&b.name));

            ctx.content(Content::JSON)
                .header(("X-Response-Type", "DirEntry"))
                .text(json!(DirResponse { children, readme }))
                .send()?;
        } else {
            let ext = path.extension().map(|x| x.to_string_lossy());
            let content = ext
                .map(|ext| {
                    get_type(&ext, &TYPES)
                        .or_else(|| MIME_TYPES.iter().find(|x| x.0 == ext).map(|x| x.1))
                })
                .flatten();

            if !no_file {
                let file = File::open(path)?;
                ctx.stream(file);
            }

            ctx.header(("X-Response-Type", "File"))
                .header((
                    HeaderName::ContentType,
                    content.unwrap_or("application/octet-stream"),
                ))
                .send()?;
        }

        Ok(())
    });
}
