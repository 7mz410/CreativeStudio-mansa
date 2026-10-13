//! The Craft App Suite: the twelve apps the launcher knows about.
//!
//! The list is data so a later version can fetch it from an API; [`SUITE`] is the built-in copy.
//!
//! Text here is English, the source the UI translates from: each `tagline` has a catalog row
//! keyed by the app id (`tr_ctx(app.id, app.tagline)` in `ui-egui`).

use serde::{Deserialize, Serialize};

/// Which generation of the suite an app belongs to.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum AppGroup {
    /// The seven original apps.
    Classic,
    /// The five apps added after them.
    New,
}

impl AppGroup {
    pub fn label(self) -> &'static str {
        match self {
            Self::Classic => "Classic",
            Self::New => "New",
        }
    }
}

/// One app in the suite.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct CraftApp {
    /// Machine name: repo, binary, package and asset stem (`tlween`).
    pub id: &'static str,
    /// Display name (`tlween`).
    pub name: &'static str,
    /// What it's for, in a few words (English; the UI translates it by app id).
    pub tagline: &'static str,
    pub group: AppGroup,
    /// Brand colour (sRGB), for surfaces: getartcraft.com's `--app`.
    pub color: [u8; 3],
    /// The brand colour for text on a light background (the site's light `--app-ink`).
    pub ink_light: [u8; 3],
    /// The brand colour for text on a dark background (the site's dark `--app-ink`).
    pub ink_dark: [u8; 3],
    /// GitHub `owner/repo`.
    pub repo: &'static str,
    /// Older names its release assets may carry (sf7aat shipped as sf7aat until 0.2.1).
    pub former_ids: &'static [&'static str],
    /// Older display names its installers registered (sf7aat's MSI said sf7aat until 0.2.1).
    pub former_names: &'static [&'static str],
}

impl CraftApp {
    /// `ai.storyteller.<id>`: macOS bundle id, Flatpak id and Linux desktop-file id.
    pub fn bundle_id(&self) -> String {
        format!("ai.storyteller.{}", self.id)
    }

    pub fn releases_url(&self) -> String {
        format!("https://github.com/{}/releases", self.repo)
    }

    pub fn repo_url(&self) -> String {
        format!("https://github.com/{}", self.repo)
    }

    /// The app's own page on Creative Studio's site.
    pub fn website(&self) -> String {
        format!("https://{}.studio.creative.ps", self.id)
    }

    /// Asset stems this app's releases may use, current first.
    pub fn stems(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.id).chain(self.former_ids.iter().copied())
    }

    /// Display names this app's installers may have registered, current first.
    pub fn names(&self) -> impl Iterator<Item = &'static str> + '_ {
        std::iter::once(self.name).chain(self.former_names.iter().copied())
    }

    pub fn by_id(id: &str) -> Option<&'static CraftApp> {
        SUITE.iter().find(|a| a.id == id)
    }
}

const fn app(
    id: &'static str,
    name: &'static str,
    tagline: &'static str,
    group: AppGroup,
    [color, ink_light, ink_dark]: [[u8; 3]; 3],
    repo: &'static str,
) -> CraftApp {
    CraftApp { id, name, tagline, group, color, ink_light, ink_dark, repo, former_ids: &[], former_names: &[] }
}

/// The Creative Studio suite: the twelve apps the launcher knows about.
/// Ids and names are the Creative Studio slugs; each `repo` is that app's fork under 7mz410.
pub const SUITE: [CraftApp; 12] = [
    app(
        "tlween",
        "tlween",
        "Image editing",
        AppGroup::Classic,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-tlween",
    ),
    app(
        "rsm",
        "rsm",
        "Vector illustration",
        AppGroup::Classic,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-rsm",
    ),
    app(
        "tsweer",
        "tsweer",
        "Video editing, color and sound",
        AppGroup::Classic,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-tsweer",
    ),
    app(
        "lamasat",
        "lamasat",
        "Photo library and raw development",
        AppGroup::Classic,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-lamasat",
    ),
    app(
        "effectat",
        "effectat",
        "Motion graphics and visual effects",
        AppGroup::Classic,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-effectat",
    ),
    app(
        "sf7aat",
        "sf7aat",
        "Reading, organizing and protecting PDFs",
        AppGroup::Classic,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-sf7aat",
    ),
    app(
        "trteeb",
        "trteeb",
        "Page layout and publishing",
        AppGroup::Classic,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-trteeb",
    ),
    app(
        "klmat",
        "klmat",
        "Writing and document design",
        AppGroup::New,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-klmat",
    ),
    app(
        "3urood",
        "3urood",
        "Presentations and slide shows",
        AppGroup::New,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-3urood",
    ),
    app(
        "arqam",
        "arqam",
        "Spreadsheets and data",
        AppGroup::New,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-arqam",
    ),
    app(
        "muhandes",
        "muhandes",
        "Computer-aided design and drafting",
        AppGroup::New,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-muhandes",
    ),
    app(
        "7rrek",
        "7rrek",
        "Motion and animation",
        AppGroup::New,
        [[0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c], [0xeb, 0x1b, 0x3c]],
        "7mz410/CreativeStudio-7rrek",
    ),
];

/// Creative Studio's website.
pub const WEBSITE_URL: &str = "https://studio.creative.ps";
/// The GitHub account every Craft App's repository lives under: the only one the launcher
/// installs from, whatever the release feed says.
pub const GITHUB_OWNER: &str = "storytold";
/// Creative Studio's GitHub account: the forks of the Craft apps live under it.
pub const CREATIVE_STUDIO_OWNER: &str = "7mz410";
/// ArtCraft on GitHub: every Craft App's source.
pub const GITHUB_URL: &str = "https://github.com/7mz410";
/// Creative Studio's social accounts: none yet, so no icons are shown.
pub const SOCIAL: [(&str, &str); 0] = [];

/// ArtCraft Launcher itself, described like a Craft App so it can update itself with the same
/// machinery (asset names `artcraft-launcher-<ver>-…`, bundle id `ai.storyteller.artcraft-launcher`,
/// `ArtCraft Launcher.app`). Not part of [`SUITE`].
pub const LAUNCHER: CraftApp = app(
    "artcraft-launcher",
    "Mansa",
    "Installs and updates the Creative Studio suite",
    AppGroup::New,
    [[0x0d, 0x2d, 0x3f], [0x0d, 0x2d, 0x3f], [0xf2, 0xf1, 0xee]],
    "7mz410/CreativeStudio-mansa",
);

/// The launcher's own repo, checked so it can say when a newer launcher is out.
pub const LAUNCHER_REPO: &str = "7mz410/CreativeStudio-mansa";
/// The launcher's own asset stem.
pub const LAUNCHER_ID: &str = "artcraft-launcher";

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn suite_has_seven_classic_and_five_new_apps_with_unique_ids() {
        assert_eq!(SUITE.iter().filter(|a| a.group == AppGroup::Classic).count(), 7);
        assert_eq!(SUITE.iter().filter(|a| a.group == AppGroup::New).count(), 5);
        let mut ids: Vec<_> = SUITE.iter().map(|a| a.id).collect();
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), SUITE.len());
        for a in &SUITE {
            assert_eq!(a.repo, format!("7mz410/CreativeStudio-{}", a.id));
            assert_eq!(a.name.to_lowercase(), a.id);
        }
    }

    #[test]
    fn websites_are_per_app() {
        assert_eq!(CraftApp::by_id("klmat").unwrap().website(), "https://klmat.studio.creative.ps");
    }
}
