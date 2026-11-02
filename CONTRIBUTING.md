# CREW Client — Katkıda Bulunma Rehberi

CREW Client projesine katkıda bulunmak istediğiniz için teşekkür ederiz! CREW Client, **CREW Network** tarafından Minecraft Java Edition 1.21.11 ve Fabric ekosistemi için geliştirilen açık kaynaklı, yüksek performanslı ve %100 Türkçe bir Minecraft istemcisidir.

Bu belge, projeye katkı sağlarken izlenmesi gereken temel prensipleri, kod standartlarını ve çekme isteği (pull request) süreçlerini özetler.

---

## 1. Temel Prensipler

1. **Önce Parite ve Kararlılık:** İstemcinin tüm oyun içi davranışları, blok mantıkları, render çıktısı ve ağ protokolü resmi Minecraft 1.21.11 standartlarıyla birebir uyumlu olmalıdır.
2. **Saf Rust ve Performans:** Başlatıcı ve motor bileşenleri Rust diliyle yazılmıştır. Sıfır çöp toplayıcı (zero-GC), düşük gecikmeli çizim ve verimli bellek yönetimi esastır.
3. **%100 Türkçe Kullanıcı Deneyimi:** Kullanıcıya dönük tüm metinler, hata bildirimleri, diyaloglar ve dokümantasyon doğal ve akıcı bir Türkçe ile yazılmalıdır.
4. **Şeffaflık ve Doğruluk:** Asla sahte sayaçlar, yanıltıcı kullanıcı sayıları veya kanıtsız performans iddiaları eklenemez.

---

## 2. Geliştirme Ortamı ve Derleme

Projeyi yerel ortamınızda çalıştırmak için:

### Gereksinimler
- **İşletim Sistemi:** Windows 10 / 11 x64
- **Rust:** 1.85+ (Önerilen: 1.90+ kararlı sürüm)
- **Python:** 3.10+ (Test araçları ve parite betikleri için)
- **Java:** Java 21 LTS (Fabric JVM çalıştırma ve doğrulama için)
- **Git**

### Depoyu Klonlama ve Çalıştırma

```powershell
# Depoyu klonlayın
git clone https://github.com/CodingM-eng/CREW-Client.git
cd CREW-Client

# Testleri çalıştırın
cargo test --manifest-path launcher/Cargo.toml --bin crew-client
cargo test -p horizon-telemetry

# Geliştirme modunda başlatıcıyı açın
cd launcher
cargo run --bin crew-client
```

---

## 3. Kod ve Lisans Kuralları

CREW Client, **MIT Lisansı** altında dağıtılmaktadır. Kod tabanına yalnızca lisansı MIT ile uyumlu açık kaynak kodlar dahil edilebilir.

### İzin Verilen Lisanslar
- MIT / MIT-0
- BSD-2-Clause / BSD-3-Clause
- Apache-2.0
- ISC, zlib, 0BSD, Unlicense, CC0-1.0

### Kesinlikle Yasak Olan Kaynaklar
- GPL / LGPL / AGPL lisanslı kaynak kodlar
- Polyform veya özel kaynak kısıtlamalı lisanslar
- Decompile edilmiş mülkiyet altındaki Mojang Java kodları doğrudan kopyalanamaz (yalnızca davranış referansı olarak incelenebilir)

Başka bir projeden adapte edilen veya kütüphane olarak kullanılan bileşenler, dosya başında telif bildirimini korumalı ve [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md) dosyasına işlenmelidir.

---

## 4. Çekme İstekleri (Pull Request) ve Süreç

Bir değişiklik yapmadan önce büyük mimari değişiklikler veya yeni özellikler için lütfen önce bir [GitHub Tartışması](https://github.com/CodingM-eng/CREW-Client/discussions) veya Issue açın.

### PR Göndermeden Önce Kontrol Listesi
1. **Biçimlendirme ve Kod Analizi:**
   ```powershell
   cargo fmt --check
   cargo clippy --manifest-path launcher/Cargo.toml --all-targets
   ```
2. **Otomasyon Testleri:**
   ```powershell
   cargo test --manifest-path launcher/Cargo.toml --lib
   cargo test --manifest-path launcher/Cargo.toml --bin crew-client
   ```
3. **Yerelleştirme Uyumluluğu:**
   - Yeni bir kullanıcı arayüzü anahtarı eklediyseniz `i18n` testlerinin (`cargo test --lib i18n`) sorunsuz geçtiğinden emin olun.
   - Türkçe metinlerin imla kurallarına uygunluğunu denetleyin.
4. **Ekran Görüntüleri:**
   - Kullanıcı arayüzünü etkileyen her PR'a öncesi/sonrası ekran görüntüleri ekleyin.

---

## 5. İletişim ve Topluluk

- **Resmi Web Sitesi:** [client.crewnetwork.space](https://client.crewnetwork.space/)
- **YouTube:** [@CREWNW](https://youtube.com/@CREWNW)
- **Discord:** [discord.gg/crewnetwork](https://discord.gg/crewnetwork)
- **Hata Bildirimi:** [GitHub Issues](https://github.com/CodingM-eng/CREW-Client/issues)

*Açık kaynak Horizon projesine ve topluluğuna katkıları için teşekkür ederiz.*
