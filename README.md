# HTTP Header Analyzer (SecOps Engine)

Bu proje, web sitelerinin HTTP güvenlik headerlarını analiz eden, CLI ve Web GUI üzerinden kullanılabilen bir güvenlik analiz aracıdır.

![Rust](https://img.shields.io/badge/Rust-000000?logo=rust&logoColor=white)
![CLI](https://img.shields.io/badge/Tool-CLI-blue)
![Web](https://img.shields.io/badge/Web-GUI-green)
![Security](https://img.shields.io/badge/Security-Analyzer-red)
![Status](https://img.shields.io/badge/Status-Stable-brightgreen)
![CI](https://github.com/frudotz/ISU-SecOps-Engine/actions/workflows/ci.yml/badge.svg)

---

## 🚀 Özellikler

- HTTP header analizi
- Severity sistemi (HIGH / MEDIUM / LOW)
- Risk açıklamaları
- Header value analizi:
  - HSTS (max-age kontrolü)
  - CSP (unsafe-inline kontrolü)
  - X-Frame-Options doğrulama
  - Referrer-Policy kontrolü
- Skor ve grade hesaplama (A–F)
- JSON rapor çıktısı (timestamp destekli)
- Batch scan desteği (.txt ile çoklu hedef)
- Renkli CLI çıktısı
- Web GUI (dashboard)

---

## 🌐 Web GUI Özellikleri

- Sidebar ile rapor yönetimi
- Arama (URL + severity)
- Grade filtreleme
- Otomatik rapor seçimi
- Detaylı analiz ekranı
- Risk summary (HIGH / MEDIUM / LOW)
- Grafik (Chart.js)
- Critical issue gösterimi
- Score bar (görsel skor)
- Responsive tasarım (mobil uyumlu)
- Sticky glass footer

---

## ⚙️ Kullanım

### CLI

```bash
cargo run -- headers https://example.com
````

### Batch Scan

```bash
cargo run -- headers targets.txt
```

### Web GUI

```bash
cargo run -- web
```

Tarayıcı:
[http://127.0.0.1:3000](http://127.0.0.1:3000)

---

## 🧪 Test

```bash
cargo test
```

---

## ⚠️ Error Handling

* Geçersiz URL kontrolü
* Timeout yönetimi
* Web GUI hata mesajları
* Backend Result<T> kullanımı

---

## 📁 Çıktılar

JSON raporlar:

```
assets/reports/
```

---

## 📸 Dashboard Preview

![Dashboard](assets/demo/dashboard.png)

---

## 🧾 Versiyon

v0.5.0

---

## 📌 Not

Bu araç temel HTTP header güvenlik analizleri yapar. Gelişmiş penetration test işlemlerini kapsamaz.
