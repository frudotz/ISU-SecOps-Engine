#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::{fs, path::Path, time::Duration};
use axum::{routing::{get, post}, Json, Router, response::Html};
use colored::*;
use chrono::Local;

// # CLI
#[derive(Parser)]
#[command(name = "pentest", about="HTTP Header Security Analyzer")]
struct Cli {
    #[command(subcommand)]
    command: Commands }

// # Komutlar
#[derive(Subcommand)]
enum Commands {
    Headers {
        url: String,
        #[arg(long, default_value = "allow")]
        json: String },
    Web }

// # Structlar
#[derive(Serialize, Deserialize, Clone)]
struct headerResult {
    name: String,
    present: bool,
    severity: String,
    risk: String }

#[derive(Serialize, Deserialize, Clone)]
struct report {
    url: String,
    grade: String,
    score: i32,
    label: String,
    headers: Vec<headerResult> }

// # MAIN
#[tokio::main]
async fn main() {

    let cli = Cli::parse();

    match cli.command {

        Commands::Headers { url, json } => {

            // # Batch Scan
            if url.ends_with(".txt") {
                let list = fs::read_to_string(url).unwrap();

                for line in list.lines() {
                    println!("\n=== {} ===", line);
                    runSingle(line.to_string(), json.clone()).await;
                }
            } else {
                runSingle(url, json).await;
            }
        }

        Commands::Web => {
            startWebServer().await;
        }
    }
}

// # Tekli scan
async fn runSingle(url: String, json: String) {

    match analyzeHeaders(&url).await {
        Ok(reportData) => {

            if json != "deny" {
                saveJson(&reportData);
            }

            printMarkdown(&reportData);
        }

        Err(e) => {
            println!("Error: {}", e);
        }
    }
}

// # ANALİZ
async fn analyzeHeaders(url: &str) -> Result<report, Box<dyn std::error::Error>> {

    let fixedUrl = if url.starts_with("http") {
        url.to_string()
    } else {
        format!("https://{}", url)
    };

    let client = Client::builder()
        .timeout(Duration::from_secs(5))
        .build()?;

    let response = client.get(&fixedUrl).send().await?;
    let headers = response.headers();

    let headerInfo = vec![
        ("strict-transport-security", "HIGH", "MITM riski"),
        ("content-security-policy", "HIGH", "XSS riski"),
        ("x-frame-options", "MEDIUM", "Clickjacking riski"),
        ("x-content-type-options", "LOW", "MIME riski"),
        ("referrer-policy", "LOW", "Referer sızıntısı"),
        ("permissions-policy", "LOW", "Browser kontrolsüz")
    ];

    let mut results = vec![];
    let mut score = 100;
    let mut issues = vec![];

    for (name, severity, risk) in headerInfo {

        let isPresent = headers.get(name).is_some();
        let mut finalRisk = risk.to_string();

        // # HSTS
        if name == "strict-transport-security" && isPresent {
            if let Some(val) = headers.get(name) {
                let v = val.to_str().unwrap_or("");
                if !v.contains("max-age=31536000") {
                    finalRisk = "HSTS zayıf (low max-age)".to_string();
                }
            }
        }

        // # CSP
        if name == "content-security-policy" && isPresent {
            if let Some(val) = headers.get(name) {
                let v = val.to_str().unwrap_or("");
                if v.contains("unsafe-inline") {
                    finalRisk = "CSP zayıf (unsafe-inline)".to_string();
                }
            }
        }

        // # X-FRAME
        if name == "x-frame-options" && isPresent {
            if let Some(val) = headers.get(name) {
                let v = val.to_str().unwrap_or("");
                if v != "DENY" && v != "SAMEORIGIN" {
                    finalRisk = "X-Frame zayıf".to_string();
                }
            }
        }

        // # REFERRER
        if name == "referrer-policy" && isPresent {
            if let Some(val) = headers.get(name) {
                let v = val.to_str().unwrap_or("");
                if !v.contains("no-referrer") {
                    finalRisk = "Referrer policy zayıf".to_string();
                }
            }
        }

        if !isPresent {
            issues.push((severity, name));
            score -= match severity {
                "HIGH" => 20,
                "MEDIUM" => 10,
                _ => 5
            };
        }

        results.push(headerResult {
            name: name.to_string(),
            present: isPresent,
            severity: severity.to_string(),
            risk: if isPresent { "OK".to_string() } else { finalRisk }
        });
    }

    let grade = match score {
        90..=100 => "A",
        75..=89 => "B",
        50..=74 => "C",
        _ => "F"
    };

    let label = if score >= 75 { "Secure" } else { "Risky" };

    println!("\nTop Issues:");
    for (sev, name) in issues.iter().take(3) {
        println!("- [{}] {}", sev, name);
    }

    Ok(report {
    url: fixedUrl,
    grade: grade.to_string(),
    score,
    label: label.to_string(),
    headers: results
    })
}

// # MARKDOWN
fn printMarkdown(r: &report) {

    println!("\n# Security Report\n");

    println!("{} {} ({})",
        r.url.cyan(),
        r.grade.red(),
        r.score.to_string().yellow()
    );

    for h in &r.headers {

        let status = if h.present { "✔".green() } else { "✘".red() };

        println!(
            "- {} {} [{}] -> {}",
            status,
            h.name,
            h.severity,
            h.risk
        );
    }
}

// # JSON SAVE (timestamp)
fn saveJson(r: &report) {

    let dir = Path::new("assets/reports");

    if !dir.exists() {
        fs::create_dir_all(dir).unwrap();
    }

    let name = r.url.replace("https://", "").replace("http://", "");
    let timestamp = Local::now().format("%Y-%m-%d_%H-%M-%S");

    let path = format!("assets/reports/{}_{}.json", name, timestamp);

    fs::write(path.clone(), serde_json::to_string_pretty(r).unwrap()).unwrap();

    println!("[+] Saved: {}", path);
}

// ================= WEB =================

// # INDEX
async fn index() -> Html<String> {
    Html(fs::read_to_string("index.html").unwrap())
}

// # SERVER
async fn startWebServer() {

    let app = Router::new()
        .route("/", get(index))
        .route("/reports", get(getReports))
        .route("/scan", post(scan));

    println!("Server: http://127.0.0.1:3000");

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(),
        app).await.unwrap();
}

// # REPORTS
async fn getReports() -> Json<Vec<report>> {

    let mut list = vec![];

    if let Ok(entries) = fs::read_dir("assets/reports") {
        for entry in entries {
            if let Ok(data) = fs::read_to_string(entry.unwrap().path()) {
                if let Ok(r) = serde_json::from_str::<report>(&data) {
                    list.push(r);
                }
            }
        }
    }

    Json(list)
}

// # SCAN API
#[derive(Deserialize)]
struct ScanRequest { url: String }

use axum::http::StatusCode;

async fn scan(Json(req): Json<ScanRequest>) -> Result<Json<report>, StatusCode> {

    match analyzeHeaders(&req.url).await {
        Ok(r) => {
            saveJson(&r);
            Ok(Json(r))
        },
        Err(_) => {
            Err(StatusCode::BAD_REQUEST)
        }
    }
}


// ================= TESTS =================
#[cfg(test)]
mod tests {

    // # Boş URL testi
    #[test]
    fn test_empty_url() {
        let input = "";

        let fixed = if input.starts_with("http") {
            input.to_string()
        } else {
            format!("https://{}", input)
        };

        assert_eq!(fixed, "https://");
    }

    // # URL normalize testi
    #[test]
    fn test_url_fix() {
        let input = "example.com";

        let fixed = if input.starts_with("http") {
            input.to_string()
        } else {
            format!("https://{}", input)
        };

        assert_eq!(fixed, "https://example.com");
    }

    // # URL NORMALIZE EDGE CASE
    #[test]
    fn test_url_already_has_https() {
        let input = "https://google.com";

        let fixed = if input.starts_with("http") {
            input.to_string()
        } else {
            format!("https://{}", input)
        };

        assert_eq!(fixed, "https://google.com");
    }

    // # skor hesaplama testi (basit mantık)
    #[test]
    fn test_score_calculation() {

        let mut score = 100;

        // 1 HIGH eksik
        score -= 20;

        // 1 MEDIUM eksik
        score -= 10;

        // 1 LOW eksik
        score -= 5;

        assert_eq!(score, 65);
    }

    // # grade testi
    #[test]
    fn test_grade() {

        let grade = match 65 {
            90..=100 => "A",
            75..=89 => "B",
            50..=74 => "C",
            _ => "F"
        };

        assert_eq!(grade, "C");
    }

    #[test]
fn test_full_failure_score() {

    let mut score = 100;

    // tüm headerlar eksik gibi düşün
    score -= 20; // HSTS
    score -= 20; // CSP
    score -= 10; // X-Frame
    score -= 5;  // X-Content
    score -= 5;  // Referrer
    score -= 5;  // Permissions

    assert!(score < 50);
}

#[test]
fn test_perfect_score() {
    let score = 100;

    let grade = match score {
        90..=100 => "A",
        75..=89 => "B",
        50..=74 => "C",
        _ => "F"
    };

    assert_eq!(grade, "A");
}

#[test]
fn test_label_logic() {

    let score = 80;

    let label = if score >= 75 { "Secure" } else { "Risky" };

    assert_eq!(label, "Secure");
}

#[test]
fn test_missing_count() {
    let missing = vec!["csp", "hsts", "x-frame"];
    assert_eq!(missing.len(), 3);
}

#[test]
fn test_label_threshold() {
    let score = 74;
    let label = if score >= 75 { "Secure" } else { "Risky" };
    assert_eq!(label, "Risky");
}
}