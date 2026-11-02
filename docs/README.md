# CREW Client Dokümantasyonu

CREW Client ile oynamak için [başlatıcıyı](../launcher/README.md) kullanabilirsiniz. Resmi web sitemiz: [client.crewnetwork.space](https://client.crewnetwork.space/).

Bu dokümantasyon dizini, CREW Client'ın mimarisini, render motorunu, mod yönetimini, sunucu entegrasyonunu ve teknik geliştirme süreçlerini kapsayan ayrıntılı rehberleri içerir.

---

## 1. Mimari ve Derleme

- **[Mimarisi](architecture.md):** Kod tabanının yapısı, `Game` döngüsü, render hattı ve başlatıcı bileşenleri.
- **[Kaynak Koddan Derleme](../README.md#14-kaynak-koddan-derleme-windows-x64):** Rust, derleme gereksinimleri ve optimize sürüm oluşturma adımları.
- **[Native İstemci](native-client.md):** Seçenekler, kontroller, adanmış sunucu ve `crew-client-ctl` komutları.
- **[Tasarım Sistemi](design-system.md):** Koyu tema (Night Palette), tipografi, GPUI bileşenleri ve görsel standartlar.

---

## 2. Kimlik Doğrulama ve Hesaplar

- **[Microsoft Hesabı ve OAuth2](microsoft-account.md):** Device Code yetkilendirme akışı, Azure Portal App Registration yapılandırması, Client ID çözümleme katmanları ve Windows Kimlik Bilgisi Yöneticisi şifrelemesi.
- **[Oyun Sahipliği Doğrulaması (Owners)](owners.md):** Minecraft Java Edition lisans denetimi ve çevrimdışı (Demo) profil mekanizması.

---

## 3. Performans ve Grafikler

- **[Doğrulanmış Benchmark Karşılaştırması](benchmark.md):** Kontrollü laboratuvar donanım testleri, test koşulları, 2.0x–2.29x hızlanma katsayıları ve [Web Hesaplayıcı](https://client.crewnetwork.space/performans/).
- **[Grafik Motoru](graphics.md):** Vulkan ve DirectX 12 render hatları, bellek yönetimi ve kare süresi stabilizasyonu.
- **[Shader Paketleri](shaders.md):** WGSL tabanlı shader desteği ve gölgelendirme mimarisi.

---

## 4. Modlar ve Mod Paketleri

- **[Mod Yönetimi](mods.md):** Modrinth entegrasyonu, Fabric 1.21.11 uyumluluğu, mod yükleme ve güncelleme.
- **[Mod Paketleri (Modpacks)](modpacks.md):** `.mrpack` ve CurseForge mod paketlerinin içe aktarılması.
- **[Kaynak Paketleri (Resource Packs)](resource-packs.md):** 1.21.11 kaynak paketlerinin işlenmesi ve doku yönetimi.

---

## 5. Çok Oyunculu ve Sunucular

- **[CREW Network Sunucusu](smp.md):** `oyna.crewnetwork.space` resmi sunucu entegrasyonu, ekonomi, pvp ve etkinlikler.
- **[Sunucu Yönetimi](server.md):** Yerel adanmış sunucu başlatma, web paneli, yedekleme ve `server.toml` ayarları.
- **[Arkadaşlarla Oynama](play-with-friends.md):** Davet kodları ve yerel ağ paylaşımı.

---

## 6. Güvenlik ve Telemetri

- **[Güvenlik Politikası](security.md):** Parola saklamama prensibi, SHA-256 doğrulama kodları ve zafiyet bildirim süreçleri.
- **[Şeffaf Veri ve İstatistikler](telemetry.md):** Sahte sayaçların bulunmaması ilkesi, GitHub API indirme verileri ve canlı sunucu durumu.

---

*CREW Network © 2026 — client.crewnetwork.space*
