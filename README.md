# HTTP Header Analyzer

Web sitelerinin HTTP güvenlik headerlarını analiz eden, CLI ve Web GUI üzerinden kullanılabilen bir araçtır.

---

## 🚀 Özellikler

- HTTP header analizi
- Severity sistemi (HIGH / MEDIUM / LOW)
- Risk açıklamaları
- Header value analizi (HSTS, CSP, X-Frame, Referrer)
- Skor ve grade hesaplama
- JSON rapor çıktısı
- Batch scan desteği (.txt ile çoklu hedef)
- Renkli terminal çıktısı
- Web GUI (dashboard)
- Grafik (risk dağılımı)
- REST API desteği

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

## 🌐 Web GUI Özellikleri

* Rapor listeleme
* Arama (search)
* Grade filtreleme
* Detaylı analiz görüntüleme
* Risk summary paneli
* Grafik (HIGH / MEDIUM / LOW dağılımı)
* Scan sırasında loading feedback

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

assets/reports/

---

## 🧪 Test

```bash
cargo test
```

---

## 🧾 Versiyon

v0.4.6

---

## 📝 Not

Bu araç temel HTTP header analizi yapar, ileri seviye pentest içermez.
