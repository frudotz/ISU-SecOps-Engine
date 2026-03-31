# HTTP Header Analyzer

Bu proje, web sitelerinin HTTP güvenlik headerlarını analiz eden ve sonuçları hem CLI hem de Web GUI üzerinden sunan bir araçtır.

---

## 🚀 Özellikler

- HTTP header analizi
- Severity sistemi (HIGH / MEDIUM / LOW)
- Risk açıklamaları
- Skor ve grade hesaplama
- JSON rapor çıktısı
- Web GUI (tarayıcı üzerinden kullanım)
- REST API desteği

---

## ⚙️ Kullanım

### CLI

```bash
cargo run -- headers https://example.com
````

JSON çıktıyı kapatmak için:

```bash
cargo run -- headers https://example.com --json deny
```

---

### Web Arayüzü

```bash
cargo run -- web
```

Tarayıcıdan aç:

```text
http://127.0.0.1:3000
```

---

## 📊 Sistem Nasıl Çalışır?

1. URL alınır
2. HTTP isteği gönderilir
3. Header'lar analiz edilir
4. Eksik header'lar için risk hesaplanır
5. Severity bazlı skor düşürülür
6. Sonuçlar:

   * Terminal
   * JSON
   * Web GUI

---

## 🧮 Skorlama

| Severity | Puan |
| -------- | ---- |
| HIGH     | -20  |
| MEDIUM   | -10  |
| LOW      | -5   |

---

## 📁 JSON Raporlar

```text
assets/reports/
```

---

## 🧪 Test

```bash
cargo test
```

---

## 🧾 Versiyon

v0.4.0

---

## 📝 Not

Bu araç temel header analizi yapar. Gelişmiş güvenlik testleri içermez.