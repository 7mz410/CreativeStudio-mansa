# mansa: changes from upstream Craft Launcher

Required by the Apache License 2.0 (section 4b): files we changed carry this notice through
this list.

- Branding (user-visible only, applied by `studio/rebrand.py`): the app name mansa, the bundle id
  `ps.creative.studio.mansa`, links to mansa.studio.creative.ps and this repository. Crate names,
  settings folders and the file format keep their upstream names.
- No Discord buttons; community links point to Creative Studio.
- App icon: the mansa star on navy (`assets/app-icon/`).
- Removed upstream's ArtCraft brand assets (`docs/brand/`), as their brand terms require of forks.
- README (upstream's is `docs/UPSTREAM-README.md`) and NOTICE: the fork, its credits and the suite.
- Release feed removed: mansa reads releases from GitHub only (`7mz410/CreativeStudio-<app>`). The
  ArtCraft release feed reader, its notices (news and alerts) and the feed's throttle are gone.
- Details drawer: no pictures. Upstream's screenshots came from `storytold/craft-launcher-assets`
  and were removed; the drawer shows text only. Our own screenshots can come later.
- Arabic is right to left: `vendor/egui` is egui 0.36.2 with a small right-to-left switch
  (`egui::set_rtl`, marked "Creative Studio"; upstream's MIT OR Apache-2.0 license applies, see
  `LICENSE-MIT` and `LICENSE-APACHE`). Its catalog is in visual order (`i18n/bidi.rs`). See
  `studio/RTL.md`.
