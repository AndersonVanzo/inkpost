mod forem;

use clap::Parser;

#[derive(Parser)]
#[command(version, about)]
struct Cli {
    /// URL from dev.to
    url: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let cli = Cli::parse();

    let client = reqwest::Client::builder()
        .user_agent(format!("inkpost-cli/{}", env!("CARGO_PKG_VERSION")))
        .build()?;

    let params = forem::parse_params(&cli.url)?;

    let article = forem::fetch_article(&params, &client).await?;
    println!("title: {}", article.title);
    println!("author: {}", article.user.name);
    println!("reading time: {} min", article.reading_time_minutes);

    let content_chars: String = article.body_html.chars().take(200).collect();
    println!("content: {}", content_chars);

    Ok(())
}
