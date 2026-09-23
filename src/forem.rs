use anyhow::{Context, bail};
use reqwest::{Client, StatusCode};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Article {
    pub title: String,
    pub body_html: String,
    pub reading_time_minutes: u32,
    pub url: String,
    pub user: User,
    pub cover_image: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct User {
    pub name: String,
}

#[derive(Debug)]
pub struct Params {
    owner: String,
    slug: String,
}

pub fn parse_params(url: &str) -> anyhow::Result<Params> {
    let normalized_url = url.strip_prefix("https://").unwrap_or(url);
    let normalized_url = normalized_url
        .strip_prefix("http://")
        .unwrap_or(normalized_url);

    let url_params = normalized_url
        .split(['#', '?'])
        .next()
        .unwrap_or(normalized_url);

    let parts: Vec<&str> = url_params.split("/").collect();

    let (owner, slug) = match parts.as_slice() {
        ["dev.to", first, second, ..] => (*first, *second),
        [_, _, _, ..] => bail!("Invalid URL: must be from dev.to"),
        _ => bail!("Invalid URL: not enough params"),
    };

    if owner.is_empty() {
        bail!("Invalid URL: owner is empty");
    }

    if slug.is_empty() {
        bail!("Invalid URL: slug is empty");
    }

    Ok(Params {
        owner: owner.to_owned(),
        slug: slug.to_owned(),
    })
}

pub async fn fetch_article(params: &Params, client: &Client) -> anyhow::Result<Article> {
    let req_url = format!(
        "https://dev.to/api/articles/{}/{}",
        params.owner, params.slug
    );

    let res = client
        .get(req_url)
        .send()
        .await
        .context("couldn't reach dev.to")?;

    if res.status() == StatusCode::NOT_FOUND {
        bail!("article not found");
    }

    let res = res.error_for_status()?;

    res.json::<Article>()
        .await
        .context("unexpected response from dev.to")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_https_url() {
        let parse = parse_params("https://dev.to/ben/some-post").expect("parse_params should pass");
        assert_eq!(parse.owner, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_parse_http_url() {
        let parse = parse_params("http://dev.to/ben/some-post").expect("parse_params should pass");
        assert_eq!(parse.owner, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_parse_no_scheme() {
        let parse = parse_params("dev.to/ben/some-post").expect("parse_params should pass");
        assert_eq!(parse.owner, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_trailing_slash() {
        let parse = parse_params("dev.to/ben/some-post/").expect("parse_params should pass");
        assert_eq!(parse.owner, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_query_params() {
        let parse =
            parse_params("dev.to/ben/some-post?utm_source=xyz").expect("parse_params should pass");
        assert_eq!(parse.owner, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_fragments() {
        let parse =
            parse_params("dev.to/ben/some-post#comments").expect("parse_params should pass");
        assert_eq!(parse.owner, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_extra_params() {
        let parse =
            parse_params("dev.to/ben/some-post/comments").expect("parse_params should pass");
        assert_eq!(parse.owner, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_fragments_params_and_query_params() {
        let parse = parse_params("dev.to/ben/some-post/comments?utm_source=xyz#comments")
            .expect("parse_params should pass");
        assert_eq!(parse.owner, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_reject_slug_missing() {
        parse_params("dev.to/ben").unwrap_err();
    }

    #[test]
    fn should_reject_only_doman() {
        parse_params("dev.to").unwrap_err();
    }

    #[test]
    fn should_reject_owner_missing() {
        parse_params("dev.to//some-post").unwrap_err();
    }

    #[test]
    fn should_reject_empty_slug() {
        parse_params("dev.to/ben//").unwrap_err();
    }

    #[test]
    fn should_reject_wrong_site() {
        parse_params("example.to/ben/some-post").unwrap_err();
    }

    #[test]
    fn should_reject_wrong_site_with_few_arguments() {
        parse_params("example.to/ben").unwrap_err();
    }

    #[test]
    fn should_reject_look_alike_doman() {
        parse_params("dev.to.invalid/ben/some-post").unwrap_err();
    }

    #[test]
    fn should_reject_empty_url() {
        parse_params("").unwrap_err();
    }
}
