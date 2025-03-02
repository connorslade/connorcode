use std::{
    fs::{self, File},
    time::UNIX_EPOCH,
};

use afire::{
    extensions::{serve_static::safe_path, RedirectResponse, RouteShorthands},
    headers::Vary,
    internal::encoding::url,
    Content, HeaderName, Server,
};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::{app::App, markdown, misc::mime::get_content_type};

#[derive(Serialize)]
struct DirResponse {
    children: Vec<DirEntry>,
    readme: Option<String>,
    description: Option<String>,
}

#[derive(Serialize)]
struct DirEntry {
    path: String,
    name: String,
    is_dir: bool,
    size: u64,
    last_modified: u64,
}

#[derive(Default, Deserialize)]
#[serde(default)]
struct Config {
    hidden: Vec<String>,
    description: Option<String>,
}

pub fn attach(server: &mut Server<App>) {
    server.get("/api/files**", move |ctx| {
        let app = ctx.app();
        let externial_url = &app.config.external_url;

        let mut local_path: &str = &safe_path(&ctx.req.path[10..]);
        if local_path.starts_with('/') {
            local_path = &local_path[1..];
        }
        let local_path = url::decode(local_path);
        let path = app.config.files_path.join(&local_path);

        if path.is_dir() {
            // Redirect to frontend file browser if the request is not an API
            // request. This is to prevent users from seeing the raw json data
            // if they modify the path manually.
            ctx.header(Vary::headers([HeaderName::Accept]));
            if let Some(accept) = ctx.req.headers.get(HeaderName::Accept) {
                if accept.contains("text/html") {
                    ctx.redirect(format!("{externial_url}/files/{local_path}"))
                        .send()?;
                    return Ok(());
                }
            }

            let files = fs::read_dir(path)?
                .filter_map(|x| x.ok())
                .filter_map(|file| Some((file.metadata().ok()?, file_name(&file), file)))
                .collect::<Vec<_>>();

            let readme = special_file(&files, "readme.md").and_then(|(_, _, file)| {
                let contents = fs::read_to_string(file.path()).ok()?;
                Some(markdown::render(&contents, None).html)
            });

            let config = special_file(&files, "config.toml").and_then(|(_, _, file)| {
                let contents = fs::read_to_string(file.path()).ok()?;
                toml::from_str::<Config>(&contents).ok()
            });

            let mut children = files
                .iter()
                .filter(|(_, name, _)| {
                    config.as_ref().map_or(true, |x| !x.hidden.contains(name))
                        && "config.toml" != name
                })
                .map(|(metadata, name, _)| DirEntry {
                    path: if local_path.is_empty() {
                        name.to_owned()
                    } else {
                        format!("{local_path}/{name}")
                    },
                    name: name.to_string(),
                    is_dir: metadata.is_dir(),
                    size: metadata.len(),
                    last_modified: file_last_modified(metadata),
                })
                .collect::<Vec<_>>();

            children.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));

            ctx.content(Content::JSON)
                .header(("X-Response-Type", "DirEntry"))
                .text(json!(DirResponse {
                    children,
                    readme,
                    description: config.and_then(|x| x.description)
                }))
                .send()?;
        } else {
            let ext = path.extension().map(|x| x.to_string_lossy());
            let content = get_content_type(ext.as_deref());

            if !ctx.req.query.has("no_file") {
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

fn file_last_modified(metadata: &fs::Metadata) -> u64 {
    metadata
        .modified()
        .unwrap()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs()
}

fn file_name(file: &fs::DirEntry) -> String {
    file.file_name().to_string_lossy().into_owned()
}

fn special_file<'a>(
    files: &'a [(fs::Metadata, String, fs::DirEntry)],
    name: &str,
) -> Option<&'a (fs::Metadata, String, fs::DirEntry)> {
    files
        .iter()
        .find(|(_, file_name, _)| file_name.eq_ignore_ascii_case(name))
}
