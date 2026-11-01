import glob
import re
import os
import json

src_files = glob.glob(r"c:\Users\efeka\Documents\antigravity\happy-mendeleev\release\horizon-main\launcher\src\**\*.rs", recursive=True)

literals = set()
for sf in src_files:
    content = open(sf, "r", encoding="utf-8", errors="ignore").read()
    matches = re.findall(r'"([^"\\]*(?:\\.[^"\\]*)*)"', content)
    for m in matches:
        s = m.replace(r'\"', '"').replace(r'\\', '\\').replace(r'\n', '\n')
        s = re.sub(r'\\\r?\n\s*', '', s)
        literals.add(s)

print(f"Total literals found: {len(literals)}")

lang_dir = r"c:\Users\efeka\Documents\antigravity\happy-mendeleev\release\horizon-main\launcher\lang"
with open(os.path.join(lang_dir, "tr.json"), "r", encoding="utf-8") as f:
    tr = json.load(f)

invalid_keys = [k for k in tr.keys() if k not in literals]
print(f"Keys in tr.json not in literals: {len(invalid_keys)}")
for k in invalid_keys:
    print("  ", repr(k))
