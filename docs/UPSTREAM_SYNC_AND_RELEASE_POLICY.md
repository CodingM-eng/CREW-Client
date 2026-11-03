# CREW Client — Üst Kaynak (Upstream) Horizon Senkronizasyon ve Sürüm Politikası

Bu belge, CREW Client'ın temelini oluşturan upstream **Horizon Engine** (`horizon-rust/horizon`) güncellemelerinin, özellikle **Horizon Nightly** ve kararlı sürümlerinin nasıl değerlendirileceğini, güvenlik/marka denetimlerinden geçirileceğini ve güvenle yayınlanacağını tanımlar.

---

## 1. Asgari Desteklenen Sürüm Politikası (Minimum Supported Version Policy)

- **Temel Referans Sürümü:** `v2026.10.13` (İlk kararlı üretim `wgpu` Vulkan/DirectX 12 motoru ve Minecraft Protocol 777 desteği).
- **Yeni Kurulum ve Güncelleme Kuralı:**
  - `v2026.10.13` sürümünden daha eski hiçbir Horizon motor sürümü yeni kurulum veya güncelleme olarak sunulmaz (`backend::is_supported_version`).
  - Eski sürümler otomatik indirme veya güncelleme listelerinde (`installable_updates`) yer almaz.
- **Mevcut Kurulu Örneklerin (Instances) Korunması:**
  - Kullanıcının diskinde bulunan daha eski sürümlü örnekler **kesinlikle silinmez veya bozulmaz**.
  - Arayüzde bu sürümler belirgin bir şekilde **"Eski Sürüm (Desteklenmiyor) · {tag}"** ve **"Desteklenmeyen Eski Sürüm (Yükseltme Önerilir)"** uyarı rozetiyle gösterilir, kullanıcı isterse çalıştırmaya devam edebilir.

---

## 2. Üç Katmanlı Sürüm Mimarisi

CREW Client projesinde üç farklı sürüm katmanı açıkça birbirinden ayrılmıştır:

1. **CREW Client Başlatıcı Sürümü (`crew-client-launcher`):**
   - Örn: `2.1.0` (SemVer).
   - Başlatıcı arayüzünü, GPUI açılış animasyonunu, hesap yönetimini, sunucu durum ping'lerini ve Java/Rust çalışma motoru köprüsünü temsil eder.
2. **Horizon Motor Sürümü (`horizon-client` / Upstream Tag):**
   - Örn: `v2026.10.13`, `v2026.10.21` veya `nightly-2026-10-28-...`
   - Rust tabanlı yerel render motorunu, `wgpu` grafik boru hattını ve Minecraft ağ protokol ayrıştırmasını temsil eder.
3. **Minecraft Oyun Uyumluluğu:**
   - Örn: `1.21.11` (Ağ Protokolü 777 / Mojang 26.3).
   - Java Edition ve yerel Rust motorunun hedeflediği resmi Minecraft sürümüdür.

---

## 3. Altı Adımlı Üst Kaynak (Nightly) Değerlendirme Süreci

Bir upstream Horizon sürümü veya Nightly derlemesi çıktığında şu 6 adımlı denetim uygulanır:

### Adım 1: Üst Kaynak Değişikliklerinin Taranması
```bash
python scripts/evaluate_upstream_update.py --check --tag <YENI_ETIKET>
```
Betik, GitHub API veya yerel önbellek üzerinden sürüm notlarını çeker; güvenlik açıkları (`CVE`, `vulnerability`), kırıcı API değişiklikleri (`breaking change`) ve grafik optimizasyonlarını otomatik sınıflandırır.

### Adım 2: Asgari Sürüm ve Protokol Doğrulaması
- Hedef sürümün `v2026.10.13` veya daha yeni bir Nightly olduğu programatik olarak teyit edilir.
- Minecraft protokol numarası (Protocol 777 / 1.21.11) ile uyumluluk kontrol edilir.

### Adım 3: Yasal ve Lisans Uyumluluğu (GPL-3.0)
- Horizon motoru GNU GPL-3.0 lisansına tabidir.
- Upstream yazarlarının telif hakları, lisans metinleri ve `THIRD_PARTY_NOTICES.md` dosyasındaki yasal atıflar **asla silinmez**.
- Kaynak kod paylaşım ve lisans gereklilikleri eksiksiz korunur.

### Adım 4: Marka ve Arayüz Yalıtımı (Branding Audit)
- Kullanıcıya görünen pencerelerde, hata mesajlarında, başlık çubuklarında ve web sitesinde yabancı marka sızıntısı olmaması sağlanır.
- Başlatıcı adı: **CREW Client**
- Resmi Web Sitesi: **https://client.crewnetwork.space/**
- Resmi Sunucu: **oyna.crewnetwork.space** (51.178.92.171:25565)
- Tema Vurgusu: Crimson Kırmızı (`#FF2B4F`) & Gece Siyahı (`#0C0810`).

### Adım 5: Tam Test ve Bütünlük Doğrulaması
Her yeni motor entegrasyonundan sonra aşağıdaki test süiti çalıştırılmalıdır:
```bash
cargo test --bin crew-client
cargo test --lib instances::tests::
cargo test --lib java_launch::tests::
cargo test --lib backend::tests::
cargo check -p horizon-client
```
Tüm testler yeşil olmadan hiçbir sürüm paketlenemez.

### Adım 6: İmzalı İkili ve Manifest Yayınlama (Maintainer Onayı)
- İkili dosyalar (`CREW-Client-Setup-x64.exe` ve `CREW-Client.exe`) derlenir ve SHA-256 özetleri çıkarılır.
- Yayınlama işlemi **kesinlikle otomatik yapılmaz**; yetkili yöneticinin (maintainer) açık `--publish` onayı gereklidir:
```bash
python scripts/evaluate_upstream_update.py --tag <ETIKET> --publish
```
- Bu komut `website/api/releases/latest.json` dosyasını günceller.
- Canlı sunucuya (`client.crewnetwork.space`) dağıtım bağımsız ve güvenli bir işlem olarak maintainer tarafından gerçekleştirilir.

---

## 4. Kullanıcı Verilerinin Güvenliği

- Güncelleme işlemleri sırasında kullanıcının `instances/`, `worlds/`, `screenshots/`, `servers.dat` ve ayar dosyaları asla sıfırlanmaz veya silinmez.
- Eksik veya hatalı indirilen arşivler `.partial` dizini altında toplanır, SHA-256 doğrulaması geçmeden asla kurulu sürümlerin üzerine yazılmaz.
