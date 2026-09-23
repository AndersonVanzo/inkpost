use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// URL from dev.to
    url: String,
}

#[derive(Debug)]
struct Params {
    username: String,
    slug: String,
}

fn parse_params(url: &str) -> Result<Params, String> {
    let normalized_url = url.strip_prefix("https://").unwrap_or(url);
    let normalized_url = normalized_url
        .strip_prefix("http://")
        .unwrap_or(normalized_url);

    let url_params = normalized_url
        .split(['#', '?'])
        .next()
        .unwrap_or(normalized_url);

    let parts: Vec<&str> = url_params.split("/").collect();

    let (username, slug) = match parts.as_slice() {
        ["dev.to", first, second, ..] => (*first, *second),
        [_, _, _, ..] => return Err("Invalid URL: must be from dev.to".into()),
        _ => return Err("Invalid URL: not enough params".into()),
    };

    if username.is_empty() {
        return Err("Invalid URL: username is empty".into());
    }

    if slug.is_empty() {
        return Err("Invalid URL: slug is empty".into());
    }

    Ok(Params {
        username: username.to_owned(),
        slug: slug.to_owned(),
    })
}

fn main() {
    let cli = Cli::parse();

    let params = match parse_params(&cli.url) {
        Ok(params) => params,
        Err(err) => {
            eprintln!("{}", err);
            std::process::exit(1);
        }
    };

    println!("username: {}", params.username);
    println!("slug: {}", params.slug);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_https_url() {
        let parse = parse_params("https://dev.to/ben/some-post").expect("parse_params should pass");
        assert_eq!(parse.username, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_parse_http_url() {
        let parse = parse_params("http://dev.to/ben/some-post").expect("parse_params should pass");
        assert_eq!(parse.username, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_parse_no_scheme() {
        let parse = parse_params("dev.to/ben/some-post").expect("parse_params should pass");
        assert_eq!(parse.username, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_trailing_slash() {
        let parse = parse_params("dev.to/ben/some-post/").expect("parse_params should pass");
        assert_eq!(parse.username, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_query_params() {
        let parse =
            parse_params("dev.to/ben/some-post?utm_source=xyz").expect("parse_params should pass");
        assert_eq!(parse.username, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_fragments() {
        let parse =
            parse_params("dev.to/ben/some-post#comments").expect("parse_params should pass");
        assert_eq!(parse.username, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_extra_params() {
        let parse =
            parse_params("dev.to/ben/some-post/comments").expect("parse_params should pass");
        assert_eq!(parse.username, "ben");
        assert_eq!(parse.slug, "some-post");
    }

    #[test]
    fn should_accept_fragments_params_and_query_params() {
        let parse = parse_params("dev.to/ben/some-post/comments?utm_source=xyz#comments")
            .expect("parse_params should pass");
        assert_eq!(parse.username, "ben");
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
    fn should_reject_username_missing() {
        parse_params("dev.to//some-post").unwrap_err();
    }

    #[test]
    fn should_reject_empty_slug() {
        parse_params("dev.to/username//").unwrap_err();
    }

    #[test]
    fn should_reject_wrong_site() {
        parse_params("example.to/username/some-post").unwrap_err();
    }

    #[test]
    fn should_reject_wrong_site_with_few_arguments() {
        parse_params("example.to/username").unwrap_err();
    }

    #[test]
    fn should_reject_look_alike_doman() {
        parse_params("dev.to.invalid/username/some-post").unwrap_err();
    }

    #[test]
    fn should_reject_empty_url() {
        parse_params("").unwrap_err();
    }
}
