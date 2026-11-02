# CREW Client — Şeffaf Telemetri, Tanılama ve Veri Gizliliği Politikası

Bu belge, CREW Client ve CREW Network platformlarının veri toplama yaklaşımını, şeffaflık ilkelerini ve gizlilik güvencelerini açıklamaktadır.

---

## 1. Temel Şeffaflık İlkeleri

CREW Client projesinde kullanıcı güveni ve şeffaflık en öncelikli değerdir:

1. **Sahte Sayaçların Kesin Reddi:**
   Ekosistemimizde hiçbir zaman *"Şu anda 990 kişi oynuyor"* veya *"10.000 aktif kullanıcı"* gibi uydurma, yapay veya manipülatif rakamlar yer almaz.
2. **Yalnızca Doğrulanabilir Veriler:**
   - **İndirme İstatistikleri:** Yalnızca resmi GitHub Releases API'si (`CodingM-eng/CREW-Client`) üzerinden çekilen gerçek derleme indirme sayıları yayınlanır.
   - **Sunucu Oyuncu Sayısı:** `oyna.crewnetwork.space` adresinin canlı Minecraft Server Query protokolü aracılığıyla anlık olarak sorgulanır.
   - **Veri Yoksa Dürüst Bildirim:** Bir veri kaynağına erişilemediğinde varsayılan sahte sayılar yerine doğrudan *"Veri şu anda alınamıyor"* veya *"Henüz veri bulunmuyor"* mesajı gösterilir.
3. **Kullanıcı Onayı Olmadan İletim Yoktur (Opt-in):**
   Kullanıcı açık onay vermediği sürece başlatıcı veya oyun motoru dış sunuculara tanılama verisi iletmez.

---

## 2. Kişisel Veri Toplanmaz

Aşağıdaki veriler **asla** toplanmaz, kaydedilmez veya üçüncü taraflarla paylaşılmaz:
- Oyuncu şifreleri, Microsoft oturum belirteçleri veya e-posta adresleri.
- Oyuncunun bilgisayarındaki kullanıcı dizini yolları (`C:\Users\<Kullanıcı Adı>\...`).
- Oyun içi sohbet mesajları, özel mesajlar veya komut geçmişi.
- Dünya haritaları, envanter içerikleri veya sunucu parolaları.
- Oyuncunun gerçek IP adresi (hata raporlarında IP adresleri temizlenir).

---

## 3. İsteğe Bağlı Tanılama Verileri (Onay Verilirse)

Kullanıcı Ayarlar menüsünden onay verdiği takdirde yalnızca performansı ve kararlılığı artırmak amacıyla aşağıdaki anonim teknik veriler işlenebilir:

| Veri Türü | Kapsam | Amaç |
|---|---|---|
| **Çökme Raporu (`crash`)** | Hatanın meydana geldiği Rust iş parçacığı ve yığın izi (kişisel yollardan arındırılmış). | Çökmeye neden olan yazılım hatalarını gidermek. |
| **Performans Özeti (`perf`)** | Ortalama FPS, %1 low kare süreleri, render mesafesi ve tepe RAM kullanımı. | Farklı donanımlarda optimizasyon sağlamak. |
| **Donanım Profili (`system`)** | Ekran kartı modeli (GPU), VRAM boyutu, CPU çekirdek sayısı, işletim sistemi mimarisi. | Sürücü uyumsuzluklarını tespit etmek. |

---

## 4. Telemetriyi Kapatma ve Veri Silme

- **Arayüzden Kapatma:**
  Başlatıcıda **Ayarlar > Veri Toplama** bölümüne giderek dilediğiniz veri anahtarını kapatabilirsiniz. Tüm anahtarlar kapatıldığında hiçbir tanılama paketi gönderilmez.
- **Tamamen Devre Dışı Bırakma:**
  Ortam değişkeni olarak `CREW_TELEMETRY_URL=off` tanımlayarak tüm ağ bağlantılarını kesin olarak engelleyebilirsiniz.
- **Verilerimi Sil:**
  Başlatıcı ayarlarında yer alan *"Verilerimi Sil"* butonuna basıldığında istemcinizin rastgele oluşturulmuş tanımlayıcısı (`install_id`) sıfırlanır ve sunucudaki ilişkili anonim kayıtların temizlenmesi talep edilir.

---

*CREW Network © 2026 — [client.crewnetwork.space](https://client.crewnetwork.space/)*
