# Attribution

Every non-code asset in this repository, with its author, source and license. Mansa's (Creative Studio's
launcher) own code and original assets are MIT OR Apache-2.0 (see [`LICENSE-MIT`](LICENSE-MIT),
[`LICENSE-APACHE`](LICENSE-APACHE) and [`NOTICE`](NOTICE)). When you add an asset, add a row here
in the same change; third-party assets must be permissively licensed and keep their license file
next to them.

## Bundled in the app

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `docs/studio/mansa.svg` | Hamza Abu Ayyash (Creative Studio) | Original artwork | MIT OR Apache-2.0 | mansa logo; replaces the upstream brand |
| `assets/app-icon/` (all files) | Hamza Abu Ayyash (Creative Studio) | the mansa star (`docs/studio/mansa.svg`) on a navy tile, rendered by `packaging/icons.sh` | MIT OR Apache-2.0 | App icon (see `assets/app-icon/README.md`) |
| `assets/social/linkedin.svg` | LinkedIn logo (not shown: the launcher has no social links yet) | Fonticons, Inc. ([Font Awesome Free](https://fontawesome.com) 6.7.2) | Font Awesome Free `svgs/brands/linkedin.svg`; white fill and a square view box, see [`assets/social/README.md`](assets/social/README.md) | CC BY 4.0. The logo is a trademark of LinkedIn Corporation. |
| `assets/social/x.svg`, `youtube.svg`, `tiktok.svg`, `instagram.svg` | X, YouTube, TikTok and Instagram logos, linking to ArtCraft's accounts | [Simple Icons](https://simpleicons.org/) contributors | Simple Icons, `fill` set to white for tinting; see [`assets/social/README.md`](assets/social/README.md) | CC0 1.0. The logos are trademarks of X Corp., Google LLC, TikTok / ByteDance and Meta Platforms, Inc. |
| (not bundled) the pictures on each app's details page | Screenshots of the Craft Apps, fetched when a page is opened and kept in the launcher's cache folder | The apps' contributors | [storytold/craft-launcher-assets](https://github.com/storytold/craft-launcher-assets), pinned to a tag (`SCREENSHOTS_REV` in `crates/core/src/details.rs`); each picture's source and license are listed in that repository's `ATTRIBUTION.md` | Each app's own license for the interface (MIT OR Apache-2.0); the work shown in them is public domain or made in the app, as that file says |
| `assets/fonts/Inter-Variable.ttf` | Inter 4.001, variable (body text) | The Inter Project Authors (Rasmus Andersson) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/inter> (`Inter[opsz,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Inter.txt`](assets/fonts/OFL-Inter.txt) |
| `assets/fonts/Archivo-Variable.ttf` | Archivo 2.001, variable (headings and app names) | The Archivo Project Authors (Omnibus-Type) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/archivo> (`Archivo[wdth,wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-Archivo.txt`](assets/fonts/OFL-Archivo.txt) |
| `assets/fonts/GeistMono-Variable.ttf` | Geist Mono 1.701, variable (labels and buttons) | The Geist Project Authors (Vercel) | Google Fonts, <https://github.com/google/fonts/tree/main/ofl/geistmono> (`GeistMono[wght].ttf`, renamed) | SIL OFL 1.1, [`assets/fonts/OFL-GeistMono.txt`](assets/fonts/OFL-GeistMono.txt) |

### Creative Studio app icons

The launcher shows each Creative Studio app with its own star icon, copied from that app's fork (128 px). Each is the star on a navy tile, rendered by the fork's `packaging/icons.sh`.

| Path | Title | Author | Source | License |
|---|---|---|---|---|
| `assets/app-icons/tlween.png` | PhotoCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-tlween`](https://github.com/7mz410/CreativeStudio-tlween) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/rsm.png` | VectorCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-rsm`](https://github.com/7mz410/CreativeStudio-rsm) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/tsweer.png` | FilmCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-tsweer`](https://github.com/7mz410/CreativeStudio-tsweer) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/lamasat.png` | LightCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-lamasat`](https://github.com/7mz410/CreativeStudio-lamasat) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/effectat.png` | EffectCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-effectat`](https://github.com/7mz410/CreativeStudio-effectat) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/sf7aat.png` | PdfCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-sf7aat`](https://github.com/7mz410/CreativeStudio-sf7aat) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/trteeb.png` | DesignCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-trteeb`](https://github.com/7mz410/CreativeStudio-trteeb) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/klmat.png` | WordCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-klmat`](https://github.com/7mz410/CreativeStudio-klmat) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/3urood.png` | DeckCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-3urood`](https://github.com/7mz410/CreativeStudio-3urood) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/arqam.png` | GridCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-arqam`](https://github.com/7mz410/CreativeStudio-arqam) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/muhandes.png` | CADCraft fork app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | [`7mz410/CreativeStudio-muhandes`](https://github.com/7mz410/CreativeStudio-muhandes) `assets/app-icon/hicolor/128x128/apps/` (the fork's own icon), copied unmodified | MIT OR Apache-2.0 |
| `assets/app-icons/7rrek.png` | 7rrek app icon (the star on a navy tile), 128 px | Hamza Abu Ayyash (Creative Studio) | Rendered for this fork from `logos/Creative_Studio/7rrek.svg` on the navy tile used by the other apps | MIT OR Apache-2.0 |

## Documentation and README

| Path | Title | Author | Source | License |
|---|---|---|---|---|

> Open item: `packaging/macos/dmg/` still carries the ArtCraft mark (upstream `docs/brand/`, now removed). It must be replaced before any DMG is built.
