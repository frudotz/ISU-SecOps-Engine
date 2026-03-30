use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::Serialize;

#[derive(Parser)]
#[command(name = "pentest")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Headers {
        url: String,
        #[arg(long, default_value = "md")]
        format: String,
    },
}

#[derive(Serialize)]
struct HeaderResult {
    name: String,
    present: bool,
}

#[derive(Serialize)]
struct Report {
    url: String,
    grade: String,
    headers: Vec<HeaderResult>,
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Headers { url, format } => {
            match analyze_headers(&url).await {
                Ok(report) => {
                    if format == "json" {
                        println!("{}", serde_json::to_string_pretty(&report).unwrap());
                    } else {
                        print_markdown(&report);
                    }
                }
                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

async fn analyze_headers(url: &str) -> Result<Report, Box<dyn std::error::Error>> {
    let client = Client::new();
    let res = client.get(url).send().await?;

    let headers = res.headers();

    let checks = vec![
        "strict-transport-security",
        "content-security-policy",
        "x-frame-options",
        "x-content-type-options",
        "referrer-policy",
        "permissions-policy",
    ];

    let mut results = vec![];
    let mut missing = 0;

    for h in checks {
        let present = headers.get(h).is_some();
        if !present {
            missing += 1;
        }

        results.push(HeaderResult {
            name: h.to_string(),
            present,
        });
    }

    let grade = match missing {
        0 => "A",
        1..=2 => "B",
        3..=4 => "C",
        _ => "F",
    };

    Ok(Report {
        url: url.to_string(),
        grade: grade.to_string(),
        headers: results,
    })
}

fn print_markdown(report: &Report) {
    println!("# Security Report\n");
    println!("**Target:** {}", report.url);
    println!("**Grade:** {}\n", report.grade);

    for h in &report.headers {
        let status = if h.present { "✅ Present" } else { "❌ Missing" };
        println!("- {}: {}", h.name, status);
    }
}