<div align="center">

# 🗡️ Skyblock Lore Renderer

### 🎨 Turn Minecraft Hypixel Skyblock item lore into beautiful images

[![Rust](https://img.shields.io/badge/Rust-1.85+-orange?logo=rust)](https://www.rust-lang.org)
[![Docker](https://img.shields.io/badge/Docker-Ready-2496ED?logo=docker)](https://www.docker.com)
[![License](https://img.shields.io/badge/License-MIT-blue.svg)](LICENSE)

<p align="center">
  <img src="examples/hyperion.png" width="500" alt="Hyperion Example">
</p>

</div>

---

## ✨ Features

| Feature | Description |
|---------|-------------|
| 🎨 **Full Color Support** | Renders all Minecraft `§` format codes with accurate colors |
| ✨ **Tier 6 & 7 Support** | Gold color for t6 and purple color for t7 enchantments |
| 🔤 **Faithful Font** | Uses the iconic Minecraft Faithful Unicode font for authentic look |
| 📏 **Auto-sizing** | Automatically calculates image dimensions based on text content |
| 🐳 **Docker Ready** | Run in a container without installing Rust locally |
| ⚡ **Fast & Lightweight** | Pure Rust with minimal dependencies — blazing fast rendering |

---

## 🚀 Quick Start

### 🐳 Via Docker (Recommended)

If you have Docker installed, you can run the renderer without installing Rust or compiling anything:

```bash
# 1. Clone the repository
git clone https://github.com/megawattka/skyblock-lore-renderer.git
cd skyblock-lore-renderer

# 2. Run application via docker compose
docker compose up --build -d
```

#### 🔧 Local Build

```bash
# Clone the repository
git clone https://github.com/megawattka/skyblock-lore-renderer.git
cd skyblock-lore-renderer

# Build in release mode
cargo build --release

# Binary will be available at ./target/release/skyblock-lore-renderer
```

---

## 📖 Usage

```python
import json
import urllib.request as ur

url = "http://127.0.0.1:8080/render"
with open("./examples/plasmaflux.txt") as fp:
    lore = fp.read()

payload = json.dumps({
    "lore": lore,
    "options": {
        "background": "#000000"
    }
}).encode()

headers = {"content-type": "application/json"} 
request = ur.Request(url, data=payload, headers=headers)

rendered = ur.urlopen(request).read()
r_json = json.loads(rendered)

print(r_json)
# {
#     'image': 'iVBORw0KGgoAAAANSUhEUgAAAj...',
#     'width': 574,
#     'height': 544,
#     'render_time_ms': 2.18,
# }
```

---

## 📝 Lore Format

The renderer supports standard Minecraft `§` format codes:

| Code | Color | Preview |
|------|-------|---------|
| `§0` | Black | ⬛ |
| `§1` | Dark Blue | 🟦 |
| `§2` | Dark Green | 🟩 |
| `§3` | Dark Aqua | 🩵 |
| `§4` | Dark Red | 🟥 |
| `§5` | Dark Purple | 🟪 |
| `§6` | Gold | 🟨 |
| `§7` | Gray | ⬜ |
| `§8` | Dark Gray | 🔲 |
| `§9` | Blue | 🔵 |
| `§a` | Green | 🟢 |
| `§b` | Aqua | 🩵 |
| `§c` | Red | 🔴 |
| `§d` | Light Purple | 🩷 |
| `§e` | Yellow | 🟡 |
| `§f` | White | ⚪ |
| `§l` | **Bold** | **B** |

### Example Input File

```text
§dHeroic Hyperion §6✪✪✪✪✪
§7Gear Score: §d1206 §8(4011)
§7Damage: §c+352 §e(+30) §8(+1,372.46)
§7Strength: §c+245 §e(+30) §9(+50) §8(+968.3)
§7Crit Damage: §9+70% §8(+294.7%)
§7Intelligence: §b+582 §9(+125) §8(+2,302.87)

§d§lUltimate Wise V, §9Bane of Arthropods VI
§9Champion X, §9Cleave V, §9Critical VI

§7Deals §c+50% §7damage to §8☠ Wither §7mobs.
§7Grants §c+1 §c❁ Damage §7and §a+2 §b✎ Intelligence §7per §cCatacombs §7level.

§6Ability: Wither Impact §e§lRIGHT CLICK
§7Teleport §a10 blocks§7 ahead of you
§7dealing §c144,841 §7damage to nearby enemies.

§8§l* §8Co-op Soulbound §8§l*
§d§l§ka§r §d§lMYTHIC DUNGEON SWORD §d§l§ka
```

---

## 🖼️ More Examples

### Plasmaflux Power Orb — Legendary Deployable
<p align="center">
  <img src="examples/plasmaflux.png" width="480" alt="Plasmaflux">
</p>

---

## ⚙️ How It Works

1. 📖 **Parse** — Regex extracts `§` format codes and text segments
2. 🎨 **Colorize** — Maps format codes to RGB colors via `phf` static map
3. 📐 **Measure** — Calculates text width using `ab_glyph` for proper image sizing
4. ✍️ **Render** — Draws text onto an `RgbImage` with `imageproc`
5. 💾 **Get the result** — Exports the final image as PNG

---

## 🛠️ Tech Stack

| Crate | Purpose |
|-------|---------|
| 🖼️ `image` + `imageproc` | Image creation and text drawing |
| 🔤 `ab_glyph` | Font loading and glyph metrics |
| ⚡ `phf` | Compile-time perfect hash maps for color codes |
| 📝 `regex` | Lore format code parsing |
| 🌿 `dotenvy` | Environment configuration |
| 📋 `anyhow` | Ergonomic error handling |
| 🪵 `log` + `env_logger` | Structured logging |
| ⚡ `warp` | WebServer Logic |

---

## 🎯 Roadmap

- [x] 🎨 Basic color format code support
- [x] ✨ Bold text rendering (`§l`)
- [x] 🖼️ Enchantments tier 6 and 7 custom colors toggle
- [ ] 🔄 Italic support (`§o`)
- [ ] ➖ Strikethrough support (`§m`)
- [ ] ➖ Underline support (`§n`)
- [x] 🖼️ Custom background support

---

## 📜 License

This project is licensed under the **MIT License**.  
Feel free to modify this API as you want.

---

<p align="center">
  <samp>
    Made with <b>❤️</b> by <a href="https://github.com/megawattka">@megawattka</a>
  </samp>
</p>

<p align="center">
  <sub><samp>independent · not affiliated with Hypixel or Mojang Studios</samp></sub>
</p>

<p align="center">
  <a href="https://github.com/megawattka/skyblock-lore-renderer">⭐ star this repo</a> if you found it useful
</p>
