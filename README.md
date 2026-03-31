# HTTP Header Analyzer

Bu proje, web sitelerinin HTTP güvenlik headerlarını analiz eden, CLI ve Web GUI üzerinden kullanılabilen bir araçtır.

---

## 🚀 Özellikler

- HTTP header analizi
- Severity sistemi (HIGH / MEDIUM / LOW)
- Risk açıklamaları
- Header value analizi (örn: HSTS kontrolü)
- Skor ve grade hesaplama
- JSON rapor çıktısı
- Batch scan desteği (.txt ile çoklu hedef)
- Web GUI (tarayıcı üzerinden kullanım)
- REST API desteği

---

## ⚙️ Kullanım

### CLI

```bash
cargo run -- headers https://example.com
````

Batch scan:

```bash id="b44"
cargo run -- headers targets.txt
```

JSON çıktıyı kapatmak için:

```bash id="c44"
cargo run -- headers https://example.com --json deny
```

---

### Web Arayüzü

```bash id="d44"
cargo run -- web
```

Tarayıcı:

```
http://127.0.0.1:3000
```

---

## 🌐 Web GUI Özellikleri

* Rapor listeleme
* Arama (search)
* Grade filtreleme
* Detaylı analiz görüntüleme
* Risk summary paneli

---

## 📊 Sistem Nasıl Çalışır?

1. URL alınır
2. HTTP isteği gönderilir
3. Header'lar analiz edilir
4. Severity ve risk belirlenir
5. Skor hesaplanır
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

```
assets/reports/
```

---

## 🧪 Test

```bash
cargo test
```

---

## 🧾 Versiyon

v0.4.4

---

## 📝 Not

Bu araç temel header analizi yapar. Gelişmiş pentest işlemleri içermez.