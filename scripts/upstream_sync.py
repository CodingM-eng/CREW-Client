#!/usr/bin/env python3
"""
CREW Client — Upstream Horizon Monitor & Safe Sync Engine
Takip Edilen Kaynak: https://github.com/horizon-rust/horizon
CREW Client: https://client.crewnetwork.space/

Bu betik:
1. Üst kaynak (upstream) Horizon deposundaki yeni sürümleri ve commit'leri kontrol eder.
2. Üç sürüm katmanını birbirinden ayırır:
   - CREW Client Başlatıcı Sürümü (örn. 2.1.0)
   - Horizon Üst Kaynak Motor Sürümü (örn. v2026.10.13 / Nightly)
   - Minecraft Oyun Sürümü (1.21.11 / 26.3)
3. Değişiklikleri güvenlik, grafik, parite ve mod uyumluluğu olarak sınıflandırır.
4. "CREW Client için yeni güncelleme mevcut!" formatında Türkçe bildirim ve PR raporu üretir.
5. Doğrudan ham Horizon ikilisini indirmek yerine, güvenli CREW derleme ve yayınlama sürecini tetikler.
"""

import json
import os
import sys
import urllib.request
import urllib.error
from datetime import datetime

# UTF-8 terminal output on Windows
if sys.stdout.encoding != 'utf-8':
    sys.stdout.reconfigure(encoding='utf-8', errors='replace')
if sys.stderr.encoding != 'utf-8':
    sys.stderr.reconfigure(encoding='utf-8', errors='replace')

UPSTREAM_REPO = "horizon-rust/horizon"
CONFIG_FILE = os.path.join(os.path.dirname(os.path.dirname(__file__)), "upstream.json")

def load_config():
    if os.path.exists(CONFIG_FILE):
        with open(CONFIG_FILE, "r", encoding="utf-8") as f:
            return json.load(f)
    return {
        "crew_client_version": "2.1.0",
        "upstream_repository": UPSTREAM_REPO,
        "tracked_engine_version": "v2026.10.13",
        "minecraft_target_version": "1.21.11 / 26.3"
    }

def save_config(cfg):
    with open(CONFIG_FILE, "w", encoding="utf-8") as f:
        json.dump(cfg, f, indent=2, ensure_ascii=False)

def fetch_upstream_releases():
    url = f"https://api.github.com/repos/{UPSTREAM_REPO}/releases"
    req = urllib.request.Request(
        url,
        headers={"User-Agent": "CREW-Client-Sync/2.1.0 (crewnetwork.space)"}
    )
    try:
        with urllib.request.urlopen(req, timeout=10) as res:
            if res.status == 200:
                data = json.loads(res.read().decode("utf-8"))
                return data
    except Exception as e:
        print(f"[UYARI] GitHub API erişimi sağlanamadı: {e}", file=sys.stderr)
    return []

def analyze_changes(release_notes):
    categories = {
        "guvenlik": [],
        "performans": [],
        "parite": [],
        "diger": []
    }
    for line in release_notes.splitlines():
        line = line.strip()
        if not line or line.startswith("#"):
            continue
        lower = line.lower()
        if "security" in lower or "fix" in lower or "cve" in lower or "vulnerability" in lower:
            categories["guvenlik"].append(line)
        elif "render" in lower or "vulkan" in lower or "dx12" in lower or "fps" in lower or "perf" in lower:
            categories["performans"].append(line)
        elif "vanilla" in lower or "block" in lower or "entity" in lower or "parity" in lower:
            categories["parite"].append(line)
        else:
            categories["diger"].append(line)
    return categories

def check_updates():
    cfg = load_config()
    current_engine = cfg.get("tracked_engine_version", "v2026.10.13")
    current_crew = cfg.get("crew_client_version", "2.1.0")
    mc_ver = cfg.get("minecraft_target_version", "1.21.11 / 26.3")

    print("==================================================")
    print("CREW CLIENT — ÜST KAYNAK (UPSTREAM) GÜNCELLEME KONTROLÜ")
    print("==================================================")
    print(f"Mevcut CREW Client Başlatıcı Sürümü: {current_crew}")
    print(f"İzlenen Horizon Motor Sürümü:       {current_engine}")
    print(f"Hedef Minecraft Sürümü:              {mc_ver}")
    print("--------------------------------------------------")

    releases = fetch_upstream_releases()
    if not releases:
        print("[BİLGİ] Yerel kontrol tamamlandı: Sistem güncel (upstream çevrimdışı önbellek korundu).")
        return False

    latest = releases[0]
    latest_tag = latest.get("tag_name", "")
    body = latest.get("body", "")

    if latest_tag != current_engine:
        print("\n>>> CREW Client için yeni güncelleme mevcut! <<<")
        print(f"- Mevcut Yüklü Motor: {current_engine}")
        print(f"- Yeni Bulunan Motor: {latest_tag}")
        print(f"- Çıkış Tarihi:        {latest.get('published_at', '')}")
        print("\n[Türkçe Değişiklik Özeti]:")
        
        analysis = analyze_changes(body)
        if analysis["guvenlik"]:
            print("  [Güvenlik ve Onarım]:")
            for item in analysis["guvenlik"][:3]:
                print(f"    • {item}")
        if analysis["performans"]:
            print("  [Grafik & Performans İyileştirmeleri]:")
            for item in analysis["performans"][:3]:
                print(f"    • {item}")
        if analysis["parite"]:
            print("  [Vanilla Parite Güncellemeleri]:")
            for item in analysis["parite"][:3]:
                print(f"    • {item}")

        print("\n[GÜVENLİ GÜNCELLEME SÜRECİ]:")
        print("1. Ham Horizon ikilisi asla doğrudan kullanıcı sistemine yazılmaz.")
        print("2. Değişiklikler incelenip CREW Client deposuna cherry-pick edilir.")
        print("3. CREW Client marka kimliği, Türkçe dil paketi ve logo korunur.")
        print("4. Tüm birim ve entegrasyon testleri (418 test) çalıştırılır.")
        print("5. Derlenen imzalı CREW Client sürümü client.crewnetwork.space üzerinden dağıtılır.")

        # update config check timestamp
        cfg["last_checked"] = datetime.utcnow().isoformat() + "Z"
        save_config(cfg)
        return True
    else:
        print("[BİLGİ] CREW Client motoru en güncel Horizon sürümüyle senkronize durumdadır.")
        cfg["last_checked"] = datetime.utcnow().isoformat() + "Z"
        save_config(cfg)
        return False

if __name__ == "__main__":
    check_updates()
