//! This module provides mappings between CSS properties and their abbreviations (short syntax).
//!
//! It allows for retrieving the full CSS property name given either the full name or its abbreviation,
//! as well as retrieving the abbreviation given the full name.
//!
//! The module also provides a list of all components (full names and abbreviations).

use once_cell::sync::Lazy;
use std::collections::HashMap;

/// A list of tuples containing CSS property full names and their abbreviations.
static PROPERTIES: &[(&str, &str)] = &[
    ("accent-color", "accent"),
    ("align-content", "algn-c"),
    ("align-items", "algn-i"),
    ("align-self", "algn-s"),
    ("align-tracks", "algn-t"),
    ("all", "all"),
    ("anchor-name", "anch-n"),
    ("anchor-scope", "anch-s"),
    ("animation", "anim"),
    ("animation-composition", "anim-comp"),
    ("animation-delay", "anim-dl"),
    ("animation-direction", "anim-dir"),
    ("animation-duration", "anim-dur"),
    ("animation-fill-mode", "anim-f-m"),
    ("animation-iteration-count", "anim-i-c"),
    ("animation-name", "anim-n"),
    ("animation-play-state", "anim-p-s"),
    ("animation-range", "anim-r"),
    ("animation-range-end", "anim-r-e"),
    ("animation-range-start", "anim-r-s"),
    ("animation-timeline", "anim-tl"),
    ("animation-timing-function", "anim-t-f"),
    ("appearance", "apr"),
    ("aspect-ratio", "asp-r"),
    ("backdrop-filter", "bdrop-f"),
    ("backface-visibility", "bf-vis"),
    ("background", "bg"),
    ("background-attachment", "bg-a"),
    ("background-blend-mode", "bg-b-m"),
    ("background-clip", "bg-clip"),
    ("background-color", "bg-c"),
    ("background-image", "bg-img"),
    ("background-origin", "bg-o"),
    ("background-position", "bg-pos"),
    ("background-position-x", "bg-pos-x"),
    ("background-position-y", "bg-pos-y"),
    ("background-repeat", "bg-rep"),
    ("background-size", "bg-sz"),
    ("block-size", "blk-sz"),
    ("border", "bd"),
    ("border-block", "bd-blk"),
    ("border-block-color", "bd-blk-c"),
    ("border-block-end", "bd-blk-e"),
    ("border-block-end-color", "bd-blk-e-c"),
    ("border-block-end-style", "bd-blk-e-s"),
    ("border-block-end-width", "bd-blk-e-w"),
    ("border-block-start", "bd-blk-s"),
    ("border-block-start-color", "bd-blk-s-c"),
    ("border-block-start-style", "bd-blk-s-s"),
    ("border-block-start-width", "bd-blk-s-w"),
    ("border-block-style", "bd-blk-st"),
    ("border-block-width", "bd-blk-w"),
    ("border-bottom", "bd-b"),
    ("border-bottom-color", "bd-b-c"),
    ("border-bottom-left-radius", "bd-b-l-rad"),
    ("border-bottom-right-radius", "bd-b-r-rad"),
    ("border-bottom-style", "bd-b-st"),
    ("border-bottom-width", "bd-b-w"),
    ("border-collapse", "bd-clps"),
    ("border-color", "bd-c"),
    ("border-end-end-radius", "bd-e-e-rad"),
    ("border-end-start-radius", "bd-e-s-rad"),
    ("border-image", "bd-img"),
    ("border-image-outset", "bd-img-o"),
    ("border-image-repeat", "bd-img-rep"),
    ("border-image-slice", "bd-img-sl"),
    ("border-image-source", "bd-img-src"),
    ("border-image-width", "bd-img-w"),
    ("border-inline", "bd-inl"),
    ("border-inline-color", "bd-inl-c"),
    ("border-inline-end", "bd-inl-e"),
    ("border-inline-end-color", "bd-inl-e-c"),
    ("border-inline-end-style", "bd-inl-e-s"),
    ("border-inline-end-width", "bd-inl-e-w"),
    ("border-inline-start", "bd-inl-s"),
    ("border-inline-start-color", "bd-inl-s-c"),
    ("border-inline-start-style", "bd-inl-s-s"),
    ("border-inline-start-width", "bd-inl-s-w"),
    ("border-inline-style", "bd-inl-st"),
    ("border-inline-width", "bd-inl-w"),
    ("border-left", "bd-l"),
    ("border-left-color", "bd-l-c"),
    ("border-left-style", "bd-l-st"),
    ("border-left-width", "bd-l-w"),
    ("border-radius", "bd-rad"),
    ("border-right", "bd-r"),
    ("border-right-color", "bd-r-c"),
    ("border-right-style", "bd-r-st"),
    ("border-right-width", "bd-r-w"),
    ("border-spacing", "bd-sp"),
    ("border-start-end-radius", "bd-s-e-rad"),
    ("border-start-start-radius", "bd-s-s-rad"),
    ("border-style", "bd-st"),
    ("border-top", "bd-t"),
    ("border-top-color", "bd-t-c"),
    ("border-top-left-radius", "bd-t-l-rad"),
    ("border-top-right-radius", "bd-t-r-rad"),
    ("border-top-style", "bd-t-st"),
    ("border-top-width", "bd-t-w"),
    ("border-width", "bd-w"),
    ("bottom", "bot"),
    ("box-align", "box-algn"),
    ("box-decoration-break", "box-d-b"),
    ("box-direction", "box-dir"),
    ("box-flex", "box-flex"),
    ("box-flex-group", "box-flex-g"),
    ("box-lines", "box-lns"),
    ("box-ordinal-group", "box-ord-g"),
    ("box-orient", "box-orient"),
    ("box-pack", "box-pack"),
    ("box-shadow", "box-shd"),
    ("box-sizing", "box-sz"),
    ("break-after", "brk-a"),
    ("break-before", "brk-b"),
    ("break-inside", "brk-i"),
    ("caption-side", "cap-s"),
    ("caret", "caret"),
    ("caret-color", "caret-c"),
    ("caret-shape", "caret-s"),
    ("clear", "clr"),
    ("clip", "clip"),
    ("clip-path", "clip-p"),
    ("color", "c"),
    ("color-scheme", "c-sch"),
    ("column-count", "col-c"),
    ("column-fill", "col-f"),
    ("column-gap", "col-g"),
    ("column-rule", "col-r"),
    ("column-rule-color", "col-r-c"),
    ("column-rule-style", "col-r-s"),
    ("column-rule-width", "col-r-w"),
    ("column-span", "col-s"),
    ("column-width", "col-w"),
    ("columns", "cols"),
    ("contain", "ctn"),
    ("contain-intrinsic-block-size", "ctn-i-blk-sz"),
    ("contain-intrinsic-height", "ctn-i-h"),
    ("contain-intrinsic-inline-size", "ctn-i-inl-sz"),
    ("contain-intrinsic-size", "ctn-i-sz"),
    ("contain-intrinsic-width", "ctn-i-w"),
    ("container", "ctnr"),
    ("container-name", "ctnr-n"),
    ("container-type", "ctnr-t"),
    ("content", "cnt"),
    ("content-visibility", "cnt-vis"),
    ("counter-increment", "ctr-i"),
    ("counter-reset", "ctr-r"),
    ("counter-set", "ctr-s"),
    ("cursor", "cur"),
    ("direction", "dir"),
    ("display", "disp"),
    ("empty-cells", "empty-c"),
    ("field-sizing", "field-sz"),
    ("filter", "fltr"),
    ("flex", "flex"),
    ("flex-basis", "flex-b"),
    ("flex-direction", "flex-dir"),
    ("flex-flow", "flex-flow"),
    ("flex-grow", "flex-g"),
    ("flex-shrink", "flex-s"),
    ("flex-wrap", "flex-w"),
    ("float", "flt"),
    ("font", "font"),
    ("font-family", "font-f"),
    ("font-feature-settings", "font-f-s"),
    ("font-kerning", "font-k"),
    ("font-language-override", "font-l-o"),
    ("font-optical-sizing", "font-o-s"),
    ("font-palette", "font-p"),
    ("font-size", "font-sz"),
    ("font-size-adjust", "font-sz-a"),
    ("font-smooth", "font-smooth"),
    ("font-stretch", "font-str"),
    ("font-style", "font-st"),
    ("font-synthesis", "font-syn"),
    ("font-synthesis-position", "font-syn-p"),
    ("font-synthesis-small-caps", "font-syn-s-c"),
    ("font-synthesis-style", "font-syn-st"),
    ("font-synthesis-weight", "font-syn-w"),
    ("font-variant", "font-v"),
    ("font-variant-alternates", "font-v-alt"),
    ("font-variant-caps", "font-v-cap"),
    ("font-variant-east-asian", "font-v-e-a"),
    ("font-variant-emoji", "font-v-e"),
    ("font-variant-ligatures", "font-v-lig"),
    ("font-variant-numeric", "font-v-num"),
    ("font-variant-position", "font-v-pos"),
    ("font-variation-settings", "font-var-s"),
    ("font-weight", "font-w"),
    ("forced-color-adjust", "force-c-a"),
    ("gap", "gap"),
    ("grid", "grid"),
    ("grid-area", "grid-a"),
    ("grid-auto-columns", "grid-a-c"),
    ("grid-auto-flow", "grid-a-f"),
    ("grid-auto-rows", "grid-a-r"),
    ("grid-column", "grid-c"),
    ("grid-column-end", "grid-c-e"),
    ("grid-column-gap", "grid-c-g"),
    ("grid-column-start", "grid-c-s"),
    ("grid-gap", "grid-g"),
    ("grid-row", "grid-r"),
    ("grid-row-end", "grid-r-e"),
    ("grid-row-gap", "grid-r-g"),
    ("grid-row-start", "grid-r-s"),
    ("grid-template", "grid-t"),
    ("grid-template-areas", "grid-t-a"),
    ("grid-template-columns", "grid-t-c"),
    ("grid-template-rows", "grid-t-r"),
    ("hanging-punctuation", "hang-p"),
    ("height", "h"),
    ("hyphenate-character", "hyph-c"),
    ("hyphenate-limit-chars", "hyph-l-c"),
    ("hyphens", "hyph"),
    ("image-orientation", "img-o"),
    ("image-rendering", "img-rnd"),
    ("image-resolution", "img-res"),
    ("ime-mode", "ime-mode"),
    ("initial-letter", "init-l"),
    ("initial-letter-align", "init-la"),
    ("inline-size", "inl-sz"),
    ("input-security", "inp-sec"),
    ("inset", "inset"),
    ("inset-area", "inset-a"),
    ("inset-block", "inset-b"),
    ("inset-block-end", "inset-b-e"),
    ("inset-block-start", "inset-b-s"),
    ("inset-inline", "inset-i"),
    ("inset-inline-end", "inset-i-e"),
    ("inset-inline-start", "inset-i-s"),
    ("isolation", "iso"),
    ("justify-content", "just-c"),
    ("justify-items", "just-i"),
    ("justify-self", "just-s"),
    ("justify-tracks", "just-t"),
    ("left", "l"),
    ("letter-spacing", "let-sp"),
    ("line-break", "ln-brk"),
    ("line-clamp", "ln-clmp"),
    ("line-height", "ln-h"),
    ("line-height-step", "ln-h-step"),
    ("list-style", "lst-s"),
    ("list-style-image", "lst-img"),
    ("list-style-position", "lst-pos"),
    ("list-style-type", "lst-type"),
    ("margin", "m"),
    ("margin-block", "m-blk"),
    ("margin-block-end", "m-blk-e"),
    ("margin-block-start", "m-blk-s"),
    ("margin-bottom", "m-b"),
    ("margin-inline", "m-inl"),
    ("margin-inline-end", "m-inl-e"),
    ("margin-inline-start", "m-inl-s"),
    ("margin-left", "m-l"),
    ("margin-right", "m-r"),
    ("margin-top", "m-t"),
    ("margin-trim", "m-trim"),
    ("mask", "mask"),
    ("mask-border", "mask-bd"),
    ("mask-border-mode", "mask-bd-m"),
    ("mask-border-outset", "mask-bd-o"),
    ("mask-border-repeat", "mask-bd-rep"),
    ("mask-border-slice", "mask-bd-sl"),
    ("mask-border-source", "mask-bd-src"),
    ("mask-border-width", "mask-bd-w"),
    ("mask-clip", "mask-clip"),
    ("mask-composite", "mask-comp"),
    ("mask-image", "mask-img"),
    ("mask-mode", "mask-m"),
    ("mask-origin", "mask-o"),
    ("mask-position", "mask-pos"),
    ("mask-repeat", "mask-rep"),
    ("mask-size", "mask-sz"),
    ("mask-type", "mask-t"),
    ("masonry-auto-flow", "mason-af"),
    ("math-depth", "math-d"),
    ("math-shift", "math-s"),
    ("math-style", "math-st"),
    ("max-block-size", "max-blk-sz"),
    ("max-height", "max-h"),
    ("max-inline-size", "max-i-sz"),
    ("max-lines", "max-ln"),
    ("max-width", "max-w"),
    ("min-block-size", "min-blk-sz"),
    ("min-height", "min-h"),
    ("min-inline-size", "min-i-sz"),
    ("min-width", "min-w"),
    ("mix-blend-mode", "mix-b-m"),
    ("object-fit", "obj-fit"),
    ("object-position", "obj-pos"),
    ("offset", "ofs"),
    ("offset-anchor", "ofs-a"),
    ("offset-distance", "ofs-d"),
    ("offset-path", "ofs-p"),
    ("offset-position", "ofs-pos"),
    ("offset-rotate", "ofs-r"),
    ("opacity", "op"),
    ("order", "order"),
    ("orphans", "orph"),
    ("outline", "ol"),
    ("outline-color", "ol-c"),
    ("outline-offset", "ol-o"),
    ("outline-style", "ol-s"),
    ("outline-width", "ol-w"),
    ("overflow", "ovf"),
    ("overflow-anchor", "ovf-a"),
    ("overflow-block", "ovf-b"),
    ("overflow-clip-box", "ovf-c-box"),
    ("overflow-clip-margin", "ovf-c-m"),
    ("overflow-inline", "ovf-i"),
    ("overflow-wrap", "ovf-w"),
    ("overflow-x", "ovf-x"),
    ("overflow-y", "ovf-y"),
    ("overlay", "overlay"),
    ("overscroll-behavior", "ovsc-b"),
    ("overscroll-behavior-block", "ovsc-b-blk"),
    ("overscroll-behavior-inline", "ovsc-b-inl"),
    ("overscroll-behavior-x", "ovsc-b-x"),
    ("overscroll-behavior-y", "ovsc-b-y"),
    ("padding", "p"),
    ("padding-block", "p-blk"),
    ("padding-block-end", "p-blk-e"),
    ("padding-block-start", "p-blk-s"),
    ("padding-bottom", "pb"),
    ("padding-inline", "p-inl"),
    ("padding-inline-end", "p-inl-e"),
    ("padding-inline-start", "p-inl-s"),
    ("padding-left", "pl"),
    ("padding-right", "pr"),
    ("padding-top", "pt"),
    ("page", "page"),
    ("page-break-after", "pg-brk-a"),
    ("page-break-before", "pg-brk-b"),
    ("page-break-inside", "pg-brk-i"),
    ("paint-order", "paint-o"),
    ("perspective", "persp"),
    ("perspective-origin", "persp-o"),
    ("place-content", "place-c"),
    ("place-items", "place-i"),
    ("place-self", "place-s"),
    ("pointer-events", "ptr-e"),
    ("position", "pos"),
    ("position-anchor", "pos-anch"),
    ("position-try", "pos-try"),
    ("position-try-options", "pos-try-opt"),
    ("position-try-order", "pos-try-ord"),
    ("position-visibility", "pos-vis"),
    ("print-color-adjust", "print-c-a"),
    ("quotes", "q"),
    ("resize", "rsz"),
    ("right", "r"),
    ("rotate", "rot"),
    ("row-gap", "row-g"),
    ("ruby-align", "ruby-a"),
    ("ruby-merge", "ruby-m"),
    ("ruby-position", "ruby-p"),
    ("scale", "scl"),
    ("scroll-behavior", "scrl-b"),
    ("scroll-margin", "scrl-m"),
    ("scroll-margin-block", "scrl-m-blk"),
    ("scroll-margin-block-end", "scrl-m-blk-e"),
    ("scroll-margin-block-start", "scrl-m-blk-s"),
    ("scroll-margin-bottom", "scrl-m-b"),
    ("scroll-margin-inline", "scrl-m-inl"),
    ("scroll-margin-inline-end", "scrl-m-inl-e"),
    ("scroll-margin-inline-start", "scrl-m-inl-s"),
    ("scroll-margin-left", "scrl-m-l"),
    ("scroll-margin-right", "scrl-m-r"),
    ("scroll-margin-top", "scrl-m-t"),
    ("scroll-padding", "scrl-p"),
    ("scroll-padding-block", "scrl-p-blk"),
    ("scroll-padding-block-end", "scrl-p-blk-e"),
    ("scroll-padding-block-start", "scrl-p-blk-s"),
    ("scroll-padding-bottom", "scrl-p-b"),
    ("scroll-padding-inline", "scrl-p-inl"),
    ("scroll-padding-inline-end", "scrl-p-inl-e"),
    ("scroll-padding-inline-start", "scrl-p-inl-s"),
    ("scroll-padding-left", "scrl-p-l"),
    ("scroll-padding-right", "scrl-p-r"),
    ("scroll-padding-top", "scrl-p-t"),
    ("scroll-snap-align", "scrl-s-a"),
    ("scroll-snap-coordinate", "scrl-s-coord"),
    ("scroll-snap-destination", "scrl-s-dest"),
    ("scroll-snap-points-x", "scrl-s-p-x"),
    ("scroll-snap-points-y", "scrl-s-p-y"),
    ("scroll-snap-stop", "scrl-s-stop"),
    ("scroll-snap-type", "scrl-s-type"),
    ("scroll-timeline", "scrl-tl"),
    ("scroll-timeline-axis", "scrl-tl-a"),
    ("scroll-timeline-name", "scrl-tl-n"),
    ("scrollbar-color", "scrlbar-c"),
    ("scrollbar-gutter", "scrlbar-g"),
    ("scrollbar-width", "scrlbar-w"),
    ("shape-image-threshold", "shp-i-t"),
    ("shape-margin", "shp-m"),
    ("shape-outside", "shp-o"),
    ("tab-size", "tab-sz"),
    ("table-layout", "tbl-l"),
    ("text-align", "txt-a"),
    ("text-align-last", "txt-a-last"),
    ("text-combine-upright", "txt-comb-u"),
    ("text-decoration", "txt-d"),
    ("text-decoration-color", "txt-d-c"),
    ("text-decoration-line", "txt-d-l"),
    ("text-decoration-skip", "txt-d-skip"),
    ("text-decoration-skip-ink", "txt-d-skip-ink"),
    ("text-decoration-style", "txt-d-st"),
    ("text-decoration-thickness", "txt-d-thick"),
    ("text-emphasis", "txt-emph"),
    ("text-emphasis-color", "txt-emph-c"),
    ("text-emphasis-position", "txt-emph-p"),
    ("text-emphasis-style", "txt-emph-s"),
    ("text-indent", "txt-i"),
    ("text-justify", "txt-just"),
    ("text-orientation", "txt-or"),
    ("text-overflow", "txt-ovf"),
    ("text-rendering", "txt-rnd"),
    ("text-shadow", "txt-shd"),
    ("text-size-adjust", "txt-sa"),
    ("text-spacing-trim", "txt-spc-t"),
    ("text-transform", "txt-tf"),
    ("text-underline-offset", "txt-u-offset"),
    ("text-underline-position", "txt-u-pos"),
    ("text-wrap", "txt-wrap"),
    ("text-wrap-mode", "txt-wrap-m"),
    ("text-wrap-style", "txt-wrap-s"),
    ("timeline-scope", "tl-scope"),
    ("top", "t"),
    ("touch-action", "tch-a"),
    ("transform", "tf"),
    ("transform-box", "tf-box"),
    ("transform-origin", "tf-o"),
    ("transform-style", "tf-s"),
    ("transition", "trs"),
    ("transition-behavior", "trs-b"),
    ("transition-delay", "trs-d"),
    ("transition-duration", "trs-dur"),
    ("transition-property", "trs-p"),
    ("transition-timing-function", "trs-t-f"),
    ("translate", "trl"),
    ("unicode-bidi", "uni-bidi"),
    ("user-select", "usr-sel"),
    ("vertical-align", "v-align"),
    ("view-timeline", "view-tl"),
    ("view-timeline-axis", "view-tl-a"),
    ("view-timeline-inset", "view-tl-i"),
    ("view-timeline-name", "view-tl-n"),
    ("view-transition-name", "view-trs-n"),
    ("visibility", "vis"),
    ("white-space", "w-s"),
    ("white-space-collapse", "w-s-coll"),
    ("widows", "widows"),
    ("width", "w"),
    ("will-change", "will-c"),
    ("word-break", "wrd-brk"),
    ("word-spacing", "wrd-sp"),
    ("word-wrap", "wrd-wrap"),
    ("writing-mode", "w-mode"),
    ("z-index", "z"),
    ("zoom", "zoom"),
    // --- CUSTOM ---
    // generate built-in animation with all predefined rules
    ("g-anim", "g-anim"),
];

/// A HashMap mapping both full names and abbreviations to the full CSS property names.
static COMPONENTS_MAP: Lazy<HashMap<&'static str, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for &(full_name, abbreviation) in PROPERTIES.iter() {
        m.insert(full_name, full_name);
        m.insert(abbreviation, full_name);
    }
    m
});

/// A HashMap mapping full CSS property names to their abbreviations.
static FULL_TO_SHORT_MAP: Lazy<HashMap<&'static str, &'static str>> =
    Lazy::new(|| PROPERTIES.iter().cloned().collect());

/// A list of all component strings (full names and abbreviations).
static ALL_COMPONENTS: Lazy<Vec<&'static str>> =
    Lazy::new(|| COMPONENTS_MAP.keys().cloned().collect());

/// Gets the full CSS property name corresponding to the given component string.
///
/// The component string can be either the full CSS property name or its abbreviation.
///
/// # Arguments
///
/// * `component_str` - The component string to look up.
///
/// # Returns
///
/// * `Option<&'static str>` - The full CSS property name if found, or `None` if not found.
pub fn get_css_property(component_str: &str) -> Option<&'static str> {
    COMPONENTS_MAP.get(component_str).cloned()
}

/// Gets the abbreviation for a given full CSS property name.
///
/// # Arguments
///
/// * `full_name` - The full CSS property name.
///
/// # Returns
///
/// * `Option<&'static str>` - The abbreviation if found, or `None` if not found.
pub fn get_shorten_component(full_name: &str) -> Option<&'static str> {
    FULL_TO_SHORT_MAP.get(full_name).cloned()
}

/// Gets a list of all components (both full CSS property names and their abbreviations).
///
/// # Returns
///
/// * `&'static [&'static str]` - A slice containing all component strings.
pub fn get_all_components() -> &'static [&'static str] {
    ALL_COMPONENTS.as_slice()
}

pub fn get_all_components_map() -> HashMap<&'static str, &'static str> {
    PROPERTIES.iter().cloned().collect()
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use super::*;

    #[test]
    fn test_get_css_property_full_name() {
        assert_eq!(get_css_property("accent-color"), Some("accent-color"));
        assert_eq!(get_css_property("align-items"), Some("align-items"));
    }

    #[test]
    fn test_get_css_property_abbreviation() {
        assert_eq!(get_css_property("accent"), Some("accent-color"));
        assert_eq!(get_css_property("algn-i"), Some("align-items"));
    }

    #[test]
    fn test_get_css_property_unknown() {
        assert_eq!(get_css_property("unknown"), None);
        assert_eq!(get_css_property("xyz"), None);
    }

    #[test]
    fn test_get_shorten_component() {
        assert_eq!(get_shorten_component("accent-color"), Some("accent"));
        assert_eq!(get_shorten_component("align-items"), Some("algn-i"));
    }

    #[test]
    fn test_get_shorten_component_unknown() {
        assert_eq!(get_shorten_component("unknown-property"), None);
        assert_eq!(get_shorten_component("acc"), None); // "acc" is an abbreviation, not a full name
    }

    #[test]
    fn test_get_all_components() {
        let components = get_all_components();
        assert!(components.contains(&"accent-color"));
        assert!(components.contains(&"accent"));
        assert!(components.contains(&"align-items"));
        assert!(components.contains(&"algn-i"));
        assert!(!components.contains(&"unknown"));
    }

    #[test]
    fn test_get_css_property_case_sensitive() {
        // Ensure that the lookup is case-sensitive
        assert_eq!(get_css_property("Acc"), None);
        assert_eq!(get_css_property("ACC"), None);
    }

    #[test]
    fn test_get_shorten_component_case_sensitive() {
        assert_eq!(get_shorten_component("Accent-Color"), None);
        assert_eq!(get_shorten_component("ACCENT-COLOR"), None);
    }

    #[test]
    fn test_unique_shortening() {
        let mut seen = HashSet::new();
        for &(_, abbreviation) in PROPERTIES.iter() {
            assert!(
                seen.insert(abbreviation),
                "Duplicate abbreviation found: {abbreviation}"
            );
        }
    }
}
