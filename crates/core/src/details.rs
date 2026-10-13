//! Hand-written product pages: what each app is, in the launcher's own words.
//!
//! The text is compiled in, so it works offline and on the web, and nothing from the network is
//! ever shown as page copy. What changes with every release (version, date, size, notes) comes
//! from the GitHub release instead. An app without an entry here simply has no details page.

/// One feature: a short name and a sentence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Feature {
    pub title: &'static str,
    pub text: &'static str,
}

/// The editorial half of an app's page.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct AppDetails {
    /// Matches [`crate::CraftApp::id`].
    pub id: &'static str,
    /// One line under the name.
    pub headline: &'static str,
    /// One or two paragraphs, separated by a blank line.
    pub about: &'static str,
    pub features: &'static [Feature],
    /// Where it runs, for display (`"macOS, Windows, Linux, web"`).
    pub platforms: &'static str,
}

const fn feature(title: &'static str, text: &'static str) -> Feature {
    Feature { title, text }
}

/// Where every Craft App runs (getartcraft.com lists the same for each of them).
const PLATFORMS: &str = "macOS, Windows, Linux, FreeBSD, web";

pub const DETAILS: [AppDetails; 13] = [
    AppDetails {
        id: "tlween",
        headline: "The image editor you already know how to use.",
        about: "Layers, masks, adjustment layers, layer styles, type, vectors and brushes in a native app \
            written in Rust. Open source, offline, and yours.\n\n\
            tlween keeps the menus, shortcuts and panels your hands already know, so there is \
            nothing to relearn.",
        features: &[
            feature("Native GPU canvas", "No Electron and no web view: a fast native compositor."),
            feature("Real layered files", "Opens and saves layered PSD and PSB documents."),
            feature("Non-destructive editing", "Adjustment layers, live-preview filters, smart objects and layer styles."),
            feature("On-device selection", "Subject and object selection and mask refinement run locally."),
            feature("Serious color", "8-, 16- and 32-bit RGB, CMYK, Lab and Grayscale, with ICC color management and soft proofing."),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "rsm",
        headline: "Vector illustration, reimagined in pure Rust.",
        about: "A native vector illustration app with the pen, panels and shortcuts illustrators expect. \
                Open source, and it also runs in a browser.",
        features: &[
            feature(
                "The workflow you know",
                "Pen, direct selection, shape booleans, snapping guides and an appearance panel, laid out the way you expect.",
            ),
            feature("Genuinely fast", "Drawings with thousands of shapes stay responsive on a retina display."),
            feature(
                "Booleans that don't fail",
                "Shape booleans work on the exact curves, so there is no \"cannot perform operation\", and undo is unlimited.",
            ),
            feature("Live, editable effects", "Blends, envelope distort, gradient mesh, repeats, glows and drop shadows all stay editable."),
            feature("Image tracing", "Turn raster images into clean vector paths, with presets for the common cases."),
            feature("Open formats", "Native JSON documents, SVG and PDF import and export, and PNG, JPEG and WebP output."),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "tsweer",
        headline: "Professional video editing, rebuilt from scratch.",
        about: "A native video editor written in pure Rust, with source and program monitors, a real timeline and \
                every trim mode.",
        features: &[
            feature("A timeline you already know", "Source and program monitors, three-point editing, every trim mode and J/K/L trimming."),
            feature("Frame-exact time", "An integer timebase keeps 23.976 and 29.97 drop-frame footage exact, with no drift."),
            feature("Its own codecs", "No FFmpeg inside: native support for H.264, HEVC, ProRes, VP9, AAC and Opus."),
            feature("Color grading", "Curves, color wheels, HSL secondaries, LUTs and HDR and log workflows, with scopes to check the result."),
            feature(
                "Effects and keyframes",
                "Video effects and transitions, keyframes with value and velocity graphs, and a linear-light GPU compositor.",
            ),
            feature("Broadcast-ready audio", "EBU R128 loudness meters, a mixer with automation and native audio effects."),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "lamasat",
        headline: "Your photos. Your pixels. Your machine.",
        about: "A fast photo library and non-destructive raw developer, written from scratch in pure Rust. \
                No account, no cloud, no telemetry and no subscription.",
        features: &[
            feature("Non-destructive by design", "A wide-gamut, 32-bit float pipeline that never touches your originals."),
            feature("Every control you reach for", "Light, color, effects, tone curve, a color mixer, color grading wheels and built-in presets."),
            feature("Precise masking", "Brush, gradient and range masks, plus sky, subject and background masks."),
            feature("Native raw decoding", "Pure-Rust decoders for the main camera raw formats: DNG, CR2, ARW, NEF, RAF, RW2, PEF and ORF."),
            feature("A library that keeps up", "Albums, ratings, flags and search, with grids that stay smooth on large catalogs."),
            feature("Instant feedback", "The GPU pipeline updates the preview as you move a slider, even on large raw files."),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "sf7aat",
        headline: "The open-source PDF workbench.",
        about: "Read, organize, combine, split and secure PDFs in a fast, native app written in Rust from the \
                ground up. No account, no telemetry and no cloud.",
        features: &[
            feature("Faithful rendering", "World scripts, vertical Japanese, color emoji and transparency."),
            feature("Search as you type", "Instant find, and text selection that follows the document's real reading order."),
            feature("Organize like cards", "Rotate, delete, insert and reorder pages, with deep undo behind every change."),
            feature("Combine, extract, split", "Assemble and break apart documents while keeping links, form fields, layers and bookmarks intact."),
            feature("Fearless saves", "Incremental, atomic saves with autosave and crash recovery."),
            feature("Built-in security", "Opens files from RC4 to AES-256 encryption and honors the author's permissions."),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "effectat",
        headline: "Motion graphics and visual effects, in pure Rust.",
        about: "A native compositor with compositions, layers, keyframes, effects, expressions, 3D cameras and \
                lights, and a render queue. Young, moving fast, and already usable.",
        features: &[
            feature(
                "Animate the way you know",
                "Familiar panels and keyframes (linear, bezier, hold and eased), with a graph editor for value and speed curves.",
            ),
            feature(
                "Layers of every kind",
                "Solids, shapes, text, footage, nested compositions, nulls and adjustment layers, with parenting, track mattes and blend modes.",
            ),
            feature(
                "A large effects library",
                "Blur, color correction, distortion, generators, keying, particles and simulation, stylize and time effects.",
            ),
            feature("Real 3D", "3D layers with cameras, depth of field, and lights that cast soft ray-traced shadows."),
            feature("Expressions", "JavaScript expressions with wiggle, loops and layer references, edited inline."),
            feature("Export anywhere", "H.264 and ProRes with alpha, PNG, TIFF and EXR sequences, GIF, and Lottie import and export."),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "trteeb",
        headline: "Page layout and publishing, rebuilt in pure Rust.",
        about: "A native layout app with spreads, parent pages, threaded stories, styles, swatches, text wrap and \
                a paragraph composer. No subscription, no license server and no telemetry.",
        features: &[
            feature("The layout tools you know", "Spreads and parent pages, frames and threaded stories, with paragraph and character styles."),
            feature(
                "Beautiful type",
                "A Knuth–Plass paragraph composer, dictionary hyphenation, optical margin alignment and baseline grids, with line breaks that match on screen and in PDF.",
            ),
            feature("Print-ready PDF", "Real selectable text with embedded fonts, CMYK and spot colors, bleed, crop marks and PDF/A output."),
            feature("Fast", "Multithreaded rendering, instant undo and cached text composition."),
            feature("Open formats", "A documented native format, PNG export and layout interchange import and export."),
            feature("Built for automation", "Every menu item, tool, panel and dialog can be driven over a JSON control channel and an MCP server."),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "klmat",
        headline: "The familiar word processor, native and offline.",
        about: "A word processor with the ribbon, shortcuts and behavior you already know, written in Rust. \
                Open source, and built from public specifications.",
        features: &[
            feature(
                "Familiar",
                "Ribbon tabs and groups, a live Styles gallery and the editing behavior you expect, from list continuation to Format Painter.",
            ),
            feature(
                "Your files",
                "Opens and saves .docx, and also .odt, .rtf, .html, .md, .tex and .txt. Opens older .doc files and exports PDF with real, selectable text.",
            ),
            feature("Fast", "Paragraph layout is cached, so typing stays responsive even in long documents."),
            feature("Private", "Spelling, grammar and everything else work offline."),
            feature("Everywhere", "One Rust codebase for macOS, Windows, Linux, BSD and the web."),
            feature(
                "Built for agents",
                "Every action is a command, driven from the ribbon, shortcuts, a command-line tool, a JSON control channel or an MCP server.",
            ),
        ],
        platforms: "macOS, Windows, Linux, BSD, web",
    },
    AppDetails {
        id: "3urood",
        headline: "Presentations, rebuilt in Rust.",
        about: "A presentation app with slide masters, shapes, animations and a presenter view, written in Rust. \
                In alpha, heading for beta.",
        features: &[
            feature("Make decks", "Slide masters and layouts with placeholders that inherit formatting, themes, sections, notes and comments."),
            feature(
                "Draw",
                "Preset shapes with adjust handles, connectors that stay glued to shapes, freeform tools, merged shapes, gradients, shadows and glow.",
            ),
            feature("Write", "In-place text editing with full font and paragraph formatting, bullets and numbering, columns and autofit."),
            feature("Add content", "Pictures with crop and corrections, styled tables, charts, audio and video."),
            feature(
                "Present",
                "Transitions including Morph, animations and motion paths, a full-screen slide show, presenter view and rehearsed timings.",
            ),
            feature("Exchange files", "Opens and saves PowerPoint .pptx and exports PDF (slides, notes pages and handouts), PNG and JPEG."),
        ],
        platforms: "macOS, Windows, Linux, BSD, web",
    },
    AppDetails {
        id: "arqam",
        headline: "Spreadsheets you already know how to use.",
        about: "A spreadsheet with the ribbon, formula bar, sheet tabs and shortcuts you know, on a calculation engine \
                written in Rust. No subscription, no license server and no telemetry.",
        features: &[
            feature(
                "Familiar",
                "The ribbon, formula bar, Name Box, fill handle, point-mode formulas, Format Cells, Paste Special, conditional formatting, tables and charts.",
            ),
            feature(
                "Compatible",
                "XLSX is the native format, with formulas, tables, conditional formats, charts, pivot tables and print settings round-tripping. CSV and TSV too.",
            ),
            feature(
                "Capable",
                "A dependency-graph calculation engine with dynamic arrays, LET and LAMBDA, structured table references, sort, AutoFilter and Flash Fill.",
            ),
            feature("Fast", "Copy-on-write workbooks, incremental recalculation and a grid that only lays out what is on screen."),
            feature(
                "Agent-native",
                "Every ribbon button, menu item, dialog and keystroke is a command, available over a JSON control channel, an MCP server or the CLI.",
            ),
            feature("Everywhere", "One Rust codebase for the desktop and the web."),
        ],
        platforms: "macOS, Windows, Linux, BSD, web",
    },
    AppDetails {
        id: "muhandes",
        headline: "Computer-aided drafting, in pure Rust.",
        about: "A drafting app with the command line, object snaps and workflow that drafters already know. Free, with no \
                account and no subscription. In early, fast development.",
        features: &[
            feature(
                "The workflow you know",
                "Type L, click two points, type @5<45 and press Enter. Prompts with clickable keywords, polar tracking, direct distance entry and grips work the way drafting habit expects.",
            ),
            feature(
                "Open files",
                "Reads and writes DXF natively (R12 through 2018) and opens and saves DWG (R13 through 2018) with the open-source acadrust library. Exports SVG and PNG.",
            ),
            feature("Fast and native", "Pure Rust and egui, with no Electron and no web view."),
            feature(
                "A full drafting toolset",
                "Draw, modify, dimension, hatch and block commands, layers with a properties manager, paper-space layouts and PDF plotting.",
            ),
            feature("Built for agents", "Every menu item, tool and prompt is a command, available over MCP, a JSON control channel or the CLI."),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "7rrek",
        headline: "Recording, editing and mixing, in Rust.",
        about: "A digital audio workstation that follows the workflow audio engineers already know: an Edit window for \
                arranging and editing, and a Mix window for balancing. Pre-alpha.",
        features: &[
            feature(
                "Editing",
                "Shuffle, Slip, Spot and Grid modes, Smart, Grabber, Trim and other tools, fades and crossfades, playlists with comping lanes and clip gain.",
            ),
            feature(
                "Mixing",
                "Faders and pans with automation, sends, busses, aux inputs, VCA masters, routing folders and automatic plugin delay compensation.",
            ),
            feature(
                "Automation",
                "Volume, pan, mute, send and every plugin parameter, with Write, Touch, Latch and Trim passes recorded live from the faders.",
            ),
            feature(
                "Plugins",
                "EQ, dynamics, reverbs, modulation effects, a subtractive synth and a drum synth, all original, and CLAP plugins load too.",
            ),
        ],
        platforms: PLATFORMS,
    },
    AppDetails {
        id: "aswat",
        headline: "Audio recording and editing, free and open source.",
        about: "A native audio editor for recording, arranging and mixing: multitrack editing, \
                automation, plugins and bounce to WAV, AIFF or FLAC. Built on SoundCraft.\n\n\
                aswat keeps the editing tools and shortcuts of a studio editor in one app that runs offline.",
        features: &[
            feature("Recording", "Punch in on a selection or on the fly, with pre-roll and loop recording."),
            feature("Editing", "Shuffle, Slip, Spot and Grid modes, with the usual selection, trim and pencil tools."),
            feature("Mixing", "Faders, pan and sends, busses and automation, with built-in EQ, compressor and limiter."),
            feature("Audio files", "WAV, AIFF, FLAC, MP3, Ogg Vorbis and AAC audio files, and bounce to 16, 24 or 32-bit float."),
        ],
        platforms: PLATFORMS,
    },
];

impl AppDetails {
    pub fn by_id(id: &str) -> Option<&'static AppDetails> {
        DETAILS.iter().find(|d| d.id == id)
    }

    pub fn paragraphs(&self) -> impl Iterator<Item = &'static str> {
        self.about.split("\n\n").map(str::trim).filter(|p| !p.is_empty())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{CraftApp, SUITE};

    #[test]
    fn every_page_belongs_to_a_suite_app_once() {
        let mut ids: Vec<_> = DETAILS.iter().map(|d| d.id).collect();
        for id in &ids {
            assert!(CraftApp::by_id(id).is_some(), "{id}");
        }
        ids.sort_unstable();
        ids.dedup();
        assert_eq!(ids.len(), DETAILS.len());
    }

    #[test]
    fn every_app_has_a_page() {
        for a in &SUITE {
            assert!(AppDetails::by_id(a.id).is_some(), "{}", a.id);
        }
    }

    #[test]
    fn pages_are_filled_in() {
        for d in &DETAILS {
            assert!(!d.headline.is_empty() && !d.features.is_empty() && d.paragraphs().next().is_some(), "{}", d.id);
        }
    }
}
