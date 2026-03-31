# HTTP Header Analyzer

Bu proje, verilen bir web sitesinin HTTP güvenlik headerlarını analiz eden bir CLI aracıdır.

## Kullanım

```bash
cargo run -- headers https://example.com
````

## Özellikler

* HTTP header analizi
* Markdown çıktı
* JSON rapor oluşturma (varsayılan açık)
* Basit güvenlik skorlama sistemi

## JSON Çıktı

Araç varsayılan olarak analiz sonuçlarını JSON formatında kaydeder:

```bash
cargo run -- headers https://example.com
```

JSON çıktıyı kapatmak için:

```bash
cargo run -- headers https://example.com --json deny
```

JSON dosyaları:

```id="4n4smr"
assets/reports/
```

klasörüne kaydedilir.

## Örnek Çıktı

```id="ncz6pn"
# Security Report

Target: https://frudotz.com
Grade: F

- strict-transport-security: ❌ Missing
- content-security-policy: ❌ Missing
- x-frame-options: ❌ Missing
- x-content-type-options: ❌ Missing
- referrer-policy: ❌ Missing
- permissions-policy: ❌ Missing
```

## Açıklama

Analiz sonucuna göre hedef web sitesinde temel güvenlik headerlarının eksik olduğu tespit edilmiştir. Bu durum, sistemin çeşitli web saldırılarına karşı daha savunmasız olabileceğini göstermektedir.

## Version

Current version: v0.2.0