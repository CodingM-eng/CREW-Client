# CREW Client — Mimari, Kurulum ve Rebrand Raporu

**Proje:** CREW Client  
**Temel Kod Tabanı:** Horizon (Rust tabanlı açık kaynak Minecraft istemcisi & başlatıcısı)  
**Hedef Platform:** Windows 10/11 x64  
**Hedef Sürümler:** Minecraft Java Edition 1.21.11 & 26.3  
**Arayüz Dili:** Türkçe  
**Varsayılan Ağ Sunucusu:** CREW Network (`oyna.crewnetwork.space`)  

---

## 1. Proje Özeti ve Yapılan Rebrand İşlemleri

Horizon kod tabanı incelenmiş, orijinal mimarisine ve açık kaynak lisanslarına zarar verilmeden **CREW Client** kimliğiyle temiz bir şekilde yeniden yapılandırılmıştır:

1. **Ön Koruma & Yedekleme:**
   - Değişiklik öncesi tüm proje `release/horizon-main-backup` dizinine eksiksiz yedeklenmiştir.

2. **Görsel & Metinsel Kimlik (Brand Identity):**
   - **Pencere Başlığı:** İşletim sistemi ve özel başlık çubuğunda `CREW Client` başlığı uygulandı (`launcher/src/titlebar.rs`).
   - **Kenar Çubuğu (Sidebar):** Üst marka başlığı `CREW Client` olarak güncellendi (`launcher/src/main.rs`).
   - **Ana Sayfa (Hero):** Örnek seçilmediğinde varsayılan istemci adı `CREW Client` ve sürüm etiketi olarak yapılandırıldı (`launcher/src/home.rs`).
   - **Açılış Animasyonu:** Başlatıcı açılışındaki harf animasyonu `"CREW CLIENT"` olarak ayarlandı (`launcher/src/main.rs`).
   - **Sistem Menüsü:** Masaüstü ve işletim sistemi uygulama menülerinde `CREW Client` başlığı ve `Quit CREW Client` aksiyonu tanımlandı.
   - **Uygulama Tanımlayıcısı:** Windows / Wayland uygulama kimliği `crew-client` olarak güncellendi.

3. **Paket ve Derleme Yapılandırması:**
   - `launcher/Cargo.toml` üzerinde ikili çalıştırılabilir adı `crew-client`, kütüphane adı `horizon_launcher` olarak ayarlandı.
   - Gereksiz ve eski deneysel kalıntılar temizlendi.

4. **Türkçe Dil Desteği:**
   - `launcher/lang/tr.json` dosyasında yer alan kullanıcıya dönük tüm metinler güncellendi; "Horizon" ifadeleri kullanıcı deneyiminde "CREW Client" olarak uyarlandı.

5. **CREW Network Sunucu Entegrasyonu:**
   - `launcher/src/server_list.rs` içinde varsayılan sunucu `CREW Network` olarak tanımlandı.
   - `launcher/links.json` içinde resmi sunucu adresi `oyna.crewnetwork.space` olarak yapılandırıldı.

---

## 2. Mimari ve Fabric Uyumluluk Analizi

### İki Çekirdekli Çalışma Mimarisi:

1. **Modded Java Edition (Fabric / Quilt):**
   - Başlatıcı içinde yer alan `loaders.rs` ve `java_launch.rs` modülleri resmi Fabric Meta API'sine (`https://meta.fabricmc.net/v2`) bağlanır.
   - Minecraft 1.21.11 ve Fabric Loader 0.20.1+ sürümlerini doğrudan indirip profil birleştirmesi (`inheritsFrom`) yaparak JVM (Java 21) üzerinde başlatır.
   - Sodium, FerriteCore, Lithium, ImmediatelyFast gibi Java Fabric modları bu modda tam performansla çalışır.

2. **Yerel Rust Motoru (Horizon Native Client):**
   - Rust ile yazılmış Vulkan ve DirectX 12 render motoruna sahip bağımsız istemci.
   - Java sanal makinesi kullanmadığı için Java bytecode modlarını (.jar) doğrudan çalıştırmaz; kendine ait hafif içerik modlarını ve yerel modları destekler.

---

## 3. Derleme ve Çalıştırma

Projeyi derlemek için:

```powershell
# Geliştirici kontrolü (Cargo check)
& "C:\Users\efeka\.cargo\bin\cargo.exe" check --manifest-path "release/horizon-main/launcher/Cargo.toml"

# Testlerin çalıştırılması
& "C:\Users\efeka\.cargo\bin\cargo.exe" test --manifest-path "release/horizon-main/launcher/Cargo.toml" --lib

# Kararlı sürüm (Release) derlemesi
& "C:\Users\efeka\.cargo\bin\cargo.exe" build --release --manifest-path "release/horizon-main/launcher/Cargo.toml"
```

Derleme çıktısı:
`release/horizon-main/target/release/crew-client.exe` (Ayrıca kök dizinde: `release/horizon-main/CREW-Client.exe`)

---

## 4. Upstream Senkronizasyonu & Web Sitesi

### Upstream Takip Aracı (`scripts/upstream_sync.py`):
```powershell
python release/horizon-main/scripts/upstream_sync.py
```
`upstream.json` yapılandırmasını kullanarak `horizon-rust/horizon` deposundaki yeni değişiklikleri sorgular, Türkçe değişiklik raporu üretir ve CREW'e özgü yerelleştirilmiş dosyaları korur.

### Resmi Web Sitesi (`website/`):
Cloudflare Pages için hazırlanmış, `client.crewnetwork.space` etki alanına dağıtıma hazır statik web platformudur:
- `website/index.html`: Ana vitrin sayfası
- `website/indir/index.html`: İndirme ve SHA-256 doğrulama sayfası
- `website/dokumanlar/index.html`: 7 bölümlük kapsamlı dokümantasyon
- `website/surumler/index.html`: Sürüm notları ve değişiklik günlüğü
- `website/_headers` & `website/wrangler.toml`: Cloudflare Pages güvenlik ve dağıtım ayarları
