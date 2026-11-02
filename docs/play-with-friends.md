# CREW Client — Arkadaşlarla Oynama ve Davet Kodu Rehberi

Bu belge, CREW Client'ın port açma veya harici sunucu kiralama ihtiyacı duymadan, tek kişilik dünyaları tek tıkla arkadaşlara açma mekanizmasını (Play with Friends) açıklamaktadır.

---

## 1. Nasıl Çalışır?

Geleneksel Minecraft sunucularında arkadaşlarla oynamak için modemden port açmak (Port Forwarding) veya karmaşık üçüncü taraf VPN araçları kurmak gerekir. CREW Client bu süreci tamamen otomatikleştirir:

1. **Tek Tıkla Paylaşım:** Oyuncu duraklatma (Pause) menüsünden **"Arkadaşlarla Oyna"** butonuna basar.
2. **Otomatik Port ve Tünel:** İstemci, yerel ağınızdaki yönlendiriciyle (UPnP/NAT-PMP) anlaşarak doğrudan şifreli bir bağlantı tüneli hazırlar.
3. **Davet Kodu Üretimi:** Oyuncuya kopyalanabilir 5'li gruplardan oluşan benzersiz bir **Davet Kodu** verilir (Örn: `2A7KQ-M3XPD-98NZ2-K4F19`).
4. **Hızlı Bağlantı:** Arkadaşınız kendi CREW Client oyununda bu kodu girdiğinde doğrudan sizin dünyanıza bağlanır.

```
 Oyuncu A (Dünya Sahibi)                       Oyuncu B (Misafir)
 ┌──────────────────────┐                     ┌──────────────────────┐
 │ Yerel Entegre Sunucu │                     │ CREW Client İstemci  │
 └──────────┬───────────┘                     └──────────▲───────────┘
            │                                            │
            ▼                                            │
 ┌──────────────────────┐   Uçtan Uca Şifreli Tünel      │
 │ Şifreli Ev Sahibi    │════════════════════════════════╛
 │ Tüneli               │  (Doğrudan P2P / NAT Geçişi)
 └──────────────────────┘
```

---

## 2. Dünyanızı Paylaşma Adımları

1. Tek kişilik bir dünyaya girin.
2. `Esc` tuşuna basarak duraklatma menüsünü açın.
3. **Arkadaşlarla Oyna** butonuna tıklayın.
4. Oyun modunu ve hile izinlerini belirledikten sonra **"Paylaşımı Başlat"** seçeneğine basın.
5. Ekranda beliren Davet Kodunu panoya kopyalayın ve arkadaşınıza gönderin.

---

## 3. Bir Arkadaşın Dünyasına Katılma

1. CREW Client ana menüsünde veya başlatıcıda **"Arkadaşa Katıl"** seçeneğine tıklayın.
2. Arkadaşınızdan aldığınız davet kodunu kutucuğa yapıştırın.
3. **"Bağlan"** butonuna basın. İstemci tünel adreslerini otomatik olarak çözümleyecek ve dünyaya giriş yapacaktır.

---

## 4. Güvenlik ve Gizlilik

- **Uçtan Uca Şifreleme:** İki oyuncu arasındaki tüm paketler doğrudan güvenli şifreleme anahtarları ile korunur.
- **Geçici İzinler:** Davet kodu yalnızca dünya açık olduğu sürece geçerlidir. Dünyadan çıktığınız anda tünel kapanır ve kod geçersiz hale gelir.
- **Yazım Hatası Koruması:** Kodlar dahili 16-bit sağlama toplamı (checksum) içerir; yanlış bir karakter girildiğinde sistem bağlantı denemeden önce hatayı bildirir.

---

*CREW Network © 2026 — [client.crewnetwork.space](https://client.crewnetwork.space/)*
