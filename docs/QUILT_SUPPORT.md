# Quilt Mod Yükleyici Desteği — CREW Client

CREW Client, Quilt ekosistemi için tasarlanmış bağımsız `crates/crew-loader-quilt` motoru ile tam uyumlu Quilt desteği sağlar.

---

## 1. Mimari Genel Bakış

- **Resmi Meta Servisi:** `https://meta.quiltmc.org/v3`
- **Birincil Maven Deposu:** `https://maven.quiltmc.org/repository/release/`
- **Yedek Depolar:** Fabric Maven ve Maven Central.
- **İstemci Başlatıcı Ana Sınıfı:** `org.quiltmc.loader.impl.launch.knot.KnotClient`

---

## 2. Uyumluluk Kuralları

- **Minimum Minecraft Sürümü:** Quilt resmi olarak Minecraft **1.18.2** ve daha yeni sürümleri destekler.
- 1.18.2 öncesi sürümler (örneğin 1.16.5, 1.17.1, 1.18.1) Quilt tarafından desteklenmez.
- CREW motoru `validate_quilt_compatibility` ile bu durumu denetleyerek hatalı kurulum girişimlerini baştan engeller.

---

## 3. Sürüm Keşfi ve Beta Ayrıştırması

- Quilt meta uç noktası sürümleri sırasız veya ters sırada döndürebilir.
- Quilt meta verisi doğrudan `stable: true` alanı taşımayabilir. CREW motoru sürüm dizesindeki `-` ayracını (`0.20.0-beta.9`) analiz ederek sürümün kararlı olup olmadığını otomatik olarak tespit eder.
- Motor, tüm Quilt sürümlerini SemVer ve ön sürüm ağırlıklarına göre yeniden sıralar ve en güncel kararlı sürümü kullanıcıya önerir.

---

## 4. Kurulum Süreci

- Profil JSON'u `https://meta.quiltmc.org/v3/versions/loader/{minecraft}/{loader_version}/profile/json` adresinden çekilir.
- Quilt ve Fabric bağımlılıkları çözümlenir, SHA-1 bütünlüğü doğrulanır.
- Profil `quilt-loader-{loader_version}-{minecraft_version}` kimliğiyle yerel sürümler dizinine kaydedilir.
