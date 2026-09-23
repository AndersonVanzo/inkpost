use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// URL from dev.to
    url: String,
}

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
