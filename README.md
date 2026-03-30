# HTTP Header Analyzer

Bu proje, verilen bir web sitesinin HTTP güvenlik headerlarını analiz eden bir CLI aracıdır.

## Kullanım

```bash
cargo run -- headers https://frudotz.com
````

## Örnek Çıktı

```
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

Analiz sonucuna göre hedef web sitesinde temel güvenlik headerlarının eksik olduğu tespit edilmiştir. Bu durum, uygulamanın çeşitli web tabanlı saldırılara karşı daha savunmasız olabileceğini göstermektedir.

## Özellikler

* HTTP header analizi
* CLI üzerinden kullanım
* Markdown ve JSON çıktı desteği
* Basit güvenlik skorlama sistemi

## Version

Current version: v0.1.0