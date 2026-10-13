# Right-to-left interface (mansa)

When the interface language is Arabic, mansa lays its interface out right to left: the brand and
the view switch sit on the right, the status on the left, and cards, rows, grids and the details
drawer (which slides in from the left) run from the right.

## How it works

egui lays everything out left to right and has no RTL mode, so we carry a small patch to egui
0.36.2 in `vendor/egui` (copied from the rsm fork, used through `[patch.crates-io]` in `Cargo.toml`).
Every change is marked `Creative Studio`:

- `layout.rs`: a process-wide switch, `egui::set_rtl(bool)` / `egui::is_rtl()`, and
  `egui::start_align()` (left, or right when RTL). `Layout::default()` starts from it.
- `containers/panel.rs`: panels start from `Layout::default()`.
- `containers/menu.rs`, `containers/popup.rs`: menu bars run right to left, menus and popups align
  right.
- `ui.rs`: `ui.vertical` aligns to the start edge, and `ui.columns` puts the first column on the
  right. `ui.horizontal` already followed its parent's direction.
- `grid.rs`: grid columns run from the right.

In our code:
- `app.rs` turns the switch on each frame when the language is in `i18n::bidi::RTL_CODES`. Unit
  tests leave it off, because they run in parallel and the switch is process-wide;
  `crates/ui-egui/tests/rtl.rs` and `tests/arabic.rs` test it in their own processes.
- `views.rs`: `far_layout()`, `near_layout()` and `start_align()` swap the sides for the header,
  toolbar, footer, cards and the drawer's close button.
- `i18n/bidi.rs` keeps `{placeholders}` whole when it puts Arabic text in visual order.

## Upgrading egui

When upstream moves to a newer egui, the patch stops applying (Cargo warns that the patch is
unused, and `egui::set_rtl` no longer exists, so the build fails). Copy the new egui into
`vendor/egui` and re-apply the `Creative Studio` changes listed above.

## Still left to right

The table view (fixed column positions), the dialogs' button rows, and the toasts. They need
mirroring one by one.
