# CREW Client — Sunucu Sahipleri İçin Yapılandırma Rehberi (`server.toml`)

Bu belge, adanmış (dedicated) sunucu çalıştıran yöneticiler için standart `server.properties` dosyasının modern `server.toml` yapısına nasıl uyarlandığını açıklamaktadır.

---

## 1. Neden `server.toml`?

Geleneksel `server.properties` dosyası düz metin anahtar-değer yapısındadır ve karmaşık ayarları yönetmeyi zorlaştırır. `server.toml` ise:
- Okunabilir bölümlere (`[server]`, `[world]`, `[network]`) ayrılmıştır.
- Tip güvenli değerler (sayılar, boolean mantığı, dizgeler) kullanır.
- Başlatılmadan önce otomatik olarak standart `server.properties` formatına da dönüştürülebilir; böylece eski eklentilerle tam uyum sağlar.

---

## 2. Mevcut Sunucuyu İçe Aktarma

Eski bir `server.properties` dosyasını `server.toml` formatına dönüştürmek için:

```powershell
crew-server.exe import server.properties
```

Bu işlem:
1. Mevcut ayarlarınızı ayrıştırır.
2. Optimize edilmiş bir `server.toml` dosyası üretir.
3. Orijinal dosyanızı yedek olarak `server.properties.imported` adıyla saklar.

---

## 3. Ayar Eşleme Tablosu

| `server.properties` | `server.toml` | Varsayılan Değer | Açıklama |
|---|---|---|---|
| `motd` | `[server] motd` | `"CREW Network Sunucusu"` | Sunucu listesinde görünen başlık. |
| `server-port` | `[server] port` | `25565` | Sunucunun dinlediği TCP portu. |
| `server-ip` | `[server] bind` | `""` | Dinlenecek IP adresi (boş = tümü). |
| `max-players` | `[server] max_players` | `20` | Maksimum eşzamanlı oyuncu sayısı. |
| `online-mode` | `[server] online_mode` | `true` | Resmi Microsoft kimlik doğrulama denetimi. |
| `white-list` | `[server] whitelist` | `false` | Beyaz liste (izinli oyuncular) filtresi. |
| `view-distance` | `[server] view_distance` | `10` | Görüş mesafesi (chunk cinsinden). |
| `simulation-distance` | `[server] simulation_distance` | `10` | Varlık simülasyon mesafesi. |
| `level-name` | `[world] name` | `"world"` | Dünya klasörünün adı. |
| `level-seed` | `[world] seed` | `""` | Dünya oluşturma tohumu (boş = rastgele). |
| `difficulty` | `[world] difficulty` | `"normal"` | Zorluk seviyesi (peaceful, easy, normal, hard). |
| `hardcore` | `[world] hardcore` | `false` | Hardcore modu (öldükten sonra izleyici olma). |

---

## 4. Özel ve Ek Özellikler

Standart Minecraft'ın diğer tüm ince ayarlarını `[properties]` bloğu altında aynı isimle tanımlayabilirsiniz:

```toml
[properties]
spawn-protection = 16
allow-flight = false
enable-command-block = true
```

---

*CREW Network © 2026 — [client.crewnetwork.space](https://client.crewnetwork.space/)*
