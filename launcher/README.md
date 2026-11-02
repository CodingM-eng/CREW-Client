# CREW Client Başlatıcısı (Launcher)

CREW Client Başlatıcısı, **CREW Network** tarafından geliştirilen; Minecraft Java Edition 1.21.11 ve Fabric mod ekosistemini yüksek performansla çalıştıran modern bir Windows masaüstü uygulamasıdır.

Kullanıcı arayüzü, GPU hızlandırmalı ve son kareden değişmeyen görünümleri anında çizen **GPUI Fast** framework'ü ile sıfırdan inşa edilmiştir.

---

## 1. Başlıca Özellikler

- **Resmi Minecraft İndirme & Hazırlama:** Mojang sunucularından doğrudan ve güvenli indirme (piston-meta, piston-data).
- **Resmi Microsoft OAuth Girişi:** Parolalarınızı asla sormayan, güvenli `microsoft.com/link` Cihaz Kodu (Device Code) akışı.
- **Uygulama İçi Client ID Yapılandırması:** Azure Uygulama Kimliği bulunmadığında uygulama içinden doğrudan girilebilen ve `auth.json` dosyasına güvenle kaydedilen yapılandırma kutusu.
- **Demo / Çevrimdışı İnceleme Modu:** Lisanslı hesabı henüz yapılandırılmamış geliştiriciler ve kullanıcılar için tek tıkla yerel profille açılış.
- **Modrinth Mod Yöneticisi:** Popüler Fabric 1.21.11 modlarını (Sodium, Lithium, Iris vb.) tek tıkla arama, indirme, güncelleme ve silme.
- **Doğrulanmış Performans & FPS Karşılaştırması:** Ana sayfada kontrollü donanım testlerine dayanan 4 farklı donanım profili (2.0x–2.29x) ve [Web Hesaplayıcı](https://client.crewnetwork.space/performans/) köprüsü.
- **Canlı YouTube Akışı:** [@CREWNW](https://youtube.com/@CREWNW) kanalındaki en yeni videoların ana ekranda listelenmesi (Shorts videoları otomatik elenir).
- **Ön Tanımlı Sunucu Entegrasyonu:** `oyna.crewnetwork.space` resmi sunucusunun canlı durumu ve tek tıkla bağlanma imkanı.

---

## 2. Dizin Yapısı ve Depolama

- **Kurulum Dizini:** `%APPDATA%\CREW Client\`
- **Yapılandırma Dosyası:** `%APPDATA%\CREW Client\settings.json`
- **Kimlik Bilgisi:** `%APPDATA%\CREW Client\auth.json` ve `%APPDATA%\CREW Client\msa_client_id.txt`
- **Örnekler & Modlar:** `%APPDATA%\CREW Client\instances\`
- **Loglar & Hata Raporları:** `%APPDATA%\CREW Client\logs\`

---

## 3. Geliştirme ve Derleme

```powershell
# Başlatıcı dizinine geçin
cd launcher

# Birim ve entegrasyon testlerini çalıştırın
cargo test --lib
cargo test --bin crew-client

# Geliştirici modunda başlatın
cargo run --bin crew-client

# Dağıtım için optimize edilmiş Release derlemesi yapın
cargo build --release --bin crew-client
```

Derlenen çalıştırılabilir dosya `launcher/target/release/crew-client.exe` konumunda oluşturulur.

---

*CREW Network © 2026 — client.crewnetwork.space*
