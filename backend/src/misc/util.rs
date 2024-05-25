use std::borrow::Cow;

use afire::{error::AnyResult, route::RouteError};

#[macro_export]
macro_rules! regex {
    ($raw:expr) => {{
        static REGEX: once_cell::sync::OnceCell<::regex::Regex> = once_cell::sync::OnceCell::new();
        REGEX.get_or_init(|| ::regex::Regex::new($raw).unwrap())
    }};
}

pub fn bail(msg: &str) -> AnyResult<()> {
    Err(RouteError {
        message: Cow::Owned(msg.to_string()),
        ..Default::default()
    }
    .into())
}
