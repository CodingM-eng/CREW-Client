# CREW Client — Rust Motoru Mimarisi ve Teknik Dokümantasyonu

CREW Client, yüksek performanslı, güvenli ve modern bir Minecraft başlatıcı ekosistemidir. Bu sistemin kalbinde, yerel hız ve bellek güvenliği sağlayan **CREW Rust Motoru** yer alır.

---

## 1. Rust Motoru vs. Java/JVM Gerçek Çalışma Zamanı Ayrımı

CREW ekosisteminde kullanıcı deneyimi ve mühendislik dürüstlüğü esastır:

- **CREW Rust Motoru (Yerel İstemci & Başlatıcı):**
  - **Arayüz & GPU Render:** GPUI ve wgpu (DirectX 12 / Vulkan / Metal) ile doğrudan ekran kartı üzerinde retained-mode çizim yapar. Webview veya Electron gibi ağır JavaScript çalışma ortamları içermez.
  - **Profil ve Sürüm Yönetimi:** Mojang sürüm JSON'larını, Fabric/Quilt meta ağaçlarını ve bağımlılık profillerini yerel Rust hızında sentezler.
  - **Ağ ve Güvenlik Pipeline'ı:** Çok iş parçacıklı kütüphane indirmeleri, Maven koordinat çözümlemeleri ve SHA-1/SHA-256 sağlama toplamı kontrollerini yürütür. Path traversal (dizin aşma) saldırılarına karşı sanitizasyon uygular.
  - **JVM Süreç Orkestrasyonu:** Seçilen Minecraft sürümü ve mod yükleyicisine uygun Java ortamını (Java 8, 16, 17, 21) belirler, classpath parametrelerini hazırlar ve JVM sürecini izole şekilde başlatır.

- **Minecraft Çalışma Zamanı (Java Edition & Modlar):**
  - Minecraft Java Edition ve modlar (Fabric, Quilt vb.) doğası gereği JVM (Java Virtual Machine) üzerinde yürütülür.
  - CREW Client, modların Java çalışma mantığını taklit etmek yerine; JVM'in en optimize bellek, çöp toplayıcı (ZGC/Shenandoah) ve başlatma bayraklarıyla en kararlı şekilde çalışmasını sağlar.

---

## 2. Modüler Rust Crate Mimarisi

Sistem, birbirinden bağımsız test edilebilir ve genişletilebilir 4 ana modülden oluşur:

```
crates/
├── crew-core/               # Temel Maven, Checksum, Sürüm Sıralama ve HTTP istemcisi
├── crew-loader-fabric/      # Fabric Meta API, sürüm keşfi, uyumluluk ve profil kurulumu
├── crew-loader-quilt/       # Quilt Meta v3, beta ayrıştırma, profil sentezi
└── crew-version-manager/    # Sürüm politikası (>=1.8.9), Java eşleme ve profil kalıtımı
```

### A. `crates/crew-core`
- **Maven Koordinatları:** `group:artifact:version[:classifier][@extension]` sözdizimini tam olarak çözümler ve güvenli göreceli dosya yollarına (`rel_path`) dönüştürür.
- **Sağlama Toplamı (Checksum):** Dosyaların bütünlüğünü doğrulamak için SHA-1 ve SHA-256 motoru sunar.
- **Sürüm Sıralayıcı (`version_order`):** SemVer, ön sürüm (`-beta`, `-rc`, `-pre`) ve sayısal parça karşılaştırmasını kusursuz şekilde yürütür.
- **Hata Modeli (`CrewError`):** Ağ, G/Ç, doğrulama ve JSON hatalarını Türkçe hata mesajlarıyla standartlaştırır.

### B. `crates/crew-loader-fabric`
- Fabric Meta v2 API (`https://meta.fabricmc.net/v2`) ile entegredir.
- Minecraft 1.14 ve üzerindeki sürümleri doğrular.
- Kararlı ve deneysel loader sürümlerini filtreleyip en güncel olanı otomatik seçer.
- İndirilen kütüphaneleri `libraries/` dizinine, profil JSON'unu `versions/` altına yerleştirir.

### C. `crates/crew-loader-quilt`
- Quilt Meta v3 API (`https://meta.quiltmc.org/v3`) ile doğrudan haberleşir.
- Minecraft 1.18.2 ve üzerindeki sürümler için uyumluluk sağlar.
- Sürüm etiketlerindeki `-` veya beta belirteçlerini ayrıştırarak kararlı sürümleri önceliklendirir.
- Maven depolarından (`https://maven.quiltmc.org/repository/release/`) kütüphaneleri SHA-1 doğrulayarak indirir.

### D. `crates/crew-version-manager`
- **Minimum Sürüm Politikası:** Minimum Minecraft 1.8.9 sürümünü denetler.
- **Java Eşleme Matrisi:** Sürüme göre Java 8, 16, 17 veya 21 gereksinimini dinamik olarak raporlar.
- **Profil Kalıtım Motoru (`merge_version_profiles`):** Alt profillerin (Fabric/Quilt) ana vanilla sürümle birleştirilmesini, çakışan kütüphanelerin tekilleştirilmesini ve JVM/oyun argümanlarının bir araya getirilmesini sağlar.

---

## 3. Güvenlik ve Bütünlük Denetimi

- **Dizin Aşma Koruması:** `..`, kök yollar veya sistem sürücü harfleri içeren kütüphane yolları `sanitize_relative_path` fonksiyonu tarafından engellenir.
- **Sağlama Toplamı Doğrulama:** Profilde yer alan veya depodan çekilen `.sha1` sağlama toplamları indirme esnasında eşleşmezse işlem iptal edilir ve bozuk dosyaların oyunu çökertmesi önlenir.
- **Döngüsel Kalıtım Koruması:** `inheritsFrom` zincirinde döngü tespit edilirse profil reddedilir.
