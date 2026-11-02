# Güvenlik Politikası — CREW Client

CREW Client ve CREW Network olarak kullanıcılarımızın güvenliğine, veri mahremiyetine ve yazılım bütünlüğüne en yüksek önceliği veriyoruz.

---

## 1. Desteklenen Sürümler

Aşağıdaki tabloda güvenlik yamalarını ve resmi güncellemeleri alan sürümler belirtilmiştir:

| Sürüm | Destek Durumu | Açıklama |
| :--- | :---: | :--- |
| **v2.1.x** | ✅ Aktif Destek | En son kararlı sürüm; güvenlik yamaları ve güncellemeler uygulanır |
| **v2.0.x** | ⚠️ Sınırlı Destek | Kritik güvenlik yamaları sağlanır, v2.1.x'e geçiş önerilir |
| **v1.x / Eski** | ❌ Desteklenmiyor | Güvenlik desteği sona ermiştir |

---

## 2. Güvenlik Mimarisi ve Prensiplerimiz

CREW Client, kullanıcı verilerini korumak için tasarlanmış modern güvenlik standartlarını uygular:

1. **Parolasız Microsoft Girişi (OAuth 2.0 Device Code):**
   - İstemci hiçbir zaman Microsoft veya Minecraft şifrenizi sormaz, okumaz veya sunuculara göndermez.
   - Kimlik doğrulama doğrudan Microsoft'un resmi `microsoft.com/link` sayfası üzerinden kullanıcı kodu ile gerçekleştirilir.
2. **Güvenli Token Saklama:**
   - Oturum belirteçleri Windows Kimlik Bilgisi Yöneticisi (Windows Credential Manager) kullanılarak işletim sistemi düzeyinde şifreli olarak korunur.
3. **SHA-256 Sağlama Toplamı ve Bütünlük Denetimi:**
   - İndirilen tüm Minecraft sürümleri, kütüphaneler, modlar ve başlatıcı güncellemeleri resmi SHA-256 / SHA-1 sağlama toplamlarıyla doğrulanmadan çalıştırılmaz.
4. **Sıfır İstenmeyen Telemetri:**
   - İstemci harici üçüncü parti izleyiciler, kişisel veri toplayıcılar veya gizli telemetri servisleri barındırmaz.

---

## 3. Güvenlik Açığı Bildirme Süreci

Eğer CREW Client uygulamasında veya altyapısında potansiyel bir güvenlik açığı tespit ettiyseniz, lütfen bunu herkese açık GitHub Issue alanında paylaşmak **yerine** aşağıdaki kanallardan bize gizlilik içinde iletiniz:

- **GitHub Güvenlik Bildirimi:** [Security Advisory Oluşturun](https://github.com/CodingM-eng/CREW-Client/security/advisories/new)
- **E-posta İletişimi:** `security@crewnetwork.space`

### Bildiriminizde Lütfen Şunları Belirtiniz:
- Açığın türü ve etkisi (örn. kimlik doğrulama baypas, uzaktan kod yürütme, veri sızıntısı)
- Açığın yeniden üretilebilmesi için adım adım talimatlar veya kavram kanıtı (PoC)
- Etkilenen işletim sistemi sürümü ve CREW Client sürümü
- Varsa önerilen geçici çözüm veya düzeltme yaması

---

## 4. İnceleme ve Yanıt Taahhüdü

1. **İlk Yanıt:** Bildiriminiz tarafımıza ulaştıktan sonra 48 saat içinde incelenir ve alındı onayı verilir.
2. **Doğrulama ve Yama:** Zafiyet doğrulandıktan sonra öncelikli olarak bir yama geliştirilir ve test edilir.
3. **Koordineli Yayın:** Güvenlik güncellemesi yayınlanana kadar zafiyet detayları gizli tutulur; yama yayınlandığında katkınız için teşekkür edilerek sürüm notlarında duyurulur.

*Topluluğumuzu ve oyuncularımızı güvende tutmamıza yardımcı olduğunuz için teşekkür ederiz.*
