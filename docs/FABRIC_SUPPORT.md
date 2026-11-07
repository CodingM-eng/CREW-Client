# Fabric Mod Yükleyici Desteği — CREW Client

CREW Client, Fabric Mod Yükleyicisi için optimize edilmiş yerel Rust motoru entegrasyonu sunar.

---

## 1. Mimari Genel Bakış

Fabric entegrasyonu, `crates/crew-loader-fabric` sandığı üzerinden yürütülür:
- **Resmi Meta Servisi:** `https://meta.fabricmc.net/v2`
- **Birincil Maven Deposu:** `https://maven.fabricmc.net/`
- **İkincil Depolar:** `https://repo1.maven.org/maven2/`

---

## 2. Uyumluluk Kuralları

- **Minimum Minecraft Sürümü:** Fabric resmi olarak Minecraft **1.14** ve daha yeni sürümleri destekler.
- Minecraft 1.8.9, 1.12.2 gibi eski sürümlerde Fabric kurulumu denendiğinde CREW motoru kullanıcıya açıklayıcı bir Türkçe uyumluluk uyarısı gösterir.
- 1.14'ten 1.21.x'e kadar olan tüm kararlı, snapshot ve release candidate sürümleri eksiksiz desteklenir.

---

## 3. Sürüm Keşfi ve Seçim Süreci

1. **Sürüm Listeleme:** `GET https://meta.fabricmc.net/v2/versions/loader/{minecraft}` uç noktasından sürüm dizisi çekilir.
2. **Sıralama:** Sürümler SemVer kurallarına göre büyükten küçüğe sıralanır.
3. **Kararlı Sürüm Önceliği:** `pick_loader_version` fonksiyonu aksi belirtilmedikçe en güncel kararlı (`stable: true`) sürümü seçer; kullanıcı isterse deneysel sürümleri de kullanabilir.
4. **Profil Kimliği:** Standartlaştırılmış `fabric-loader-{loader_version}-{minecraft_version}` kimliği üretilir.

---

## 4. Kurulum ve Profil Sentezi

1. `GET https://meta.fabricmc.net/v2/versions/loader/{minecraft}/{loader_version}/profile/json` üzerinden profil şeması alınır.
2. `inheritsFrom` alanı kontrol edilerek profilin istenen Minecraft sürümüne ait olduğu doğrulanır.
3. `libraries` listesi taranır:
   - Maven koordinatları çözümlenir (`group:artifact:version`).
   - Her kütüphane için SHA-1 sağlama toplamı doğrulaması yapılır.
   - Dosyalar `.minecraft/libraries/` altında hedeflenen dizine güvenle yazılır.
4. Tamamlanan profil dosyası `.minecraft/versions/{profile_id}/{profile_id}.json` olarak saklanır.
