use afire::{extensions::RouteShorthands, Content, Server};
use indoc::indoc;

use crate::app::App;

pub fn attach(server: &mut Server<App>) {
    server.get("/api/writing/rss", |ctx| {
        let app = ctx.app();

        let mut articles = Vec::new();
        for article in app.writing.read().articles.iter().take(5) {
            articles.push(format!(
                indoc! {
                    r#"<item>
                        <title>{}</title>
                            <description>{}</description>
                            <pubDate>{}</pubDate>
                            <link>{}/writing/{}</link>
                    </item>"#
                },
                article.front_matter.title,
                article.front_matter.description,
                *article.front_matter.date,
                app.config.external_url,
                article.front_matter.path
            ))
        }

        let rss = format!(
            indoc! {
                r#"<?xml version="1.0" encoding="UTF-8" ?>
                <rss version="2.0">
                <channel>
                <title>ConnorCode</title>
                <description>ConnorCode Articles</description>
                <link>{}</link>
                <copyright>Connor Slade</copyright>
                <language>en</language>
                <ttl>1800</ttl>
                {}
                </channel>
                </rss>"#
            },
            app.config.external_url,
            articles.join("\n")
        );

        ctx.text(rss).content(Content::XML).send()?;
        Ok(())
    });
}
