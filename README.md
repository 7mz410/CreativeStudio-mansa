<p align="center">
  <img alt="mansa" src="docs/studio/mansa.svg" width="140">
</p>

<h1 align="center">mansa</h1>

<p align="center">
  <b>Free, open-source app launcher — part of the Creative Studio creative suite.</b><br>
  <b>مشغّل التطبيقات، مجاني ومفتوح المصدر — جزء من حزمة Creative Studio.</b>
</p>

<p align="center">
  <a href="https://mansa.studio.creative.ps/">mansa.studio.creative.ps</a> ·
  <a href="https://studio.creative.ps/">studio.creative.ps</a>
</p>

<p align="center">
  <img alt="License: MIT OR Apache-2.0" src="https://img.shields.io/badge/license-MIT%20OR%20Apache--2.0-555555">
  <img alt="Based on Craft Launcher" src="https://img.shields.io/badge/based%20on-Craft Launcher-0d2d3f">
</p>

## Built on Craft Launcher

mansa is a fork of **[Craft Launcher](https://github.com/storytold/craft-launcher)**, the open-source, clean-room
application written in Rust by the **[ArtCraft](https://getartcraft.com/)** team and the Craft Launcher
contributors. Almost everything that makes this app work is their work. Thank you. If you like
mansa, please star and support the upstream project as well.

What we add: the Creative Studio identity, the Arabic interface laid out right to left, and Arabic
features to come. Our changes are listed in [`studio/CHANGES.md`](studio/CHANGES.md).
Upstream's own README, with the full feature list and build instructions, is in
[`docs/UPSTREAM-README.md`](docs/UPSTREAM-README.md).

## The Creative Studio suite

Fifteen free, open-source apps with an Arabic interface laid out right to left. **mansa** installs and updates them all. The whole suite: [github.com/7mz410/CreativeStudio](https://github.com/7mz410/CreativeStudio) · [studio.creative.ps](https://studio.creative.ps/).

| App | | What it's for | Saves as | Based on |
|---|---|---|---|---|
| [rsm](https://github.com/7mz410/CreativeStudio-rsm) | رسم | Vector illustration | `.rsm` | VectorCraft |
| [7rrek](https://github.com/7mz410/CreativeStudio-7rrek) | حرّك | Animation | `.7rrek` | VectorCraft (via rsm) |
| [tlween](https://github.com/7mz410/CreativeStudio-tlween) | تلوين | Image editing | `.tlween` | PhotoCraft |
| [lamasat](https://github.com/7mz410/CreativeStudio-lamasat) | لمسات | Photo library and raw development | — | LightCraft |
| [tsweer](https://github.com/7mz410/CreativeStudio-tsweer) | تصوير | Video editing | `.tsweer` | FilmCraft |
| [effectat](https://github.com/7mz410/CreativeStudio-effectat) | تأثيرات | Motion graphics and effects | `.effectat` | EffectCraft |
| [aswat](https://github.com/7mz410/CreativeStudio-aswat) | أصوات | Audio recording and mixing | `.aswat` | SoundCraft |
| [al7an](https://github.com/7mz410/CreativeStudio-al7an) | ألحان | Music making | `.al7an` | SoundCraft |
| [trteeb](https://github.com/7mz410/CreativeStudio-trteeb) | ترتيب | Page layout | `.trteeb` | DesignCraft |
| [sharek](https://github.com/7mz410/CreativeStudio-sharek) | شارك | Interface and graphic design | `.sharek` | FigCraft |
| [sf7aat](https://github.com/7mz410/CreativeStudio-sf7aat) | صفحات | PDF documents | `.pdf` | PdfCraft |
| [klmat](https://github.com/7mz410/CreativeStudio-klmat) | كلمات | Word processing | `.docx` | WordCraft |
| [arqam](https://github.com/7mz410/CreativeStudio-arqam) | أرقام | Spreadsheets | `.xlsx` | GridCraft |
| [3urood](https://github.com/7mz410/CreativeStudio-3urood) | عروض | Presentations | `.3urood` | DeckCraft |
| [muhandes](https://github.com/7mz410/CreativeStudio-muhandes) | مهندس | Computer-aided design | `.dxf` | CADCraft |

## Building

```sh
cargo run --release -p craft-launcher
# after merging upstream:
git fetch upstream && git merge upstream/main && python3 studio/rebrand.py
```

## Licence

Code: MIT OR Apache-2.0 (`LICENSE-MIT`, `LICENSE-APACHE`, `NOTICE`), like upstream. The Creative Studio
name and logos in `docs/studio/` belong to the Creative Studio project.
