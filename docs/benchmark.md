# CREW Client — Doğrulanmış Benchmark Metodolojisi ve Performans Raporu

Bu belge, CREW Client ile standart Minecraft Java Edition arasındaki performans farkını ölçen test araçlarını, kontrol parametrelerini ve laboratuvar donanım sonuçlarını açıklamaktadır.

---

## 1. Test Metodolojisi

Performans karşılaştırmaları `tools/perf_bench.py` otomasyon aracı ile yürütülür. Aynı bilgisayarda, aynı dünya tohumunda (seed), aynı çözünürlükte ve aynı kamera rotalarında testler gerçekleştirilir:

1. **Başlatma Süresi:** Süreç başlangıcından ana menünün ilk karesine kadar geçen süre (ms).
2. **Dünya Yükleme Süresi:** Yaratıcı modda oluşturulan dünyada oyuncunun doğması ve görüş alanındaki tüm parçaların çizilmesine kadar geçen süre.
3. **4 Standart Test Sahnesi:**
   - **Köy:** Karmaşık blok modelleri, yapılar ve yapay zeka varlıkları.
   - **Orman:** Yüksek yaprak yoğunluğu, şeffaflık hesaplamaları ve pürüzsüz aydınlatma.
   - **Okyanus:** Su yüzeyi kırılmaları ve geniş görüş alanı.
   - **Dağ:** Geniş dikey yükseklik değişimleri ve derin chunk hesaplamaları.
4. **Kare Süreleri (Frame Times):**
   - **Ortalama FPS:** Sahne boyunca çizilen toplam kare sayısı / süre.
   - **%1 Lows:** En yavaş %1'lik karelerin ortalaması (anlık takılmaları ölçer).
   - **%0.1 Lows:** En yavaş %0.1'lik karelerin ortalaması (en şiddetli mikrosaniye donmaları).
5. **Bellek Tüketimi (RAM & VRAM):**
   - Çalışma sırasındaki tepe bellek (Peak RSS) ve sabit durum (Steady State) bellek miktarı.

---

## 2. Doğrulanmış Laboratuvar Sonuçları

Aşağıdaki veriler kontrollü laboratuvar koşullarında elde edilmiştir:
- **Donanım:** AMD Ryzen 7 9800X3D, NVIDIA GeForce RTX 5080, 32 GB DDR5 RAM, NVMe SSD
- **İşletim Sistemi:** Windows 11 Pro 64-bit
- **Grafik Ayarları:** 1920x1080 (1080p), 16 Chunk Görüş Mesafesi, Sınırsız Kare Hızı (Max FPS), VSync Kapalı
- **Sürüm:** Minecraft Java 1.21.11 vs CREW Client

| Test Sahnesi | Vanilla Java 1.21.11 | CREW Client | Hızlanma Katsayısı | %1 Lows (Vanilla vs CREW) |
|---|:---:|:---:|:---:|:---:|
| **Köy (Village)** | 185 FPS | **412 FPS** | **2.23x** | 82 FPS → **198 FPS** |
| **Orman (Forest)** | 172 FPS | **394 FPS** | **2.29x** | 74 FPS → **186 FPS** |
| **Okyanus (Ocean)** | 210 FPS | **420 FPS** | **2.00x** | 98 FPS → **215 FPS** |
| **Dağ (Mountain)** | 168 FPS | **381 FPS** | **2.27x** | 71 FPS → **179 FPS** |
| **Tepe RAM Tüketimi** | 3.4 GB | **1.2 GB** | **%64 Tasarruf** | - |

> [!NOTE]
> **Hızlanma Hesaplama Formülü:**
> $$\text{Hızlanma Katsayısı} = \frac{\text{CREW Client Ortalama FPS}}{\text{Vanilla Java 1.21.11 Ortalama FPS}}$$
> Ortalama performans artışı donanımınıza bağlı olarak **2.00x ile 2.29x** arasında gerçekleşmektedir.

Kendi donanımınızdaki tahmini FPS artışını görmek için web sitemizdeki [İnteraktif Performans Hesaplayıcıyı](https://client.crewnetwork.space/performans/) kullanabilirsiniz.

---

## 3. Kendi Bilgisayarınızda Test Çalıştırma

CREW Client'ın dahili benchmark modunu çalıştırmak için terminalden aşağıdaki komutu verebilirsiniz:

```powershell
crew-client.exe --benchmark standard --render-distance 16
```

Bu komut:
1. `crew-benchmark` isimli geçici bir test dünyası oluşturur.
2. Dört sahneyi otomatik olarak uçarak kaydeder.
3. Sonuçları `%APPDATA%\CREW Client\benchmarks\` dizinine detaylı JSON ve metin raporu olarak yazar.
4. Mevcut dünyalarınıza veya ayarlarınıza kesinlikle dokunmaz.

---

*CREW Network © 2026 — [client.crewnetwork.space](https://client.crewnetwork.space/)*
