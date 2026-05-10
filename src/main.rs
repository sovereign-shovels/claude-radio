use anyhow::Result;
use clap::{Parser, Subcommand};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "claude-radio")]
#[command(about = "Push-to-talk voice loop with any LLM.")]
#[command(version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Fetch RSS and generate podcast script
    Generate {
        /// RSS feed URL
        #[arg(short, long)]
        feed: String,
        /// Output file
        #[arg(short, long, default_value = "podcast-script.txt")]
        output: PathBuf,
        /// LLM endpoint
        #[arg(short, long, default_value = "http://localhost:11434/v1/chat/completions")]
        endpoint: String,
        /// Model name
        #[arg(short, long, default_value = "llama3.2")]
        model: String,
    },
}

#[derive(Debug, Serialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Serialize)]
struct Message {
    role: String,
    content: String,
}

#[derive(Debug, Deserialize)]
struct ChatResponse {
    choices: Vec<Choice>,
}

#[derive(Debug, Deserialize)]
struct Choice {
    message: ResponseMessage,
}

#[derive(Debug, Deserialize)]
struct ResponseMessage {
    content: String,
}

async fn fetch_rss(url: &str) -> Result<String> {
    let body = reqwest::get(url).await?.text().await?;
    let mut titles = Vec::new();
    for line in body.lines() {
        if let Some(start) = line.find("<title>") {
            if let Some(end) = line.find("</title>") {
                let title = line[start + 7..end].trim();
                if !title.is_empty() && title != "RSS Feed" {
                    titles.push(title.to_string());
                }
            }
        }
    }
    Ok(titles.into_iter().take(5).collect::<Vec<_>>().join(". "))
}

async fn generate_script(endpoint: &str, model: &str, articles: &str) -> Result<String> {
    let client = reqwest::Client::new();
    let body = ChatRequest {
        model: model.into(),
        messages: vec![
            Message {
                role: "system".into(),
                content: "You are a podcast host. Write a 2-minute podcast script summarizing these news items in an engaging, conversational style.".into(),
            },
            Message {
                role: "user".into(),
                content: articles.into(),
            },
        ],
        stream: false,
    };

    let res = client.post(endpoint).json(&body).send().await?;
    if !res.status().is_success() {
        return Err(anyhow::anyhow!("LLM error: {}", res.status()));
    }

    let data: ChatResponse = res.json().await?;
    Ok(data.choices.into_iter().next().map(|c| c.message.content).unwrap_or_default())
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    match cli.command {
        Commands::Generate {
            feed,
            output,
            endpoint,
            model,
        } => {
            println!("Fetching RSS...");
            let articles = fetch_rss(&feed).await?;
            println!("Generating script...");
            let script = generate_script(&endpoint, &model, &articles).await?;
            tokio::fs::write(&output, script).await?;
            println!("Saved script to {}", output.display());
        }
    }
    Ok(())
}
