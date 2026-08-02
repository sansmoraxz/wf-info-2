set shell := ["bash", "-euo", "pipefail", "-c"]

items_base_url := "https://raw.githubusercontent.com/WFCD/warframe-items/refs/heads/master/data/json"
items_dir := "testdata/items"

# List available recipes
default:
    @just --list

# Download the latest item-data test fixtures from WFCD/warframe-items
fetch-testdata:
    mkdir -p {{items_dir}}
    for f in \
        Arcanes.json Arch-Gun.json Arch-Melee.json Archwing.json \
        Enemy.json Fish.json Gear.json Glyphs.json Melee.json Misc.json \
        Mods.json Node.json Pets.json Primary.json Quests.json \
        Railjack.json Relics.json Resources.json Secondary.json \
        Sentinels.json SentinelWeapons.json Sigils.json Skins.json \
        Warframes.json; \
    do \
        echo "fetching $f"; \
        curl -fsSL --retry 3 -o "{{items_dir}}/$f.tmp" "{{items_base_url}}/$f"; \
        mv "{{items_dir}}/$f.tmp" "{{items_dir}}/$f"; \
    done
