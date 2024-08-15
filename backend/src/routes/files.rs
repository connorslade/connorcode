use std::{
    fs::{self, File},
    time::UNIX_EPOCH,
};

use afire::{
    extensions::{serve_static::safe_path, RouteShorthands},
    internal::encoding::url,
    Content, HeaderName, Server,
};
use serde::Serialize;
use serde_json::json;

use crate::{app::App, markdown, misc::mime::get_content_type};

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

pub fn attach(server: &mut Server<App>) {
    server.get("/api/files**", move |ctx| {
        let mut local_path: &str = &safe_path(&ctx.req.path[10..]);
        if local_path.starts_with('/') {
            local_path = &local_path[1..];
        }
        let local_path = url::decode(local_path);
        let path = ctx.app().config.files_path.join(&local_path);

        let no_file = ctx.req.query.has("no_file");

        if path.is_dir() {
            let mut children = Vec::new();
            let mut readme = None;

            for file in fs::read_dir(path)?.filter_map(|x| x.ok()) {
                let metadata = file.metadata()?;
                let name = file.file_name().to_string_lossy().into_owned();

                if name.eq_ignore_ascii_case("readme.md") {
                    let contents = fs::read_to_string(file.path())?;
                    readme = Some(markdown::render(&contents, None).html);
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

            children.sort_by(|a, b| b.is_dir.cmp(&a.is_dir).then_with(|| a.name.cmp(&b.name)));

            ctx.content(Content::JSON)
                .header(("X-Response-Type", "DirEntry"))
                .text(json!(DirResponse { children, readme }))
                .send()?;
        } else {
            let ext = path.extension().map(|x| x.to_string_lossy());
            let content = get_content_type(ext.as_deref());

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
