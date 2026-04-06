# HTTP Header Analyzer (SecOps Engine)

Modern web uygulamalarının güvenlik seviyesini hızlı ve anlaşılır şekilde analiz etmek için geliştirilmiş, CLI ve Web GUI destekli bir HTTP header analiz aracıdır.

Bu proje, gerçek dünya kullanım senaryolarını hedefleyen, performanslı ve modüler bir SecOps çözümü olarak tasarlanmıştır.

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![CLI](https://img.shields.io/badge/Tool-CLI-blue)
![Web](https://img.shields.io/badge/Web-GUI-green)
![Security](https://img.shields.io/badge/Security-Analyzer-red)
![Status](https://img.shields.io/badge/Status-Stable-brightgreen)
![CI](https://github.com/frudotz/ISU-SecOps-Engine/actions/workflows/ci.yml/badge.svg)
![Docker](https://img.shields.io/badge/Docker-Supported-2496ED?logo=docker&logoColor=white)

---

| ![ISU](https://www.istinye.edu.tr/sites/default/files/2025-07/isu_logo_tr-1.svg) | Sızma Testi Proje Ödevi |
|---|---|
| **Öğrenci Adı** | Hamza Arda Karabacak |
| **Öğrenci No.** | 2520191010 |
| **Öğretim Gör. (Danışman)** | Keyvan Arasteh Abbasabad |
| **Ders Kodu & Adı** | BGT006 Sızma Testi |

---

## 📚 İçindekiler

- [🚀 Özellikler](#-özellikler)
- [🌐 Web GUI](#-web-gui)
- [🧠 Mimari Yaklaşım](#-mimari-yaklaşım)
- [⚙️ Kullanım](#️-kullanım)
  - [CLI](#cli)
  - [Batch Scan](#batch-scan)
  - [Web GUI](#web-gui)
  - [Docker](#docker)
- [🧪 Testing](#-testing)
- [🔒 Code Quality & Dev Workflow](#-code-quality--dev-workflow)
- [⚠️ Error Handling](#️-error-handling)
- [📁 Çıktılar](#-çıktılar)
- [📸 Dashboard Preview](#-dashboard-preview)
- [⚠️ Limitations](#️-limitations)
- [🧾 Versiyon](#-versiyon)
- [📌 Not](#-not)

---

## 🚀 Özellikler

* HTTP security header analizi
* Severity sistemi (HIGH / MEDIUM / LOW)
* Risk açıklamaları ve güvenlik yorumları
* Header value analizi:

  * HSTS (max-age kontrolü)
  * CSP (unsafe-inline kontrolü)
  * X-Frame-Options doğrulama
  * Referrer-Policy kontrolü
* Skor ve grade hesaplama (A–F)
* JSON rapor çıktısı (timestamp destekli)
* Batch scan desteği (.txt ile çoklu hedef)
* Renkli CLI çıktısı
* Web GUI (dashboard)

---

## 🌐 Web GUI

Kullanıcı dostu dashboard ile analiz sonuçlarını görselleştirir:

* Sidebar ile rapor yönetimi
* Arama (URL bazlı)
* Otomatik rapor seçimi
* Detaylı header analizi
* Risk dağılım grafiği (Chart.js)
* Critical issue gösterimi
* Görsel skor barı
* Responsive tasarım (mobil uyumlu)

---

## 🧠 Mimari Yaklaşım

Proje 3 ana katmandan oluşur:

* **CLI Layer** → kullanıcı etkileşimi ve batch işlemler
* **Analyzer Engine** → HTTP istekleri ve güvenlik analizi
* **Web Layer (Axum)** → REST API + GUI entegrasyonu

Bu yapı sayesinde sistem modüler, genişletilebilir ve sürdürülebilir hale getirilmiştir.

---

## ⚙️ Kullanım

### CLI

```bash
cargo run -- headers https://example.com
```

### Batch Scan

```bash
cargo run -- headers targets.txt
```

### Web GUI

```bash
cargo run -- web
```

Tarayıcı:
http://127.0.0.1:3000

### Docker

Projeyi sisteminize Rust kurmadan doğrudan Docker üzerinden çalıştırabilirsiniz:

**İmajı Derlemek İçin:**
```bash
docker build -t pentester .
```

**CLI aracı olarak çalıştırmak için:**
```bash
docker run --rm pentester headers --json allow example.com
```

**Web arayüzünü (GUI) başlatmak için:**
```bash
docker run -d -p 8080:8080 --name pentester-web pentester web
```

---

## 🧪 Testing

```bash
cargo test
```

Ek olarak proje, gerçek dünya senaryolarını simüle eden async testler içerir.

---

## 🔒 Code Quality & Dev Workflow

* `cargo fmt` ile standart formatlama
* `cargo clippy` (strict mode) ile lint kontrolü
* Multi-platform CI pipeline (Ubuntu, Windows, Kali)
* `Justfile` ile standart geliştirme komutları:

  * `just build`
  * `just test`
  * `just lint`
  * `just ci`

---

## ⚠️ Error Handling

* Geçersiz URL kontrolü
* Timeout yönetimi
* Backend tarafında `Result<T>` kullanımı
* Web GUI kullanıcı geri bildirimleri

---

## 📁 Çıktılar

JSON raporlar:

```
assets/reports/
```

---

## 🎬 Demo

### Web Arayüzü (Web GUI)

<img src="assets/demo/web-demo.webp" width="800" alt="Web GUI Demo Video">

### Uçbirim Arayüzü (CLI/Terminal)

<img src="assets/demo/project-demo.webp" width="800" alt="CLI/Terminal Demo Video">

---

## ⚠️ Limitations

* Sadece HTTP header analizi yapar
* Aktif exploitation veya penetration test içermez
* Hedef sistem erişilebilir olmalıdır

---

## 🧾 Versiyon

v0.5.2

---

## 📌 Not

Bu proje, SecOps ve güvenlik analiz araçlarının temel prensiplerini göstermek amacıyla geliştirilmiştir. Gerçek sistemlerde daha kapsamlı güvenlik testleri önerilir.
