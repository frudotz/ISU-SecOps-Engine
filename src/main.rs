use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::Serialize;
use std::fs;
use std::path::Path;

// CLI Tanımlama
#[derive(Parser)]
#[command(name = "pentest")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

// Komutlar
#[derive(Subcommand)]
enum Commands {
    Headers {
        url: String,

        #[arg(long, default_value = "allow")]
        json: String
    }
}

// Header Sonuç Yapısı
#[derive(Serialize)]
struct headerResult {
    name: String,
    present: bool
}

// Genel Rapor Yapısı
#[derive(Serialize)]
struct report {
    url: String,
    grade: String,
    headers: Vec<headerResult>
}

#[tokio::main]
async fn main() {

    // CLI Parse İşlemi
    let cli = Cli::parse();

    // Komut Yakalama
    match cli.command {
        Commands::Headers { url, json } => {

            match analyzeHeaders(&url).await {
                Ok(reportData) => {

                    // JSON Kaydetme (default açık)
                    if json != "deny" {
                        saveJson(&reportData);
                    }

                    // Markdown Çıktı
                    printMarkdown(&reportData);
                }

                Err(e) => {
                    eprintln!("Error: {}", e);
                }
            }
        }
    }
}

// Header Analiz İşlemi
async fn analyzeHeaders(url: &str) -> Result<report, Box<dyn std::error::Error>> {

    let client = Client::new();
    let response = client.get(url).send().await?;

    let headers = response.headers();

    let headerList = vec![
        "strict-transport-security",
        "content-security-policy",
        "x-frame-options",
        "x-content-type-options",
        "referrer-policy",
        "permissions-policy"
    ];

    let mut results = vec![];
    let mut missingCount = 0;

    // Header Kontrol Döngüsü
    for h in headerList {

        let isPresent = headers.get(h).is_some();

        if !isPresent {
            missingCount += 1;
        }

        results.push(headerResult {
            name: h.to_string(),
            present: isPresent
        });
    }

    // Grade Hesaplama
    let grade = match missingCount {
        0 => "A",
        1..=2 => "B",
        3..=4 => "C",
        _ => "F"
    };

    Ok(report {
        url: url.to_string(),
        grade: grade.to_string(),
        headers: results
    })
}

// Markdown Çıktı
fn printMarkdown(reportData: &report) {

    println!("# Security Report\n");
    println!("**Target:** {}", reportData.url);
    println!("**Grade:** {}\n", reportData.grade);

    for h in &reportData.headers {

        let status = if h.present { "✅ Present" } else { "❌ Missing" };
        println!("- {}: {}", h.name, status);
    }
}

// JSON Kaydetme İşlemi
fn saveJson(reportData: &report) {

    let dirPath = Path::new("assets/reports");

    if !dirPath.exists() {
        fs::create_dir_all(dirPath).unwrap();
    }

    let cleanUrl = reportData
        .url
        .replace("https://", "")
        .replace("http://", "")
        .replace("/", "_");

    let filePath = format!("assets/reports/{}.json", cleanUrl);

    let jsonData = serde_json::to_string_pretty(reportData).unwrap();

    fs::write(&filePath, jsonData).unwrap();

    println!("\n[+] JSON report saved: {}", filePath);
}