# HTTP Header Analyzer

Bu proje, verilen bir web sitesinin HTTP güvenlik headerlarını analiz eden CLI tabanlı bir araçtır.

Sistem yalnızca header varlığını kontrol etmekle kalmaz, aynı zamanda eksik yapılandırmalar için olası güvenlik risklerini değerlendirir ve bir skor üretir.

---

## 🚀 Özellikler

- HTTP güvenlik header analizi
- Severity (HIGH / MEDIUM / LOW) sistemi
- Risk açıklamaları (örn: XSS, MITM, Clickjacking)
- Ağırlıklı skor ve grade hesaplama
- Markdown terminal çıktısı
- JSON rapor oluşturma (varsayılan açık)

---

## ⚙️ Kullanım

```bash
cargo run -- headers https://example.com
````

JSON çıktıyı kapatmak için:

```bash
cargo run -- headers https://example.com --json deny
```

---

## 📊 Örnek Çıktı

```id="ornekcikti"
# Security Report

Target: https://frudotz.com
Grade: F (Score: 20)

- ❌ content-security-policy [HIGH]
  → XSS saldırılarına açık olabilir

- ❌ strict-transport-security [HIGH]
  → MITM saldırılarına açık olabilir

- ❌ x-frame-options [MEDIUM]
  → Clickjacking saldırılarına açık olabilir
```

---

## 🧠 Nasıl Çalışır?

1. Hedef URL’e HTTP isteği gönderilir
2. Response header’ları alınır
3. Tanımlı güvenlik header listesi ile karşılaştırılır
4. Her header için:

   * Var/yok kontrolü yapılır
   * Severity atanır
   * Eksikse risk açıklaması eklenir
5. Genel skor hesaplanır
6. Sonuçlar terminalde ve JSON olarak sunulur

---

## 📁 JSON Çıktı

Raporlar aşağıdaki klasöre kaydedilir:

```
assets/reports/
```

---

## 🧮 Skorlama Sistemi

Başlangıç puanı: **100**

Eksik header’lara göre düşüş:

| Severity | Puan |
| -------- | ---- |
| HIGH     | -20  |
| MEDIUM   | -10  |
| LOW      | -5   |

### Grade

| Skor   | Not |
| ------ | --- |
| 90-100 | A   |
| 75-89  | B   |
| 50-74  | C   |
| <50    | F   |

---

## 🧪 Test

```bash
cargo test
```

---

## 🧾 Versiyon

**Current version:** v0.3.0

---

## 📝 Not

Bu araç yalnızca header varlığını kontrol eder ve basit risk yorumları üretir. Header içeriklerinin doğruluğu veya tam güvenlik analizi yapılmamaktadır.

---

## 🎯 Amaç

Bu proje, temel web güvenliği prensiplerini anlamak ve Rust ile CLI tabanlı bir analiz aracı geliştirmek amacıyla hazırlanmıştır.