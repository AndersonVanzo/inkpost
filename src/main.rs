use clap::Parser;

#[derive(Parser)]
#[command(name = "inkpost")]
#[command(version = "0.1.0")]
#[command(about = "save DEV Community posts to your Kidle")]
struct Cli {
    #[arg(long)]
    url: String,
}

struct Params {
    username: String,
    slug: String,
}

fn parse_params(url: String) -> Params {
    let normalized_url = url.replace("https://", "");
    if !normalized_url.starts_with("dev.to/") {
        panic!("URL must be from dev.to");
    }

    let parts: Vec<&str> = normalized_url.split("/").collect();

    let username = parts[1];
    let slug = parts[2];

    Params {
        username: username.into(),
        slug: slug.into(),
    }
}

fn main() {
    let cli = Cli::parse();

    let params = parse_params(cli.url);

    println!("username: {:?}", params.username);
    println!("slug: {:?}", params.slug);
}
