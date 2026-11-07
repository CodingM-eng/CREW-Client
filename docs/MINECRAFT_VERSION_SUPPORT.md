# Minecraft ve Java Sürüm Uyumluluk Matrisi — CREW Client

CREW Client ekosistemi, Minecraft'ın efsanevi PvP sürümlerinden en son çıkan modern güncellemelerine kadar geniş bir sürüm yelpazesini resmi olarak destekler.

---

## 1. Sürüm Destek Politikası

- **Minimum Desteklenen Sürüm:** Minecraft **1.8.9**
- **Maksimum Desteklenen Sürüm:** Minecraft **1.21.x** (ve güncel snapshot/pre-release sürümleri)
- Minecraft 1.8.9 altındaki sürümler (ör. 1.7.10) modern işletim sistemleri, güncel ekran kartı sürücüleri ve güvenlik standartları nedeniyle CREW Client tarafından resmi destek kapsamında tutulmamaktadır.

---

## 2. Java Çalışma Zamanı (JVM) Eşleme Tablosu

Minecraft'ın farklı çağları, farklı Java mimarilerini zorunlu kılar. CREW Client bu eşlemeyi `crew-version-manager` üzerinden otomatik olarak belirler:

| Minecraft Sürüm Aralığı | Önerilen Java Sürümü | Açıklama |
| :--- | :--- | :--- |
| **1.8.9 — 1.16.5** | **Java 8 (LTS)** | Eski Mojang launch wrapper ve LWJGL 2 / erken LWJGL 3 sürümleri için kararlı ortam. |
| **1.17 — 1.17.1** | **Java 16** | Mojang'ın Java 16 geçiş dönemi. Record sınıfları ve yeni dil özellikleri. |
| **1.18 — 1.20.4** | **Java 17 (LTS)** | Yeni dünya oluşturma motoru (Caves & Cliffs) ve modern Fabric/Quilt altyapısı. |
| **1.20.5 — 1.21.x+** | **Java 21 (LTS)** | Modern sanal iş parçacıkları (Virtual Threads), gelişmiş ZGC ve en yüksek performans. |

---

## 3. Mod Yükleyici Uyumluluk Matrisi

| Minecraft Sürümü | Vanilla | Fabric | Quilt |
| :--- | :---: | :---: | :---: |
| **1.8.9** | Desteklenir | Desteklenmez | Desteklenmez |
| **1.12.2** | Desteklenir | Desteklenmez | Desteklenmez |
| **1.14 — 1.16.5** | Desteklenir | Desteklenir (>= 1.14) | Desteklenmez |
| **1.17 — 1.18.1** | Desteklenir | Desteklenir | Desteklenmez |
| **1.18.2 — 1.20.4** | Desteklenir | Desteklenir | Desteklenir (>= 1.18.2) |
| **1.20.5 — 1.21.x** | Desteklenir | Desteklenir | Desteklenir |
