# CREW Client — Microsoft Hesabı ile Giriş ve Kimlik Doğrulama Rehberi

Bu belge, CREW Client'ın resmi Microsoft OAuth2 Device Code akışını, Azure Uygulama Kaydı (App Registration) adımlarını, Client ID yapılandırma katmanlarını, token saklama güvenliğini ve çevrimdışı (Demo) mod mekanizmasını ayrıntılarıyla açıklamaktadır.

---

## 1. Genel Bakış

CREW Client, oyuncuların resmi Mojang/Microsoft sunucularına bağlanabilmesi için standart Minecraft Java Edition OAuth2 Device Code protokolünü (RFC 8628) kullanır. Bu protokol sayesinde:
- Oyuncunun parolası asla CREW Client tarafından görülmez veya saklanmaz.
- Giriş işlemi doğrudan resmi `microsoft.com/link` adresinde güvenli bir şekilde tamamlanır.
- Alınan erişim anahtarları Windows Kimlik Bilgisi Yöneticisi (Windows Credential Manager) üzerinde güvenli bir biçimde şifrelenerek saklanır.
- Lisanslı hesaba sahip olmayan kullanıcılar için veya hızlı test amacıyla **Demo / Çevrimdışı Mod** desteği sunulur.

---

## 2. Azure Uygulama Kaydı (App Registration)

Kendi Client ID'nizi kullanmak veya geliştirme yapmak istiyorsanız aşağıdaki adımları izleyebilirsiniz:

1. **Azure Portal'a Giriş:**
   - [Azure Portal](https://portal.azure.com/) adresine gidin.
   - **Microsoft Entra ID** (eski adıyla Azure Active Directory) > **Uygulama kayıtları (App registrations)** > **Yeni kayıt (New registration)** seçeneğine tıklayın.
2. **Uygulama Bilgileri:**
   - **Ad:** `CREW Client` (Kullanıcılara onay ekranında görünecektir).
   - **Desteklenen hesap türleri:** Yalnızca kişisel Microsoft hesapları (**Personal Microsoft accounts only**). Minecraft tüketici hesapları `consumers` kiracısında çalışır.
   - **Yeniden Yönlendirme URI (Redirect URI):** Platform olarak "Mobil ve masaüstü uygulamaları" seçin ve `https://login.microsoftonline.com/common/oauth2/nativeclient` değerini ekleyin.
3. **Genel İstemci Akışını Etkinleştirme:**
   - Uygulama kaydedildikten sonra sol menüden **Kimlik Doğrulama (Authentication)** sekmesine gidin.
   - **Gelişmiş ayarlar (Advanced settings)** altında **Genel istemci akışlarına izin ver (Allow public client flows)** seçeneğini **Evet (Yes)** olarak işaretleyin ve kaydedin.
4. **İstemci Kimliği (Application / Client ID):**
   - **Genel Bakış (Overview)** sayfasına dönerek oluşturulan GUID biçimindeki **Uygulama (istemci) Kimliği** değerini kopyalayın (Örn: `12345678-abcd-1234-abcd-1234567890ab`).

---

## 3. Client ID Yapılandırma Katmanları

CREW Client, istemci kimliğini (Client ID) 4 farklı kaynaktan hiyerarşik olarak okur. Bu sayede başlatıcıyı yeniden derlemeden dilediğiniz gibi yapılandırabilirsiniz:

| Öncelik | Yöntem | Açıklama |
|---|---|---|
| **1. Öncelik** | **Uygulama İçi Arayüz (In-App)** | Başlatıcı açıkken Giriş modalındaki *"Özel Client ID Gir"* alanına yapıştırıp *"Kaydet"* demeniz yeterlidir. |
| **2. Öncelik** | **Yapılandırma Dosyası** | `%APPDATA%\CREW Client\auth.json` dosyası içerisindeki `"client_id"` anahtarı. |
| **3. Öncelik** | **Ortam Değişkeni** | `CREW_MSA_CLIENT_ID` veya `HORIZON_MSA_CLIENT_ID` sistem/oturum ortam değişkeni. |
| **4. Öncelik** | **Derleme Zamanı** | Kaynak kod derlenirken tanımlanan sabit değer (`option_env!("CREW_MSA_CLIENT_ID")`). |

> [!TIP]
> En kolay yöntem, başlatıcı arayüzünde "Giriş Yap" penceresini açıp Client ID'nizi doğrudan kutucuğa yapıştırmaktır. Başlatıcı bu değeri otomatik olarak `%APPDATA%\CREW Client\auth.json` dosyasına işler.

---

## 4. Kimlik Doğrulama Akışı (Device Code Flow)

1. **Cihaz Kodu Talebi:**
   İstemci, `POST https://login.microsoftonline.com/consumers/oauth2/v2.0/devicecode` uç noktasına `client_id` ve `scope=XboxLive.signin offline_access` parametreleriyle istek gönderir.
2. **Kullanıcı Onayı:**
   Başlatıcı ekranda 8 haneli tek kullanımlık bir kod (`user_code`) gösterir ve varsayılan web tarayıcısında `https://www.microsoft.com/link` sayfasını açar. Oyuncu kodu tarayıcıya girerek Microsoft hesabıyla oturum açar.
3. **Yoklama (Polling):**
   Başlatıcı `interval` aralığıyla `POST .../consumers/oauth2/v2.0/token` uç noktasını yoklar. Oyuncu onay verdiğinde Microsoft erişim anahtarı ve yenileme anahtarı (refresh token) teslim alınır.
4. **Xbox Live & XSTS Kimlik Doğrulaması:**
   - `https://user.auth.xboxlive.com/user/authenticate` üzerinden Xbox Live belirteci ve kullanıcı karması (`uhs`) alınır.
   - `https://xsts.auth.xboxlive.com/xsts/authorize` üzerinden Minecraft hizmetleri için XSTS yetkilendirme belirteci temin edilir.
5. **Minecraft Services Oturumu:**
   `POST https://api.minecraftservices.com/authentication/login_with_xbox` çağrısı yapılarak 24 saat geçerli resmi Minecraft oturum jetonu temin edilir.
6. **Oyun Sahipliği ve Profil:**
   - `GET https://api.minecraftservices.com/entitlements/mcstore` ile hesabın Minecraft Java Edition lisansı doğrulanır.
   - `GET https://api.minecraftservices.com/minecraft/profile` ile oyuncunun UUID'si, oyun içi adı, pelerin ve dış görünüş (skin) bilgileri çekilir.

---

## 5. Token Saklama ve Güvenlik

- **Windows Credential Manager Entegrasyonu:**
  Hassas veriler (Microsoft refresh token ve Minecraft session token) Windows Kimlik Bilgisi Yöneticisi (`CREW Client Microsoft Token`) kasasında şifrelenir.
- **Düz Metin Parola Bulunmaz:**
  Oyuncunun şifresi hiçbir zaman disk üzerinde veya bellekte tutulmaz.
- **Yenileme Mekanizması:**
  Oturum süresi dolmaya yakınken (1 saatten az kaldığında) başlatıcı arka planda yenileme anahtarını kullanarak oturumu otomatik olarak tazeler.
- **Çıkış Yap:**
  Kullanıcı arayüzünden "Çıkış Yap" butonuna basıldığında Windows Kimlik Bilgisi Yöneticisi ve yerel yapılandırmadaki tüm oturum anahtarları kalıcı olarak silinir.

---

## 6. Demo / Çevrimdışı Mod

Henüz bir Microsoft hesabıyla giriş yapmamış veya internet bağlantısı olmayan kullanıcılar, başlatıcı ana ekranındaki **"Demo / Çevrimdışı Modda Başlat"** seçeneğini kullanarak yerel dünyalarda ve çevrimdışı sunucularda CREW Client'ın yüksek performanslı Rust motorunu ve Fabric 1.21.11 ortamını hemen test edebilirler.

---

*CREW Network © 2026 — [client.crewnetwork.space](https://client.crewnetwork.space/)*
