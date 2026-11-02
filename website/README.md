# CREW Client — Resmi Web Sitesi & Cloudflare Pages Dağıtım Kılavuzu

Bu dizin (`website/`), CREW Client'ın resmi web sitesini (`https://client.crewnetwork.space/`) oluşturan statik dosyaları içerir. Herhangi bir harici JavaScript çatısı veya derleme aracı gerektirmeden saf HTML5, modern CSS3 ve optimize edilmiş JavaScript ile çalışır.

---

## 📁 Dizin Yapısı

```
website/
├── _headers                   # Cloudflare Pages güvenlik ve önbellekleme başlıkları
├── wrangler.toml              # Cloudflare Pages proje yapılandırması
├── index.html                 # Ana sayfa (Hero, Özellikler, Performans, Videolar)
├── ozellikler/
│   └── index.html             # Ayrıntılı özellikler sayfası
├── performans/
│   └── index.html             # İnteraktif FPS hesaplayıcı & doğrulanmış benchmarklar
├── indir/
│   └── index.html             # Setup & Portable indirmeleri ve SHA-256 kodları
├── surumler/
│   └── index.html             # Sürüm geçmişi & katman ayrımı
├── surum-notlari/
│   └── index.html             # Türkçe değişiklik günlüğü (changelog)
├── stats/
│   └── index.html             # Canlı GitHub ve Minecraft sunucu istatistikleri
├── videolar/
│   └── index.html             # YouTube kanal akışı & Shorts filtresi
├── dokumanlar/
│   └── index.html             # Dokümantasyon rehberi ve SSS
└── assets/
    ├── css/style.css          # Koyu tema, akıcı grid ve tipografi
    ├── js/main.js             # Mobil menü, GitHub Releases API, YouTube senkronizasyonu
    ├── data/benchmarks.json   # Doğrulanmış laboratuvar benchmark veri seti
    └── img/                   # CREW marka SVG ve PNG logoları / simgeleri
```

---

## 🚀 Yerel Test (Local Preview)

Herhangi bir statik HTTP sunucusu ile test edebilirsiniz:

### Python 3 ile:
```powershell
cd website
python -m http.server 8080
```
Tarayıcınızda açın: `http://localhost:8080/`

### Node.js / npx ile:
```bash
cd website
npx serve .
```

---

## 🌐 Cloudflare Pages Dağıtımı (Deployment)

Web sitesini Cloudflare Pages üzerine iki farklı yöntemle dağıtabilirsiniz:

### Yöntem 1: GitHub Entegrasyonu (Önerilen)

1. [Cloudflare Dashboard](https://dash.cloudflare.com/)'a giriş yapın.
2. Sol menüden **Compute (Workers & Pages)** > **Create** > **Pages** > **Connect to Git** seçeneğine tıklayın.
3. `CodingM-eng/CREW-Client` (veya ilgili depo) reposunu seçin.
4. Yapılandırma ayarlarını aşağıdaki gibi girin:
   - **Project Name:** `crew-client`
   - **Production Branch:** `main` (veya `master`)
   - **Framework Preset:** `None`
   - **Build Command:** *(Boş bırakın)*
   - **Build Output Directory:** `website`
5. **Save and Deploy** butonuna tıklayın. Siteniz saniyeler içinde `<project>.pages.dev` üzerinde yayına girecektir.

### Yöntem 2: Wrangler CLI ile Doğrudan Yükleme

Cloudflare Wrangler CLI yüklüyse doğrudan terminalden dağıtabilirsiniz:

```bash
# İlk dağıtım veya önizleme
npx wrangler pages deploy website --project-name=crew-client

# Canlı (production) dağıtım
npx wrangler pages deploy website --project-name=crew-client --branch=main
```

---

## 🔗 Özel Alan Adı (Custom Domain) Bağlama: `client.crewnetwork.space`

`client.crewnetwork.space` alt alan adını Cloudflare Pages projenize bağlamak için:

1. Cloudflare Dashboard'da projenize gidin (`crew-client`).
2. **Custom Domains** sekmesine tıklayın ve **Set up a custom domain** butonuna basın.
3. Alan adı olarak `client.crewnetwork.space` yazın ve **Continue** deyin.
4. Eğer `crewnetwork.space` alan adınız aynı Cloudflare hesabındaysa, DNS CNAME kaydı otomatik olarak eklenecektir.
5. Harici bir DNS sağlayıcı kullanıyorsanız, aşağıdaki DNS kaydını oluşturun:
   - **Kayıt Türü:** `CNAME`
   - **Ad / Host:** `client`
   - **Hedef:** `crew-client.pages.dev`
   - **TTL:** `Auto` (veya `300`)
   - **Proxy Durumu:** `Proxied` (Turuncu Bulut)

---

## 🔒 Güvenlik & SSL Ayarları

- Cloudflare **SSL/TLS** modunu **Full (Strict)** olarak ayarlayın.
- **Always Use HTTPS** ve **Automatic HTTPS Rewrites** seçeneklerini aktif tutun.
- Web sitesinde bulunan `_headers` dosyası; Clickjacking (`X-Frame-Options: DENY`), MIME sniffing (`X-Content-Type-Options: nosniff`), Strict CSP ve varlıklar için 1 yıllık uzun vadeli önbellek (`immutable`) kurallarını otomatik olarak uygular.
