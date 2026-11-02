# CREW Client — Güvenlik Politikası ve Mimarisi

Bu belge, CREW Client'ın kullanıcı güvenliğini sağlamak amacıyla uyguladığı mimari tedbirleri, şifreleme mekanizmalarını ve zafiyet bildirim süreçlerini özetlemektedir.

---

## 1. Temel Güvenlik İlkeleri

CREW Client, kullanıcıların gizliliğini ve hesap güvenliğini korumak için tasarlanmıştır:

1. **Parola Saklamama İlkesi:**
   Kullanıcıların Microsoft veya Minecraft hesap parolaları hiçbir koşulda CREW Client tarafından talep edilmez, kaydedilmez veya sunucularımıza gönderilmez. Giriş işlemleri doğrudan resmi Microsoft portali üzerinden Device Code akışı (RFC 8628) ile gerçekleşir.
2. **İzole Kimlik Bilgisi Depolama:**
   Giriş sonrası elde edilen oturum belirteçleri Windows Kimlik Bilgisi Yöneticisi (`CREW Client Microsoft Token`) kasasında işletim sistemi düzeyinde şifrelenerek korunur.
3. **Komut Satırı Güvenliği (Argv Gizliliği):**
   Minecraft oturum jetonları süreç listesinde (`Task Manager` veya `Process Hacker`) görünmemesi için komut satırı argümanı (`--accessToken`) olarak iletilmez. Geçici ve izinleri kısıtlanmış bir bellek içi oturum mekanizması kullanılır.
4. **Resmi Dağıtım ve SHA-256 Doğrulama:**
   Her CREW Client sürümü resmi GitHub Releases sayfası üzerinden SHA-256 bütünlük kodlarıyla birlikte yayınlanır:
   - Kurulum Dosyası (`CREW-Client-Setup-x64.exe`)
   - Taşınabilir Sürüm (`CREW-Client.exe`)

---

## 2. Desteklenen Sürümler

| Sürüm | Durum | Güvenlik Güncellemeleri |
|---|:---:|:---:|
| **v2.1.x (Güncel)** | **Destekleniyor** | **Aktif** |
| v2.0.x | Eski | Sürüm güncellemesi önerilir |
| v1.x / Horizon Mirası | Desteklenmiyor | Sonlandırıldı |

---

## 3. Güvenlik Açığı Bildirimi

Eğer CREW Client içerisinde bir güvenlik zafiyeti veya veri sızıntısı tespit ederseniz:

- **Lütfen GitHub Issues üzerinde herkese açık paylaşım yapmayınız.**
- Güvenlik raporunuzu doğrudan **[iletisim@crewnetwork.space](mailto:iletisim@crewnetwork.space)** adresine iletiniz.
- E-postanızda şu detayları belirtiniz:
  - Zafiyetin türü ve etkilenen bileşen (Başlatıcı, Oyun Motoru, Ağ Protokolü).
  - Sorunu yeniden oluşturmak için gerekli adımlar (PoC).
  - Güvenlik ekibimiz raporu inceleyecek ve 48 saat içerisinde geri bildirim sağlayacaktır.

---

*CREW Network © 2026 — [client.crewnetwork.space](https://client.crewnetwork.space/)*
