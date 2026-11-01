import json
import os

lang_dir = r"c:\Users\efeka\Documents\antigravity\happy-mendeleev\release\horizon-main\launcher\lang"
de_path = os.path.join(lang_dir, "de.json")
tr_path = os.path.join(lang_dir, "tr.json")

with open(de_path, "r", encoding="utf-8") as f:
    de_data = json.load(f)

with open(tr_path, "r", encoding="utf-8") as f:
    tr_data = json.load(f)

# Specific exact key mappings for the 4 telemetry/move keys
tr_data['Horizon can send a few things to its developers. Each switch is off until you turn it on, and Settings can change it any time.'] = (
    'CREW Client anonim performans ve kilitlenme telemetrisi toplayabilir. Varsayılan olarak kapalıdır ve Ayarlar\'dan her zaman değiştirilebilir.'
)
tr_data['This moves the instance to Horizon {version}. It compiles the game from scratch, so it takes longer than a normal rebuild: up to 30 minutes.'] = (
    'Bu işlem profili CREW Client {version} sürümüne taşır. Oyunu sıfırdan derlediğinden ilk seferde biraz zaman alabilir.'
)
tr_data["When the game crashes: the error, where in Horizon's code it happened, your graphics adapter and settings. No log, no world or server, no user name in paths."] = (
    'Oyun çöktüğünde: hata mesajı, ekran kartı ve grafik ayarlarınız. Günlük dosyası, dünya adı veya kullanıcı adı kesinlikle içermez.'
)
tr_data['Each switch is off unless you turn it on. What it sends is below it; the privacy page on playhorizon.dev says how long it is kept.'] = (
    'Her seçenek siz açmadığınız sürece kapalıdır. Nelerin gönderildiği altında belirtilmiştir; detaylı gizlilik politikası web sitemizde yer alır.'
)
tr_data['Accounts…'] = 'Hesaplar…'
tr_data['Choose…'] = 'Seçin…'

# Create aligned tr dictionary that has EXACTLY the same keys as de.json in the same order
aligned_tr = {}
for k in de_data.keys():
    if k in tr_data:
        aligned_tr[k] = tr_data[k]
    else:
        aligned_tr[k] = k

with open(tr_path, "w", encoding="utf-8") as f:
    json.dump(aligned_tr, f, ensure_ascii=False, indent=2)

print(f"tr.json aligned! Keys count: {len(aligned_tr)} (matches de.json: {len(de_data)})")
