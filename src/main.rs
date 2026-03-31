#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::{Serialize, Deserialize};
use std::fs;
use std::path::Path;
use axum::{
    routing::{get, post},
    Json, Router
};
use axum::response::Html;

// # CLI
#[derive(Parser)]
#[command(name = "pentest")]
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
    headers: Vec<headerResult> }

// # MAIN
#[tokio::main]
async fn main() {

    let cli = Cli::parse();

    match cli.command {

        Commands::Headers { url, json } => {

            match analyzeHeaders(&url).await {
                Ok(reportData) => {

                    if json != "deny" {
                        saveJson(&reportData) }

                    printMarkdown(&reportData) }

                Err(e) => {
                    eprintln!("Error: {}", e) } } }

        Commands::Web => {
            startWebServer().await } } }

// # ANALİZ
async fn analyzeHeaders(url: &str) -> Result<report, Box<dyn std::error::Error>> {

    // # URL normalize işlemi
    let fixedUrl = if url.starts_with("http://") || url.starts_with("https://") {
        url.to_string()
    } else {
        format!("https://{}", url)
    };

    let client = Client::new();
    let response = client.get(&fixedUrl).send().await?;
    let headers = response.headers();

    let headerInfo = vec![
        ("strict-transport-security", "HIGH", "MITM riski"),
        ("content-security-policy", "HIGH", "XSS riski"),
        ("x-frame-options", "MEDIUM", "Clickjacking riski"),
        ("x-content-type-options", "LOW", "MIME riski"),
        ("referrer-policy", "LOW", "Referer sızıntısı"),
        ("permissions-policy", "LOW", "Browser kontrolsüz") ];

    let mut results = vec![];
    let mut score = 100;

    for (name, severity, risk) in headerInfo {

        let isPresent = headers.get(name).is_some();

        if !isPresent {
            score -= match severity {
                "HIGH" => 20,
                "MEDIUM" => 10,
                _ => 5 } }

        results.push(headerResult {
            name: name.to_string(),
            present: isPresent,
            severity: severity.to_string(),
            risk: if isPresent { "OK".to_string() } else { risk.to_string() } }) }

    let grade = match score {
        90..=100 => "A",
        75..=89 => "B",
        50..=74 => "C",
        _ => "F" };

    Ok(report {
        url: fixedUrl,
        grade: grade.to_string(),
        score,
        headers: results }) }

// # MARKDOWN
fn printMarkdown(r: &report) {

    println!("# Security Report\n");
    println!("{} - {} ({})\n", r.url, r.grade, r.score);

    for h in &r.headers {

        let status = if h.present { "✅" } else { "❌" };

        println!("- {} {} [{}] -> {}", status, h.name, h.severity, h.risk) } }

// # JSON SAVE
fn saveJson(r: &report) {

    let dir = Path::new("assets/reports");

    if !dir.exists() {
        fs::create_dir_all(dir).unwrap() }

    let name = r.url.replace("https://", "").replace("http://", "");
    let path = format!("assets/reports/{}.json", name);

    fs::write(path.clone(), serde_json::to_string_pretty(r).unwrap()).unwrap();

    println!("[+] Saved: {}", path) }

// ======================
// 🌐 WEB SERVER
// ======================

// # SERVER
async fn startWebServer() {

    let app = Router::new()
        .route("/", get(index)) 
        .route("/reports", get(getReports))
        .route("/scan", post(scan));

    println!("Server running: http://127.0.0.1:3000");

    axum::serve(
        tokio::net::TcpListener::bind("127.0.0.1:3000").await.unwrap(),
        app).await.unwrap() }

// # REPORT LIST
async fn getReports() -> Json<Vec<report>> {

    let mut list = vec![];

    if let Ok(entries) = fs::read_dir("assets/reports") {

        for entry in entries {

            if let Ok(file) = fs::read_to_string(entry.unwrap().path()) {
                if let Ok(r) = serde_json::from_str::<report>(&file) {
                    list.push(r) } } } }

    Json(list) }

// # SCAN API
#[derive(Deserialize)]
struct ScanRequest { url: String }

async fn scan(Json(req): Json<ScanRequest>) -> Json<report> {

    let r = analyzeHeaders(&req.url).await.unwrap();
    saveJson(&r);

    Json(r) }

// # INDEX SAYFASI
async fn index() -> Html<String> {

    let html = std::fs::read_to_string("index.html")
        .unwrap_or("<h1>index.html bulunamadı</h1>".to_string());

    Html(html)
}