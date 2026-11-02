# CREW Client — Mod Yönetimi ve Modrinth Entegrasyonu

Bu belge, CREW Client başlatıcısında ve oyun motorunda modların nasıl yüklendiğini, yönetildiğini ve Modrinth entegrasyonunun nasıl çalıştığını açıklamaktadır.

---

## 1. Genel Bakış

CREW Client, oyuncuların oyun deneyimini zenginleştirmek için Fabric 1.21.11 modları ve dahili içerik paketleri için kapsamlı destek sunar.

Başlatıcı arayüzünde yer alan **Modlar** sekmesi sayesinde:
- Doğrudan **Modrinth** kütüphanesinden mod arayabilir,
- Tek tıkla modları indirip profilinize ekleyebilir,
- Yüklü modları etkinleştirebilir veya devre dışı bırakabilirsiniz.

---

## 2. Mod Türleri

| Tür | Uzantı | Açıklama | Konum |
|---|---|---|---|
| **Fabric Modları** | `.jar` | Minecraft Java 1.21.11 ve Fabric Loader için geliştirilmiş standart modlar. | `%APPDATA%\CREW Client\mods\` |
| **İçerik Paketleri** | `.hmod` | Rust tabanlı yerel motor için veri ve görsel varlık paketleri (bloklar, tarifler, eşyalar). | `%APPDATA%\CREW Client\horizon-mods\` |
| **Kaynak Paketleri** | `.zip` | Doku paketleri, sesler ve kullanıcı arayüzü özelleştirmeleri. | `%APPDATA%\CREW Client\resourcepacks\` |

---

## 3. Başlatıcı Üzerinden Mod Yükleme (Modrinth)

1. CREW Client başlatıcısını açın.
2. Sol menüden **Modlar (Mods)** sayfasına geçiş yapın.
3. Arama çubuğuna yüklemek istediğiniz modun adını yazın (Örn: *Sodium*, *Lithium*, *Iris*).
4. Mod kartındaki **İndir** butonuna tıklayın. Başlatıcı modun 1.21.11 Fabric uyumlu sürümünü ve bağımlılıklarını otomatik olarak ilgili dizine indirir.
5. Oyunu başlattığınızda yüklenen modlar aktif hale gelir.

---

## 4. Elle (Manuel) Mod Ekleme

Kendi indirdiğiniz `.jar` veya `.hmod` dosyalarını eklemek için:
1. Klavyenizden `Windows + R` tuşlarına basarak **Çalıştır** penceresini açın.
2. `%APPDATA%\CREW Client\mods` yazıp Enter tuşuna basın.
3. İndirdiğiniz mod dosyalarını bu klasöre yapıştırın.
4. Başlatıcıyı yeniden başlattığınızda modlar listede görünecektir.

---

*CREW Network © 2026 — [client.crewnetwork.space](https://client.crewnetwork.space/)*
