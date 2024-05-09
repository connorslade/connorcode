use afire::extensions::serve_static::{get_type, TYPES};

const MIME_TYPES: &[(&str, &str)] = &[("md", "text/markdown"), ("wasm", "application/wasm")];

pub fn get_content_type(ext: Option<&str>) -> Option<&'static str> {
    ext.and_then(|ext| {
        get_type(ext, &TYPES).or_else(|| MIME_TYPES.iter().find(|x| x.0 == ext).map(|x| x.1))
    })
}
