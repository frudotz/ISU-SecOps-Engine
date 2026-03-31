#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

use clap::{Parser, Subcommand};
use reqwest::Client;
use serde::Serialize;
use std::fs;
use std::path::Path;

// # CLI Tanımlama
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
        json: String } }

// # Header Sonuç Yapısı
#[derive(Serialize)]
struct headerResult {
    name: String,
    present: bool,
    severity: String,
    risk: String }

// # Genel Rapor Yapısı
#[derive(Serialize)]
struct report {
    url: String,
    grade: String,
    score: i32,
    headers: Vec<headerResult> }

#[tokio::main]
async fn main() {

    // # CLI Parse
    let cli = Cli::parse();

    // # Komut Yakalama
    match cli.command {
        Commands::Headers { url, json } => {

            match analyzeHeaders(&url).await {
                Ok(reportData) => {

                    // # JSON Kaydetme (default açık)
                    if json != "deny" {
                        saveJson(&reportData) }

                    // # Terminal Çıktı
                    printMarkdown(&reportData) }

                Err(e) => {
                    eprintln!("Error: {}", e) } } } } }

// # Header Analiz İşlemi
async fn analyzeHeaders(url: &str) -> Result<report, Box<dyn std::error::Error>> {

    let client = Client::new();
    let response = client.get(url).send().await?;
    let headers = response.headers();

    // # Header Bilgi Tanımları
    let headerInfo = vec![
        ("strict-transport-security", "HIGH", "MITM saldırılarına açık olabilir"),
        ("content-security-policy", "HIGH", "XSS saldırılarına açık olabilir"),
        ("x-frame-options", "MEDIUM", "Clickjacking saldırılarına açık olabilir"),
        ("x-content-type-options", "LOW", "MIME sniffing riski oluşabilir"),
        ("referrer-policy", "LOW", "Referer bilgisi sızabilir"),
        ("permissions-policy", "LOW", "Tarayıcı özellikleri kontrolsüz olabilir") ];

    let mut results = vec![];
    let mut score = 100;

    // # Header Kontrol Döngüsü
    for (name, severity, risk) in headerInfo {

        let isPresent = headers.get(name).is_some();

        // # Score düşürme
        if !isPresent {
            score -= match severity {
                "HIGH" => 20,
                "MEDIUM" => 10,
                _ => 5 } }

        results.push(headerResult {
            name: name.to_string(),
            present: isPresent,
            severity: severity.to_string(),
            risk: if isPresent {
                "No immediate risk".to_string()
            } else {
                risk.to_string() } }) }

    // # Grade Hesaplama
    let grade = match score {
        90..=100 => "A",
        75..=89 => "B",
        50..=74 => "C",
        _ => "F" };

    Ok(report {
        url: url.to_string(),
        grade: grade.to_string(),
        score,
        headers: results }) }

// # Markdown Çıktı
fn printMarkdown(reportData: &report) {

    println!("- ### Guvenlik Raporu ### -\n");
    println!("- Hedef: {}", reportData.url);
    println!("- Derece: {} (Skor: {})\n", reportData.grade, reportData.score);

    for h in &reportData.headers {

        let status = if h.present { "✅" } else { "❌" };

        println!(
            "- {} {} [{}]\n  → {}\n",
            status,
            h.name,
            h.severity,
            h.risk ) } }

// # JSON Kaydetme
fn saveJson(reportData: &report) {

    let dirPath = Path::new("assets/reports");

    if !dirPath.exists() {
        fs::create_dir_all(dirPath).unwrap() }

    let cleanUrl = reportData
        .url
        .replace("https://", "")
        .replace("http://", "")
        .replace("/", "_");

    let filePath = format!("assets/reports/{}.json", cleanUrl);

    let jsonData = serde_json::to_string_pretty(reportData).unwrap();

    fs::write(&filePath, jsonData).unwrap();

    println!("\n[+] JSON report saved: {}", filePath) }

// # Test
#[cfg(test)]
mod tests {

    #[test]
    fn testScoreSystem() {

        let score = 80;

        let grade = match score {
            90..=100 => "A",
            75..=89 => "B",
            50..=74 => "C",
            _ => "F" };

        assert_eq!(grade, "B") } }