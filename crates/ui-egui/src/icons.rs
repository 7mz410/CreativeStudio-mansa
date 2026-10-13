//! Bundled images: each Craft App's icon and the launcher's own.

use egui::ImageSource;

/// The 128 px icon of a Craft App.
pub fn app(id: &str) -> Option<ImageSource<'static>> {
    Some(match id {
        "tlween" => egui::include_image!("../../../assets/app-icons/tlween.png"),
        "rsm" => egui::include_image!("../../../assets/app-icons/rsm.png"),
        "tsweer" => egui::include_image!("../../../assets/app-icons/tsweer.png"),
        "lamasat" => egui::include_image!("../../../assets/app-icons/lamasat.png"),
        "sf7aat" => egui::include_image!("../../../assets/app-icons/sf7aat.png"),
        "effectat" => egui::include_image!("../../../assets/app-icons/effectat.png"),
        "trteeb" => egui::include_image!("../../../assets/app-icons/trteeb.png"),
        "klmat" => egui::include_image!("../../../assets/app-icons/klmat.png"),
        "3urood" => egui::include_image!("../../../assets/app-icons/3urood.png"),
        "arqam" => egui::include_image!("../../../assets/app-icons/arqam.png"),
        "muhandes" => egui::include_image!("../../../assets/app-icons/muhandes.png"),
        "7rrek" => egui::include_image!("../../../assets/app-icons/7rrek.png"),
        _ => return None,
    })
}

/// The Creative Studio star, for the header. Red on both backgrounds.
pub fn mansa_logo(_dark_background: bool) -> ImageSource<'static> {
    egui::include_image!("../../../docs/studio/mansa.svg")
}

/// The star is square.
pub const MANSA_LOGO_ASPECT: f32 = 1.0;

/// A social service's logo (Simple Icons, white, for tinting), by its name in `catalog::SOCIAL`.
pub fn social(service: &str) -> Option<ImageSource<'static>> {
    Some(match service {
        "X" => egui::include_image!("../../../assets/social/x.svg"),
        "YouTube" => egui::include_image!("../../../assets/social/youtube.svg"),
        "TikTok" => egui::include_image!("../../../assets/social/tiktok.svg"),
        "Instagram" => egui::include_image!("../../../assets/social/instagram.svg"),
        "LinkedIn" => egui::include_image!("../../../assets/social/linkedin.svg"),
        _ => return None,
    })
}

/// The Creative Studio star, for the dialogs' titles and the app icon's source.
pub fn mansa_mark() -> ImageSource<'static> {
    egui::include_image!("../../../docs/studio/mansa.svg")
}
