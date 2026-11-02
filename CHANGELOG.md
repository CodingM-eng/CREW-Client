# Değişiklik Günlüğü (Changelog) — CREW Client

Bu dosya, CREW Client projesinde yapılan tüm kararlı ve önemli sürüm değişikliklerini içerir.  
Format, [Keep a Changelog](https://keepachangelog.com/tr/1.0.0/) standartlarına ve [SemVer](https://semver.org/lang/tr/) sürümleme kurallarına dayanmaktadır.

---

## [2.1.0] — 2026-11-02

### 🚀 Öne Çıkan Geliştirmeler
- **Uygulama İçi Microsoft Client ID Yapılandırması:** `HORIZON_MSA_CLIENT_ID missing` hatası giderildi. Başlatıcı içerisinden doğrudan Azure Client ID girilebilmesi, `auth.json` ve `msa_client_id.txt` dosyalarına kalıcı kaydedilmesi sağlandı.
- **Sahte İstatistiklerin ve Telemetrinin Kaldırılması:** Eski Horizon telemetri sunucusuna (`telemetry.playhorizon.dev`) yapılan tüm çağrılar ve yanıltıcı sayaçlar kaldırıldı. İstemci ana sayfası doğrudan `oyna.crewnetwork.space` sunucusunun canlı durumuna bağlandı.
- **Demo / Çevrimdışı Profil Desteği:** Microsoft hesabı olmadan arayüzü, yerel dünyaları ve mod yöneticisini test etmek isteyenler için 'Demo / Çevrimdışı İncele' seçeneği eklendi.
- **Doğrulanmış FPS Benchmark Karşılaştırma Sistemi:** Kontrollü laboratuvar testleriyle elde edilmiş doğrulanmış oranlar üzerinden çalışan FPS hesaplayıcı ve ortak veri seti (`benchmarks.json`) entegre edildi.
- **%100 Doğal Türkçe Arayüz:** Başlatıcı menüleri, diyaloglar, hata asistanı ve tüm web sayfaları Türkçe dil yapısına uygun hale getirildi.
- **9 Sayfalı Web Sitesi Altyapısı:** `client.crewnetwork.space` üzerinde Ana Sayfa, Özellikler, Performans, İndir, Sürümler, Sürüm Notları, İstatistikler, Videolar ve Dokümantasyon sayfaları yayınlandı.
- **Güvenli Üst Kaynak (Upstream) Takibi:** `.github/workflows/upstream-sync.yml` iş akışı ile `horizon-rust/horizon` deposundaki değişiklikler otomatik takibe alındı.

### 🔒 Güvenlik ve Doğrulama
- Kurulum Sihirbazı (`CREW-Client-Setup-x64.exe`): `9fe605f3ba8eccb4ebe5468176f88121716ceea5aff83d964ebfabb6b2f25462`
- Taşınabilir Sürüm (`CREW-Client.exe`): `7c666d598d63b96c7b6d93743d583e77a0906159bafb383f92943cd74d8b7d93`

---

## [2.0.0] — 2026-11-01

### ✨ İlk Kararlı Yeniden Markalama
- Horizon açık kaynak kod tabanının CREW Client olarak yapılandırılması.
- Kırmızı ve beyaz kurumsal CREW Network görsel kimliğinin uygulanması.
- Windows x64 mimarisi ve temel Fabric mod desteği hazırlığı.
- `oyna.crewnetwork.space` resmi sunucusunun ön tanımlı sunucu listesine eklenmesi.
- YouTube kanalı (`@CREWNW`) video akışı entegrasyonu.
