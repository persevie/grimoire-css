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
    ("accent-color", "acc"),
    ("align-content", "ac"),
    ("align-items", "ai"),
    ("align-self", "as"),
    ("align-tracks", "atr"),
    ("all", "all"),
    ("anchor-name", "anc-n"),
    ("anchor-scope", "anc-s"),
    ("animation", "anim"),
    ("animation-composition", "anim-comp"),
    ("animation-delay", "anim-d"),
    ("animation-direction", "anim-dir"),
    ("animation-duration", "anim-du"),
    ("animation-fill-mode", "anim-fm"),
    ("animation-iteration-count", "anim-ic"),
    ("animation-name", "anim-n"),
    ("animation-play-state", "anim-ps"),
    ("animation-range", "anim-r"),
    ("animation-range-end", "anim-re"),
    ("animation-range-start", "anim-rs"),
    ("animation-timeline", "at"),
    ("animation-timing-function", "atf"),
    ("animationType", "atype"),
    ("appearance", "app"),
    ("appliesto", "applies"),
    ("aspect-ratio", "ar"),
    ("azimuth", "az"),
    ("backdrop-filter", "bf"),
    ("backface-visibility", "bvis"),
    ("background", "bg"),
    ("background-attachment", "bga"),
    ("background-blend-mode", "bgblm"),
    ("background-clip", "bg-cl"),
    ("background-color", "bgc"),
    ("background-image", "bgi"),
    ("background-origin", "bgo"),
    ("background-position", "bgp"),
    ("background-position-x", "bgpx"),
    ("background-position-y", "bgpy"),
    ("background-repeat", "bgr"),
    ("background-size", "bgsz"),
    ("block-size", "bsz"),
    ("border", "b"),
    ("border-block", "bb"),
    ("border-block-color", "bbc"),
    ("border-block-end", "bbe"),
    ("border-block-end-color", "bbec"),
    ("border-block-end-style", "bbes"),
    ("border-block-end-width", "bbew"),
    ("border-block-start", "bbs"),
    ("border-block-start-color", "bbsc"),
    ("border-block-start-style", "bbss"),
    ("border-block-start-width", "bbsw"),
    ("border-block-style", "bbs"),
    ("border-block-width", "bbw"),
    ("border-bottom", "bb"),
    ("border-bottom-color", "bbc"),
    ("border-bottom-left-radius", "bblr"),
    ("border-bottom-right-radius", "bbrr"),
    ("border-bottom-style", "bbs"),
    ("border-bottom-width", "bbw"),
    ("border-collapse", "bcoll"),
    ("border-color", "bc"),
    ("border-end-end-radius", "beer"),
    ("border-end-start-radius", "besr"),
    ("border-image", "bi"),
    ("border-image-outset", "bio"),
    ("border-image-repeat", "bir"),
    ("border-image-slice", "bis"),
    ("border-image-source", "bisrc"),
    ("border-image-width", "biw"),
    ("border-inline", "bli"),
    ("border-inline-color", "blic"),
    ("border-inline-end", "blie"),
    ("border-inline-end-color", "bliec"),
    ("border-inline-end-style", "blies"),
    ("border-inline-end-width", "bliew"),
    ("border-inline-start", "blis"),
    ("border-inline-start-color", "blisc"),
    ("border-inline-start-style", "bliss"),
    ("border-inline-start-width", "blisw"),
    ("border-inline-style", "blis"),
    ("border-inline-width", "bliw"),
    ("border-left", "bl"),
    ("border-left-color", "blc"),
    ("border-left-style", "bls"),
    ("border-left-width", "blw"),
    ("border-radius", "br"),
    ("border-right", "brt"),
    ("border-right-color", "brc"),
    ("border-right-style", "brs"),
    ("border-right-width", "brw"),
    ("border-spacing", "bsp"),
    ("border-start-end-radius", "bser"),
    ("border-start-start-radius", "bssr"),
    ("border-style", "bst"),
    ("border-top", "bt"),
    ("border-top-color", "btc"),
    ("border-top-left-radius", "btlr"),
    ("border-top-right-radius", "btrr"),
    ("border-top-style", "bts"),
    ("border-top-width", "btw"),
    ("border-width", "bw"),
    ("bottom", "btm"),
    ("box-align", "bxa"),
    ("box-decoration-break", "bxdb"),
    ("box-direction", "bxd"),
    ("box-flex", "bxf"),
    ("box-flex-group", "bxfg"),
    ("box-lines", "bxl"),
    ("box-ordinal-group", "bxog"),
    ("box-orient", "bxo"),
    ("box-pack", "bxp"),
    ("box-shadow", "bxsh"),
    ("box-sizing", "bxs"),
    ("break-after", "ba"),
    ("break-before", "bb"),
    ("break-inside", "bi"),
    ("caption-side", "cs"),
    ("caret", "crt"),
    ("caret-color", "cc"),
    ("caret-shape", "cs"),
    ("clear", "clr"),
    ("clip", "clp"),
    ("clip-path", "clpp"),
    ("color", "c"),
    ("color-scheme", "csch"),
    ("column-count", "ccnt"),
    ("column-fill", "cf"),
    ("column-gap", "cg"),
    ("column-rule", "cr"),
    ("column-rule-color", "crc"),
    ("column-rule-style", "crs"),
    ("column-rule-width", "crw"),
    ("column-span", "csn"),
    ("column-width", "cw"),
    ("columns", "cols"),
    ("computed", "cmp"),
    ("contain", "cntn"),
    ("contain-intrinsic-block-size", "cibs"),
    ("contain-intrinsic-height", "cih"),
    ("contain-intrinsic-inline-size", "ciis"),
    ("contain-intrinsic-size", "cis"),
    ("contain-intrinsic-width", "ciw"),
    ("container", "ctr"),
    ("container-name", "ctrn"),
    ("container-type", "ctrt"),
    ("content", "cnt"),
    ("content-visibility", "cntv"),
    ("counter-increment", "cinc"),
    ("counter-reset", "crst"),
    ("counter-set", "cset"),
    ("cursor", "cur"),
    ("direction", "dir"),
    ("display", "d"),
    ("empty-cells", "ec"),
    ("field-sizing", "fsz"),
    ("filter", "flt"),
    ("flex", "flx"),
    ("flex-basis", "flxb"),
    ("flex-direction", "flex-dir"),
    ("flex-flow", "flex-fl"),
    ("flex-grow", "flex-gr"),
    ("flex-shrink", "flex-sh"),
    ("flex-wrap", "flex-wr"),
    ("float", "flt"),
    ("font", "fnt"),
    ("font-family", "ff"),
    ("font-feature-settings", "ffs"),
    ("font-kerning", "fk"),
    ("font-language-override", "flo"),
    ("font-optical-sizing", "fos"),
    ("font-palette", "fp"),
    ("font-size", "fs"),
    ("font-size-adjust", "fsa"),
    ("font-smooth", "fsm"),
    ("font-stretch", "fstr"),
    ("font-style", "fsty"),
    ("font-synthesis", "fsyn"),
    ("font-synthesis-position", "fsynp"),
    ("font-synthesis-small-caps", "fssc"),
    ("font-synthesis-style", "fss"),
    ("font-synthesis-weight", "fsw"),
    ("font-variant", "fv"),
    ("font-variant-alternates", "fva"),
    ("font-variant-caps", "fvc"),
    ("font-variant-east-asian", "fvea"),
    ("font-variant-emoji", "fve"),
    ("font-variant-ligatures", "fvl"),
    ("font-variant-numeric", "fvn"),
    ("font-variant-position", "fvp"),
    ("font-variation-settings", "fvs"),
    ("font-weight", "fw"),
    ("forced-color-adjust", "fca"),
    ("gap", "g"),
    ("grid", "gr"),
    ("grid-area", "gra"),
    ("grid-auto-columns", "grac"),
    ("grid-auto-flow", "graf"),
    ("grid-auto-rows", "grar"),
    ("grid-column", "gc"),
    ("grid-column-end", "gce"),
    ("grid-column-gap", "gcg"),
    ("grid-column-start", "gcs"),
    ("grid-gap", "gg"),
    ("grid-row", "gr"),
    ("grid-row-end", "gre"),
    ("grid-row-gap", "grg"),
    ("grid-row-start", "grs"),
    ("grid-template", "gt"),
    ("grid-template-areas", "gta"),
    ("grid-template-columns", "gtc"),
    ("grid-template-rows", "gtr"),
    ("groups", "grps"),
    ("hanging-punctuation", "hp"),
    ("height", "h"),
    ("hyphenate-character", "hc"),
    ("hyphenate-limit-chars", "hlc"),
    ("hyphens", "hy"),
    ("image-orientation", "io"),
    ("image-rendering", "imgr"),
    ("image-resolution", "imgres"),
    ("ime-mode", "im"),
    ("inherited", "inh"),
    ("initial", "init"),
    ("initial-letter", "initl"),
    ("initial-letter-align", "initla"),
    ("inline-size", "insz"),
    ("input-security", "inps"),
    ("inset", "in"),
    ("inset-area", "ina"),
    ("inset-block", "inb"),
    ("inset-block-end", "inbe"),
    ("inset-block-start", "inbs"),
    ("inset-inline", "ini"),
    ("inset-inline-end", "inie"),
    ("inset-inline-start", "inis"),
    ("isolation", "iso"),
    ("justify-content", "jc"),
    ("justify-items", "ji"),
    ("justify-self", "js"),
    ("justify-tracks", "jt"),
    ("left", "l"),
    ("letter-spacing", "ls"),
    ("line-break", "lb"),
    ("line-clamp", "lc"),
    ("line-height", "lh"),
    ("line-height-step", "lhs"),
    ("list-style", "ls"),
    ("list-style-image", "lsi"),
    ("list-style-position", "lsp"),
    ("list-style-type", "lst"),
    ("margin", "m"),
    ("margin-block", "mb"),
    ("margin-block-end", "mbe"),
    ("margin-block-start", "mbs"),
    ("margin-bottom", "mb"),
    ("margin-inline", "mi"),
    ("margin-inline-end", "mie"),
    ("margin-inline-start", "mis"),
    ("margin-left", "ml"),
    ("margin-right", "mr"),
    ("margin-top", "mt"),
    ("margin-trim", "mtrim"),
    ("mask", "mask"),
    ("mask-border", "mask-b"),
    ("mask-border-mode", "mask-bm"),
    ("mask-border-outset", "mask-bo"),
    ("mask-border-repeat", "mask-br"),
    ("mask-border-slice", "mask-bs"),
    ("mask-border-source", "mask-bsou"),
    ("mask-border-width", "mask-bw"),
    ("mask-clip", "mask-c"),
    ("mask-composite", "mask-comp"),
    ("mask-image", "mask-i"),
    ("mask-mode", "mask-m"),
    ("mask-origin", "mask-o"),
    ("mask-position", "mask-pos"),
    ("mask-repeat", "mask-r"),
    ("mask-size", "mask-sz"),
    ("mask-type", "mask-t"),
    ("masonry-auto-flow", "mas-af"),
    ("math-depth", "math-d"),
    ("math-shift", "math-s"),
    ("math-style", "math-st"),
    ("max-block-size", "max-bs"),
    ("max-height", "max-h"),
    ("max-inline-size", "max-is"),
    ("max-lines", "max-l"),
    ("max-width", "max-w"),
    ("mdn_url", "mdn-u"),
    ("media", "med"),
    ("min-block-size", "min-bs"),
    ("min-height", "min-h"),
    ("min-inline-size", "min-is"),
    ("min-width", "min-w"),
    ("mix-blend-mode", "mbm"),
    ("object-fit", "obj-fit"),
    ("object-position", "obj-pos"),
    ("offset", "off"),
    ("offset-anchor", "ofa"),
    ("offset-distance", "ofd"),
    ("offset-path", "ofp"),
    ("offset-position", "ofpos"),
    ("offset-rotate", "ofr"),
    ("opacity", "op"),
    ("order", "ord"),
    ("orphans", "orphan"),
    ("outline", "out"),
    ("outline-color", "outc"),
    ("outline-offset", "outo"),
    ("outline-style", "outs"),
    ("outline-width", "outw"),
    ("overflow", "ov"),
    ("overflow-anchor", "ova"),
    ("overflow-block", "ovb"),
    ("overflow-clip-box", "ovcb"),
    ("overflow-clip-margin", "ovcm"),
    ("overflow-inline", "ovi"),
    ("overflow-wrap", "ovw"),
    ("overflow-x", "ovx"),
    ("overflow-y", "ovy"),
    ("overlay", "overlay"),
    ("overscroll-behavior", "ovsb"),
    ("overscroll-behavior-block", "ovsb-b"),
    ("overscroll-behavior-inline", "ovsb-i"),
    ("overscroll-behavior-x", "ovsbx"),
    ("overscroll-behavior-y", "ovsby"),
    ("padding", "p"),
    ("padding-block", "pb"),
    ("padding-block-end", "pbe"),
    ("padding-block-start", "pbs"),
    ("padding-bottom", "pb"),
    ("padding-inline", "pi"),
    ("padding-inline-end", "pie"),
    ("padding-inline-start", "pis"),
    ("padding-left", "pl"),
    ("padding-right", "pr"),
    ("padding-top", "pt"),
    ("page", "page"),
    ("page-break-after", "pba"),
    ("page-break-before", "pbb"),
    ("page-break-inside", "pbi"),
    ("paint-order", "po"),
    ("percentages", "pct"),
    ("perspective", "pers"),
    ("perspective-origin", "pers-or"),
    ("place-content", "pc"),
    ("place-items", "pi"),
    ("place-self", "ps"),
    ("pointer-events", "pe"),
    ("position", "pos"),
    ("position-anchor", "pos-anch"),
    ("position-try", "pos-try"),
    ("position-try-options", "pos-try-opt"),
    ("position-try-order", "pos-try-ord"),
    ("position-visibility", "pos-vis"),
    ("print-color-adjust", "pca"),
    ("quotes", "q"),
    ("resize", "rsz"),
    ("right", "r"),
    ("rotate", "rot"),
    ("row-gap", "rg"),
    ("ruby-align", "ra"),
    ("ruby-merge", "rm"),
    ("ruby-position", "rp"),
    ("scale", "sc"),
    ("scroll-behavior", "sb"),
    ("scroll-margin", "sm"),
    ("scroll-margin-block", "smb"),
    ("scroll-margin-block-end", "smbe"),
    ("scroll-margin-block-start", "smbs"),
    ("scroll-margin-bottom", "smbt"),
    ("scroll-margin-inline", "smi"),
    ("scroll-margin-inline-end", "smie"),
    ("scroll-margin-inline-start", "smis"),
    ("scroll-margin-left", "sml"),
    ("scroll-margin-right", "smr"),
    ("scroll-margin-top", "smt"),
    ("scroll-padding", "sp"),
    ("scroll-padding-block", "spb"),
    ("scroll-padding-block-end", "spbe"),
    ("scroll-padding-block-start", "spbs"),
    ("scroll-padding-bottom", "spbot"),
    ("scroll-padding-inline", "spi"),
    ("scroll-padding-inline-end", "spie"),
    ("scroll-padding-inline-start", "spis"),
    ("scroll-padding-left", "spl"),
    ("scroll-padding-right", "spr"),
    ("scroll-padding-top", "spt"),
    ("scroll-snap-align", "ssa"),
    ("scroll-snap-coordinate", "ssc"),
    ("scroll-snap-destination", "ssd"),
    ("scroll-snap-points-x", "sspx"),
    ("scroll-snap-points-y", "sspy"),
    ("scroll-snap-stop", "sss"),
    ("scroll-snap-type", "sst"),
    ("scroll-snap-type-x", "sstx"),
    ("scroll-snap-type-y", "ssty"),
    ("scroll-timeline", "stl"),
    ("scroll-timeline-axis", "sta"),
    ("scroll-timeline-name", "stn"),
    ("scrollbar-color", "sc"),
    ("scrollbar-gutter", "sg"),
    ("scrollbar-width", "sw"),
    ("shape-image-threshold", "sit"),
    ("shape-margin", "sm"),
    ("shape-outside", "so"),
    ("stacking", "stk"),
    ("status", "sts"),
    ("syntax", "syn"),
    ("tab-size", "ts"),
    ("table-layout", "tl"),
    ("text-align", "ta"),
    ("text-align-last", "tal"),
    ("text-combine-upright", "tcu"),
    ("text-decoration", "td"),
    ("text-decoration-color", "tdc"),
    ("text-decoration-line", "tdl"),
    ("text-decoration-skip", "tds"),
    ("text-decoration-skip-ink", "tdsi"),
    ("text-decoration-style", "tdst"),
    ("text-decoration-thickness", "tdth"),
    ("text-emphasis", "te"),
    ("text-emphasis-color", "tec"),
    ("text-emphasis-position", "tep"),
    ("text-emphasis-style", "tes"),
    ("text-indent", "ti"),
    ("text-justify", "tj"),
    ("text-orientation", "to"),
    ("text-overflow", "tov"),
    ("text-rendering", "tr"),
    ("text-shadow", "tsh"),
    ("text-size-adjust", "tsa"),
    ("text-spacing-trim", "tst"),
    ("text-transform", "tt"),
    ("text-underline-offset", "tuo"),
    ("text-underline-position", "tup"),
    ("text-wrap", "tw"),
    ("text-wrap-mode", "twm"),
    ("text-wrap-style", "tws"),
    ("timeline-scope", "tls"),
    ("top", "t"),
    ("touch-action", "ta"),
    ("transform", "tf"),
    ("transform-box", "tfb"),
    ("transform-origin", "tfo"),
    ("transform-style", "tfs"),
    ("transition", "tr"),
    ("transition-behavior", "trb"),
    ("transition-delay", "trd"),
    ("transition-duration", "trdu"),
    ("transition-property", "trp"),
    ("transition-timing-function", "trtf"),
    ("translate", "tl"),
    ("unicode-bidi", "ub"),
    ("user-select", "us"),
    ("vertical-align", "va"),
    ("view-timeline", "vt"),
    ("view-timeline-axis", "vta"),
    ("view-timeline-inset", "vti"),
    ("view-timeline-name", "vtn"),
    ("view-transition-name", "vtrn"),
    ("visibility", "vis"),
    ("white-space", "ws"),
    ("white-space-collapse", "wsc"),
    ("widows", "wdw"),
    ("width", "w"),
    ("will-change", "wc"),
    ("word-break", "wb"),
    ("word-spacing", "wsp"),
    ("word-wrap", "ww"),
    ("writing-mode", "wm"),
    ("z-index", "z"),
    ("zoom", "zm"),
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_css_property_full_name() {
        assert_eq!(get_css_property("accent-color"), Some("accent-color"));
        assert_eq!(get_css_property("align-items"), Some("align-items"));
    }

    #[test]
    fn test_get_css_property_abbreviation() {
        assert_eq!(get_css_property("acc"), Some("accent-color"));
        assert_eq!(get_css_property("ai"), Some("align-items"));
    }

    #[test]
    fn test_get_css_property_unknown() {
        assert_eq!(get_css_property("unknown"), None);
        assert_eq!(get_css_property("xyz"), None);
    }

    #[test]
    fn test_get_shorten_component() {
        assert_eq!(get_shorten_component("accent-color"), Some("acc"));
        assert_eq!(get_shorten_component("align-items"), Some("ai"));
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
        assert!(components.contains(&"acc"));
        assert!(components.contains(&"align-items"));
        assert!(components.contains(&"ai"));
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
}
