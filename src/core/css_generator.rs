//! This module provides functionality for generating CSS based on spells and configuration settings.
//!
//! The module includes functions to generate CSS class names, handle media queries, and adapt targets
//! based on a given configuration. It is designed to work with `Spell` objects and `GrimoireCSSConfig`
//! to produce the final CSS output, which can include complex rules such as responsive sizing (`mrs` function).
//!
//! Key functionalities:
//!
//! * **CSS Class Name Generation**: Handles the creation of CSS class names from spell components, including
//!   escaping special characters and incorporating spell effects.
//!
//! * **Media Query Wrapping**: Provides functionality to wrap CSS rules within media queries based on screen sizes.
//!
//! * **Grimoire Funtions Handling**: like `mrs`, allowing for flexible and adaptive designs.
//!
//! * **Target Adaptation**: Modifies and adapts CSS targets based on predefined variables in the configuration.
//!
//! The module also includes internal helper functions to manage specific CSS-related tasks such as
//! unit stripping, handling of regex patterns, and combining base CSS with media queries.

use crate::buffer::add_message;

use super::animations::ANIMATIONS;
use super::{config::Config, spell::Spell, GrimoireCSSError};
use regex::Regex;
use std::collections::HashMap;
use std::collections::HashSet;

type GeneratedCSS = (String, (String, String), Option<String>);

type MRSRes = (String, [(String, String); 2]);

pub struct CSSGenerator<'a> {
    config: &'a Config,
    base_css_regex: Regex,
    mrs_regex: Regex,
    unit_regex: Regex,
    animation_block_regex: Regex,
}

impl<'a> CSSGenerator<'a> {
    pub fn new(config: &'a Config) -> Result<Self, GrimoireCSSError> {
        let base_css_regex = Regex::new(r"(\w+)\(([^)]*)\)").map_err(|_| {
            GrimoireCSSError::Regex(regex::Error::Syntax("Invalid regex pattern".to_string()))
        })?;
        let mrs_regex = Regex::new(r"[a-zA-Z]+")?;
        let unit_regex = Regex::new(r"(\d+(\.\d+)?)")?;
        let animation_block_regex =
            Regex::new(r"(?m)(\.GRIMOIRE_CSS_ANIMATION\s*\{[^}]*\})").unwrap();

        Ok(Self {
            config,
            base_css_regex,
            mrs_regex,
            unit_regex,
            animation_block_regex,
        })
    }

    /// Generates CSS based on the given `Spell` and `GrimoireCSSConfig`.
    ///
    /// # Arguments
    ///
    /// * `spell` - A reference to the `Spell` object containing the spell's details.
    /// * `config` - A reference to the `GrimoireCSSConfig` object containing CSS configuration.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(String, String))` 0: containing the generated CSS string if the spell's component is recognized; 1: css class name
    /// * `Ok(None)` if the spell's component is not recognized.
    /// * `Err(GrimoireCSSError)` if there is an error during CSS generation.
    pub fn generate_css(&self, spell: &Spell) -> Result<Option<GeneratedCSS>, GrimoireCSSError> {
        // generate css class name
        let css_class_name = self.generate_css_class_name(
            &spell.raw_spell,
            &spell.effects,
            &spell.focus,
            spell.with_template,
        )?;

        let component_str = spell.component.as_str();

        // match component and get css property
        let css_property: Option<&str> = if component_str.starts_with("--") {
            // css custom properties support
            Some(component_str)
        } else {
            match component_str {
                "accent-color" | "acc" => Some("accent-color"),
                "align-content" | "ac" => Some("align-content"),
                "align-items" | "ai" => Some("align-items"),
                "align-self" | "as" => Some("align-self"),
                "align-tracks" | "atr" => Some("align-tracks"),
                "all" => Some("all"),
                "anchor-name" | "anc-n" => Some("anchor-name"),
                "anchor-scope" | "anc-s" => Some("anchor-scope"),
                "animation" | "anim" => Some("animation"),
                "animation-composition" | "anim-comp" => Some("animation-composition"),
                "animation-delay" | "anim-d" => Some("animation-delay"),
                "animation-direction" | "anim-dir" => Some("animation-direction"),
                "animation-duration" | "anim-du" => Some("animation-duration"),
                "animation-fill-mode" | "anim-fm" => Some("animation-fill-mode"),
                "animation-iteration-count" | "anim-ic" => Some("animation-iteration-count"),
                "animation-name" | "anim-n" => Some("animation-name"),
                "animation-play-state" | "anim-ps" => Some("animation-play-state"),
                "animation-range" | "anim-r" => Some("animation-range"),
                "animation-range-end" | "anim-re" => Some("animation-range-end"),
                "animation-range-start" | "anim-rs" => Some("animation-range-start"),
                "animation-timeline" | "at" => Some("animation-timeline"),
                "animation-timing-function" | "atf" => Some("animation-timing-function"),
                "animationType" | "atype" => Some("animationType"),
                "appearance" | "app" => Some("appearance"),
                "appliesto" | "applies" => Some("appliesto"),
                "aspect-ratio" | "ar" => Some("aspect-ratio"),
                "azimuth" | "az" => Some("azimuth"),
                "backdrop-filter" | "bf" => Some("backdrop-filter"),
                "backface-visibility" | "bvis" => Some("backface-visibility"),
                "background" | "bg" => Some("background"),
                "background-attachment" | "bga" => Some("background-attachment"),
                "background-blend-mode" | "bgblm" => Some("background-blend-mode"),
                "background-clip" | "bgclip" => Some("background-clip"),
                "background-color" | "bgc" => Some("background-color"),
                "background-image" | "bgi" => Some("background-image"),
                "background-origin" | "bgo" => Some("background-origin"),
                "background-position" | "bgp" => Some("background-position"),
                "background-position-x" | "bgpx" => Some("background-position-x"),
                "background-position-y" | "bgpy" => Some("background-position-y"),
                "background-repeat" | "bgr" => Some("background-repeat"),
                "background-size" | "bgsz" => Some("background-size"),
                "block-size" | "bsz" => Some("block-size"),
                "border" | "b" => Some("border"),
                "border-block" | "bblk" => Some("border-block"),
                "border-block-color" | "bblk-c" => Some("border-block-color"),
                "border-block-end" | "bbe" => Some("border-block-end"),
                "border-block-end-color" | "bbec" => Some("border-block-end-color"),
                "border-block-end-style" | "bbes" => Some("border-block-end-style"),
                "border-block-end-width" | "bbew" => Some("border-block-end-width"),
                "border-block-start" | "bblk-s" => Some("border-block-start"),
                "border-block-start-color" | "bbsc" => Some("border-block-start-color"),
                "border-block-start-style" | "bbss" => Some("border-block-start-style"),
                "border-block-start-width" | "bbsw" => Some("border-block-start-width"),
                "border-block-style" | "bblk-sty" => Some("border-block-style"),
                "border-block-width" | "bblk-w" => Some("border-block-width"),
                "border-bottom" | "bbtm" => Some("border-bottom"),
                "border-bottom-color" | "bbtm-c" => Some("border-bottom-color"),
                "border-bottom-left-radius" | "bblr" => Some("border-bottom-left-radius"),
                "border-bottom-right-radius" | "bbrr" => Some("border-bottom-right-radius"),
                "border-bottom-style" | "bbtm-sty" => Some("border-bottom-style"),
                "border-bottom-width" | "bbtm-w" => Some("border-bottom-width"),
                "border-collapse" | "bcoll" => Some("border-collapse"),
                "border-color" | "bc" => Some("border-color"),
                "border-end-end-radius" | "beer" => Some("border-end-end-radius"),
                "border-end-start-radius" | "besr" => Some("border-end-start-radius"),
                "border-image" | "bi" => Some("border-image"),
                "border-image-outset" | "bio" => Some("border-image-outset"),
                "border-image-repeat" | "bir" => Some("border-image-repeat"),
                "border-image-slice" | "bis" => Some("border-image-slice"),
                "border-image-source" | "bisrc" => Some("border-image-source"),
                "border-image-width" | "biw" => Some("border-image-width"),
                "border-inline" | "bli" => Some("border-inline"),
                "border-inline-color" | "blic" => Some("border-inline-color"),
                "border-inline-end" | "blie" => Some("border-inline-end"),
                "border-inline-end-color" | "bliec" => Some("border-inline-end-color"),
                "border-inline-end-style" | "blies" => Some("border-inline-end-style"),
                "border-inline-end-width" | "bliew" => Some("border-inline-end-width"),
                "border-inline-start" | "blis" => Some("border-inline-start"),
                "border-inline-start-color" | "blisc" => Some("border-inline-start-color"),
                "border-inline-start-style" | "bliss" => Some("border-inline-start-style"),
                "border-inline-start-width" | "blisw" => Some("border-inline-start-width"),
                "border-inline-style" | "blin-sty" => Some("border-inline-style"),
                "border-inline-width" | "bliw" => Some("border-inline-width"),
                "border-left" | "bl" => Some("border-left"),
                "border-left-color" | "blc" => Some("border-left-color"),
                "border-left-style" | "bls" => Some("border-left-style"),
                "border-left-width" | "blw" => Some("border-left-width"),
                "border-radius" | "br" => Some("border-radius"),
                "border-right" | "brt" => Some("border-right"),
                "border-right-color" | "brc" => Some("border-right-color"),
                "border-right-style" | "brs" => Some("border-right-style"),
                "border-right-width" | "brw" => Some("border-right-width"),
                "border-spacing" | "bsp" => Some("border-spacing"),
                "border-start-end-radius" | "bser" => Some("border-start-end-radius"),
                "border-start-start-radius" | "bssr" => Some("border-start-start-radius"),
                "border-style" | "bst" => Some("border-style"),
                "border-top" | "bt" => Some("border-top"),
                "border-top-color" | "btc" => Some("border-top-color"),
                "border-top-left-radius" | "btlr" => Some("border-top-left-radius"),
                "border-top-right-radius" | "btrr" => Some("border-top-right-radius"),
                "border-top-style" | "bts" => Some("border-top-style"),
                "border-top-width" | "btw" => Some("border-top-width"),
                "border-width" | "bw" => Some("border-width"),
                "bottom" | "btm" => Some("bottom"),
                "box-align" | "bxa" => Some("box-align"),
                "box-decoration-break" | "bxdb" => Some("box-decoration-break"),
                "box-direction" | "bxd" => Some("box-direction"),
                "box-flex" | "bxf" => Some("box-flex"),
                "box-flex-group" | "bxfg" => Some("box-flex-group"),
                "box-lines" | "bxl" => Some("box-lines"),
                "box-ordinal-group" | "bxog" => Some("box-ordinal-group"),
                "box-orient" | "bxo" => Some("box-orient"),
                "box-pack" | "bxp" => Some("box-pack"),
                "box-shadow" | "bxsh" => Some("box-shadow"),
                "box-sizing" | "bxs" => Some("box-sizing"),
                "break-after" | "ba" => Some("break-after"),
                "break-before" | "brkbf" => Some("break-before"),
                "break-inside" | "brkin" => Some("break-inside"),
                "caption-side" | "capside" => Some("caption-side"),
                "caret" | "crt" => Some("caret"),
                "caret-color" | "cc" => Some("caret-color"),
                "caret-shape" | "carshp" => Some("caret-shape"),
                "clear" | "clr" => Some("clear"),
                "clip" | "clp" => Some("clip"),
                "clip-path" | "clpp" => Some("clip-path"),
                "color" | "c" => Some("color"),
                "color-scheme" | "csch" => Some("color-scheme"),
                "column-count" | "ccnt" => Some("column-count"),
                "column-fill" | "cf" => Some("column-fill"),
                "column-gap" | "cg" => Some("column-gap"),
                "column-rule" | "cr" => Some("column-rule"),
                "column-rule-color" | "crc" => Some("column-rule-color"),
                "column-rule-style" | "crs" => Some("column-rule-style"),
                "column-rule-width" | "crw" => Some("column-rule-width"),
                "column-span" | "csn" => Some("column-span"),
                "column-width" | "cw" => Some("column-width"),
                "columns" | "cols" => Some("columns"),
                "computed" | "cmp" => Some("computed"),
                "contain" | "cntn" => Some("contain"),
                "contain-intrinsic-block-size" | "cibs" => Some("contain-intrinsic-block-size"),
                "contain-intrinsic-height" | "cih" => Some("contain-intrinsic-height"),
                "contain-intrinsic-inline-size" | "ciis" => Some("contain-intrinsic-inline-size"),
                "contain-intrinsic-size" | "cis" => Some("contain-intrinsic-size"),
                "contain-intrinsic-width" | "ciw" => Some("contain-intrinsic-width"),
                "container" | "ctr" => Some("container"),
                "container-name" | "ctrn" => Some("container-name"),
                "container-type" | "ctrt" => Some("container-type"),
                "content" | "cnt" => Some("content"),
                "content-visibility" | "cntv" => Some("content-visibility"),
                "counter-increment" | "cinc" => Some("counter-increment"),
                "counter-reset" | "crst" => Some("counter-reset"),
                "counter-set" | "cset" => Some("counter-set"),
                "cursor" | "cur" => Some("cursor"),
                "direction" | "dir" => Some("direction"),
                "display" | "d" => Some("display"),
                "empty-cells" | "ec" => Some("empty-cells"),
                "field-sizing" | "fsz" => Some("field-sizing"),
                "filter" | "flt" => Some("filter"),
                "flex" | "flx" => Some("flex"),
                "flex-basis" | "flxb" => Some("flex-basis"),
                "flex-direction" | "flex-dir" => Some("flex-direction"),
                "flex-flow" | "flex-fl" => Some("flex-flow"),
                "flex-grow" | "flex-gr" => Some("flex-grow"),
                "flex-shrink" | "flex-sh" => Some("flex-shrink"),
                "flex-wrap" | "flex-wr" => Some("flex-wrap"),
                "float" => Some("float"),
                "font" | "fnt" => Some("font"),
                "font-family" | "ff" => Some("font-family"),
                "font-feature-settings" | "ffs" => Some("font-feature-settings"),
                "font-kerning" | "fk" => Some("font-kerning"),
                "font-language-override" | "flo" => Some("font-language-override"),
                "font-optical-sizing" | "fos" => Some("font-optical-sizing"),
                "font-palette" | "fp" => Some("font-palette"),
                "font-size" | "fs" => Some("font-size"),
                "font-size-adjust" | "fsa" => Some("font-size-adjust"),
                "font-smooth" | "fsm" => Some("font-smooth"),
                "font-stretch" | "fstr" => Some("font-stretch"),
                "font-style" | "fsty" => Some("font-style"),
                "font-synthesis" | "fsyn" => Some("font-synthesis"),
                "font-synthesis-position" | "fsynp" => Some("font-synthesis-position"),
                "font-synthesis-small-caps" | "fssc" => Some("font-synthesis-small-caps"),
                "font-synthesis-style" | "fss" => Some("font-synthesis-style"),
                "font-synthesis-weight" | "fsw" => Some("font-synthesis-weight"),
                "font-variant" | "fv" => Some("font-variant"),
                "font-variant-alternates" | "fva" => Some("font-variant-alternates"),
                "font-variant-caps" | "fvc" => Some("font-variant-caps"),
                "font-variant-east-asian" | "fvea" => Some("font-variant-east-asian"),
                "font-variant-emoji" | "fve" => Some("font-variant-emoji"),
                "font-variant-ligatures" | "fvl" => Some("font-variant-ligatures"),
                "font-variant-numeric" | "fvn" => Some("font-variant-numeric"),
                "font-variant-position" | "fvp" => Some("font-variant-position"),
                "font-variation-settings" | "fvs" => Some("font-variation-settings"),
                "font-weight" | "fw" => Some("font-weight"),
                "forced-color-adjust" | "fca" => Some("forced-color-adjust"),
                "gap" | "g" => Some("gap"),
                "grid" | "grd" => Some("grid"),
                "grid-area" | "gra" => Some("grid-area"),
                "grid-auto-columns" | "grac" => Some("grid-auto-columns"),
                "grid-auto-flow" | "graf" => Some("grid-auto-flow"),
                "grid-auto-rows" | "grar" => Some("grid-auto-rows"),
                "grid-column" | "gc" => Some("grid-column"),
                "grid-column-end" | "gce" => Some("grid-column-end"),
                "grid-column-gap" | "gcg" => Some("grid-column-gap"),
                "grid-column-start" | "gcs" => Some("grid-column-start"),
                "grid-gap" | "gg" => Some("grid-gap"),
                "grid-row" | "grd-r" => Some("grid-row"),
                "grid-row-end" | "gre" => Some("grid-row-end"),
                "grid-row-gap" | "grg" => Some("grid-row-gap"),
                "grid-row-start" | "grs" => Some("grid-row-start"),
                "grid-template" | "gt" => Some("grid-template"),
                "grid-template-areas" | "gta" => Some("grid-template-areas"),
                "grid-template-columns" | "gtc" => Some("grid-template-columns"),
                "grid-template-rows" | "gtr" => Some("grid-template-rows"),
                "groups" | "grps" => Some("groups"),
                "hanging-punctuation" | "hp" => Some("hanging-punctuation"),
                "height" | "h" => Some("height"),
                "hyphenate-character" | "hc" => Some("hyphenate-character"),
                "hyphenate-limit-chars" | "hlc" => Some("hyphenate-limit-chars"),
                "hyphens" | "hy" => Some("hyphens"),
                "image-orientation" | "io" => Some("image-orientation"),
                "image-rendering" | "imgr" => Some("image-rendering"),
                "image-resolution" | "imgres" => Some("image-resolution"),
                "ime-mode" | "im" => Some("ime-mode"),
                "inherited" | "inh" => Some("inherited"),
                "initial" | "init" => Some("initial"),
                "initial-letter" | "initl" => Some("initial-letter"),
                "initial-letter-align" | "initla" => Some("initial-letter-align"),
                "inline-size" | "insz" => Some("inline-size"),
                "input-security" | "inps" => Some("input-security"),
                "inset" | "in" => Some("inset"),
                "inset-area" | "ina" => Some("inset-area"),
                "inset-block" | "inb" => Some("inset-block"),
                "inset-block-end" | "inbe" => Some("inset-block-end"),
                "inset-block-start" | "inbs" => Some("inset-block-start"),
                "inset-inline" | "ini" => Some("inset-inline"),
                "inset-inline-end" | "inie" => Some("inset-inline-end"),
                "inset-inline-start" | "inis" => Some("inset-inline-start"),
                "isolation" | "iso" => Some("isolation"),
                "justify-content" | "jc" => Some("justify-content"),
                "justify-items" | "ji" => Some("justify-items"),
                "justify-self" | "js" => Some("justify-self"),
                "justify-tracks" | "jt" => Some("justify-tracks"),
                "left" | "l" => Some("left"),
                "letter-spacing" | "ltrsp" => Some("letter-spacing"),
                "line-break" | "lb" => Some("line-break"),
                "line-clamp" | "lc" => Some("line-clamp"),
                "line-height" | "lh" => Some("line-height"),
                "line-height-step" | "lhs" => Some("line-height-step"),
                "list-style" | "lstyl" => Some("list-style"),
                "list-style-image" | "lsi" => Some("list-style-image"),
                "list-style-position" | "lsp" => Some("list-style-position"),
                "list-style-type" | "lst" => Some("list-style-type"),
                "margin" | "m" => Some("margin"),
                "margin-block" | "mblk" => Some("margin-block"),
                "margin-block-end" | "mbe" => Some("margin-block-end"),
                "margin-block-start" | "mbs" => Some("margin-block-start"),
                "margin-bottom" | "mbtm" => Some("margin-bottom"),
                "margin-inline" | "mi" => Some("margin-inline"),
                "margin-inline-end" | "mie" => Some("margin-inline-end"),
                "margin-inline-start" | "mis" => Some("margin-inline-start"),
                "margin-left" | "ml" => Some("margin-left"),
                "margin-right" | "mr" => Some("margin-right"),
                "margin-top" | "mt" => Some("margin-top"),
                "margin-trim" | "mtrim" => Some("margin-trim"),
                "mask" => Some("mask"),
                "mask-border" | "mask-b" => Some("mask-border"),
                "mask-border-mode" | "mask-bm" => Some("mask-border-mode"),
                "mask-border-outset" | "mask-bo" => Some("mask-border-outset"),
                "mask-border-repeat" | "mask-br" => Some("mask-border-repeat"),
                "mask-border-slice" | "mask-bs" => Some("mask-border-slice"),
                "mask-border-source" | "mask-bsou" => Some("mask-border-source"),
                "mask-border-width" | "mask-bw" => Some("mask-border-width"),
                "mask-clip" | "mask-c" => Some("mask-clip"),
                "mask-composite" | "mask-comp" => Some("mask-composite"),
                "mask-image" | "mask-i" => Some("mask-image"),
                "mask-mode" | "mask-m" => Some("mask-mode"),
                "mask-origin" | "mask-o" => Some("mask-origin"),
                "mask-position" | "mask-pos" => Some("mask-position"),
                "mask-repeat" | "mask-r" => Some("mask-repeat"),
                "mask-size" | "mask-sz" => Some("mask-size"),
                "mask-type" | "mask-t" => Some("mask-type"),
                "masonry-auto-flow" | "mas-af" => Some("masonry-auto-flow"),
                "math-depth" | "math-d" => Some("math-depth"),
                "math-shift" | "math-s" => Some("math-shift"),
                "math-style" | "math-st" => Some("math-style"),
                "max-block-size" | "max-bs" => Some("max-block-size"),
                "max-height" | "max-h" => Some("max-height"),
                "max-inline-size" | "max-is" => Some("max-inline-size"),
                "max-lines" | "max-l" => Some("max-lines"),
                "max-width" | "max-w" => Some("max-width"),
                "mdn_url" | "mdn-u" => Some("mdn_url"),
                "media" | "med" => Some("media"),
                "min-block-size" | "min-bs" => Some("min-block-size"),
                "min-height" | "min-h" => Some("min-height"),
                "min-inline-size" | "min-is" => Some("min-inline-size"),
                "min-width" | "min-w" => Some("min-width"),
                "mix-blend-mode" | "mbm" => Some("mix-blend-mode"),
                "object-fit" | "obj-fit" => Some("object-fit"),
                "object-position" | "obj-pos" => Some("object-position"),
                "offset" | "off" => Some("offset"),
                "offset-anchor" | "ofa" => Some("offset-anchor"),
                "offset-distance" | "ofd" => Some("offset-distance"),
                "offset-path" | "ofp" => Some("offset-path"),
                "offset-position" | "ofpos" => Some("offset-position"),
                "offset-rotate" | "ofr" => Some("offset-rotate"),
                "opacity" | "op" => Some("opacity"),
                "order" | "ord" => Some("order"),
                "orphans" | "orphan" => Some("orphans"),
                "outline" | "out" => Some("outline"),
                "outline-color" | "outc" => Some("outline-color"),
                "outline-offset" | "outo" => Some("outline-offset"),
                "outline-style" | "outs" => Some("outline-style"),
                "outline-width" | "outw" => Some("outline-width"),
                "overflow" | "ov" => Some("overflow"),
                "overflow-anchor" | "ova" => Some("overflow-anchor"),
                "overflow-block" | "ovb" => Some("overflow-block"),
                "overflow-clip-box" | "ovcb" => Some("overflow-clip-box"),
                "overflow-clip-margin" | "ovcm" => Some("overflow-clip-margin"),
                "overflow-inline" | "ovi" => Some("overflow-inline"),
                "overflow-wrap" | "ovw" => Some("overflow-wrap"),
                "overflow-x" | "ovx" => Some("overflow-x"),
                "overflow-y" | "ovy" => Some("overflow-y"),
                "overlay" => Some("overlay"),
                "overscroll-behavior" | "ovsb" => Some("overscroll-behavior"),
                "overscroll-behavior-block" | "ovsb-b" => Some("overscroll-behavior-block"),
                "overscroll-behavior-inline" | "ovsb-i" => Some("overscroll-behavior-inline"),
                "overscroll-behavior-x" | "ovsbx" => Some("overscroll-behavior-x"),
                "overscroll-behavior-y" | "ovsby" => Some("overscroll-behavior-y"),
                "padding" | "p" => Some("padding"),
                "padding-block" | "pblk" => Some("padding-block"),
                "padding-block-end" | "pbe" => Some("padding-block-end"),
                "padding-block-start" | "pbs" => Some("padding-block-start"),
                "padding-bottom" | "pbtm" => Some("padding-bottom"),
                "padding-inline" | "pinl" => Some("padding-inline"),
                "padding-inline-end" | "pie" => Some("padding-inline-end"),
                "padding-inline-start" | "pis" => Some("padding-inline-start"),
                "padding-left" | "pl" => Some("padding-left"),
                "padding-right" | "pr" => Some("padding-right"),
                "padding-top" | "pt" => Some("padding-top"),
                "page" => Some("page"),
                "page-break-after" | "pba" => Some("page-break-after"),
                "page-break-before" | "pbb" => Some("page-break-before"),
                "page-break-inside" | "pbi" => Some("page-break-inside"),
                "paint-order" | "po" => Some("paint-order"),
                "percentages" | "pct" => Some("percentages"),
                "perspective" | "pers" => Some("perspective"),
                "perspective-origin" | "pers-or" => Some("perspective-origin"),
                "place-content" | "pc" => Some("place-content"),
                "place-items" | "plc-it" => Some("place-items"),
                "place-self" | "ps" => Some("place-self"),
                "pointer-events" | "pe" => Some("pointer-events"),
                "position" | "pos" => Some("position"),
                "position-anchor" | "pos-anch" => Some("position-anchor"),
                "position-try" | "pos-try" => Some("position-try"),
                "position-try-options" | "pos-try-opt" => Some("position-try-options"),
                "position-try-order" | "pos-try-ord" => Some("position-try-order"),
                "position-visibility" | "pos-vis" => Some("position-visibility"),
                "print-color-adjust" | "pca" => Some("print-color-adjust"),
                "quotes" | "q" => Some("quotes"),
                "resize" | "rsz" => Some("resize"),
                "right" | "r" => Some("right"),
                "rotate" | "rot" => Some("rotate"),
                "row-gap" | "rg" => Some("row-gap"),
                "ruby-align" | "ra" => Some("ruby-align"),
                "ruby-merge" | "rm" => Some("ruby-merge"),
                "ruby-position" | "rp" => Some("ruby-position"),
                "scale" | "scl" => Some("scale"),
                "scroll-behavior" | "sb" => Some("scroll-behavior"),
                "scroll-margin" | "scrlm" => Some("scroll-margin"),
                "scroll-margin-block" | "smb" => Some("scroll-margin-block"),
                "scroll-margin-block-end" | "smbe" => Some("scroll-margin-block-end"),
                "scroll-margin-block-start" | "smbs" => Some("scroll-margin-block-start"),
                "scroll-margin-bottom" | "smbt" => Some("scroll-margin-bottom"),
                "scroll-margin-inline" | "smi" => Some("scroll-margin-inline"),
                "scroll-margin-inline-end" | "smie" => Some("scroll-margin-inline-end"),
                "scroll-margin-inline-start" | "smis" => Some("scroll-margin-inline-start"),
                "scroll-margin-left" | "sml" => Some("scroll-margin-left"),
                "scroll-margin-right" | "smr" => Some("scroll-margin-right"),
                "scroll-margin-top" | "smt" => Some("scroll-margin-top"),
                "scroll-padding" | "sp" => Some("scroll-padding"),
                "scroll-padding-block" | "spb" => Some("scroll-padding-block"),
                "scroll-padding-block-end" | "spbe" => Some("scroll-padding-block-end"),
                "scroll-padding-block-start" | "spbs" => Some("scroll-padding-block-start"),
                "scroll-padding-bottom" | "spbot" => Some("scroll-padding-bottom"),
                "scroll-padding-inline" | "spi" => Some("scroll-padding-inline"),
                "scroll-padding-inline-end" | "spie" => Some("scroll-padding-inline-end"),
                "scroll-padding-inline-start" | "spis" => Some("scroll-padding-inline-start"),
                "scroll-padding-left" | "spl" => Some("scroll-padding-left"),
                "scroll-padding-right" | "spr" => Some("scroll-padding-right"),
                "scroll-padding-top" | "spt" => Some("scroll-padding-top"),
                "scroll-snap-align" | "ssa" => Some("scroll-snap-align"),
                "scroll-snap-coordinate" | "ssc" => Some("scroll-snap-coordinate"),
                "scroll-snap-destination" | "ssd" => Some("scroll-snap-destination"),
                "scroll-snap-points-x" | "sspx" => Some("scroll-snap-points-x"),
                "scroll-snap-points-y" | "sspy" => Some("scroll-snap-points-y"),
                "scroll-snap-stop" | "sss" => Some("scroll-snap-stop"),
                "scroll-snap-type" | "sst" => Some("scroll-snap-type"),
                "scroll-snap-type-x" | "sstx" => Some("scroll-snap-type-x"),
                "scroll-snap-type-y" | "ssty" => Some("scroll-snap-type-y"),
                "scroll-timeline" | "stl" => Some("scroll-timeline"),
                "scroll-timeline-axis" | "sta" => Some("scroll-timeline-axis"),
                "scroll-timeline-name" | "stn" => Some("scroll-timeline-name"),
                "scrollbar-color" | "scrlbc" => Some("scrollbar-color"),
                "scrollbar-gutter" | "sg" => Some("scrollbar-gutter"),
                "scrollbar-width" | "sw" => Some("scrollbar-width"),
                "shape-image-threshold" | "sit" => Some("shape-image-threshold"),
                "shape-margin" | "shpm" => Some("shape-margin"),
                "shape-outside" | "so" => Some("shape-outside"),
                "stacking" | "stk" => Some("stacking"),
                "status" | "sts" => Some("status"),
                "syntax" | "syn" => Some("syntax"),
                "tab-size" | "ts" => Some("tab-size"),
                "table-layout" | "tbllyt" => Some("table-layout"),
                "text-align" | "ta" => Some("text-align"),
                "text-align-last" | "tal" => Some("text-align-last"),
                "text-combine-upright" | "tcu" => Some("text-combine-upright"),
                "text-decoration" | "td" => Some("text-decoration"),
                "text-decoration-color" | "tdc" => Some("text-decoration-color"),
                "text-decoration-line" | "tdl" => Some("text-decoration-line"),
                "text-decoration-skip" | "tds" => Some("text-decoration-skip"),
                "text-decoration-skip-ink" | "tdsi" => Some("text-decoration-skip-ink"),
                "text-decoration-style" | "tdst" => Some("text-decoration-style"),
                "text-decoration-thickness" | "tdth" => Some("text-decoration-thickness"),
                "text-emphasis" | "te" => Some("text-emphasis"),
                "text-emphasis-color" | "tec" => Some("text-emphasis-color"),
                "text-emphasis-position" | "tep" => Some("text-emphasis-position"),
                "text-emphasis-style" | "tes" => Some("text-emphasis-style"),
                "text-indent" | "ti" => Some("text-indent"),
                "text-justify" | "tj" => Some("text-justify"),
                "text-orientation" | "to" => Some("text-orientation"),
                "text-overflow" | "tov" => Some("text-overflow"),
                "text-rendering" | "txtrnd" => Some("text-rendering"),
                "text-shadow" | "tsh" => Some("text-shadow"),
                "text-size-adjust" | "tsa" => Some("text-size-adjust"),
                "text-spacing-trim" | "tst" => Some("text-spacing-trim"),
                "text-transform" | "tt" => Some("text-transform"),
                "text-underline-offset" | "tuo" => Some("text-underline-offset"),
                "text-underline-position" | "tup" => Some("text-underline-position"),
                "text-wrap" | "tw" => Some("text-wrap"),
                "text-wrap-mode" | "twm" => Some("text-wrap-mode"),
                "text-wrap-style" | "tws" => Some("text-wrap-style"),
                "timeline-scope" | "tls" => Some("timeline-scope"),
                "top" | "t" => Some("top"),
                "touch-action" | "tchact" => Some("touch-action"),
                "transform" | "tf" => Some("transform"),
                "transform-box" | "tfb" => Some("transform-box"),
                "transform-origin" | "tfo" => Some("transform-origin"),
                "transform-style" | "tfs" => Some("transform-style"),
                "transition" | "trans" => Some("transition"),
                "transition-behavior" | "trb" => Some("transition-behavior"),
                "transition-delay" | "trd" => Some("transition-delay"),
                "transition-duration" | "trdu" => Some("transition-duration"),
                "transition-property" | "trp" => Some("transition-property"),
                "transition-timing-function" | "trtf" => Some("transition-timing-function"),
                "translate" | "transl" => Some("translate"),
                "unicode-bidi" | "ub" => Some("unicode-bidi"),
                "user-select" | "us" => Some("user-select"),
                "vertical-align" | "va" => Some("vertical-align"),
                "view-timeline" | "vt" => Some("view-timeline"),
                "view-timeline-axis" | "vta" => Some("view-timeline-axis"),
                "view-timeline-inset" | "vti" => Some("view-timeline-inset"),
                "view-timeline-name" | "vtn" => Some("view-timeline-name"),
                "view-transition-name" | "vtrn" => Some("view-transition-name"),
                "visibility" | "vis" => Some("visibility"),
                "white-space" | "ws" => Some("white-space"),
                "white-space-collapse" | "wsc" => Some("white-space-collapse"),
                "widows" | "wdw" => Some("widows"),
                "width" | "w" => Some("width"),
                "will-change" | "wc" => Some("will-change"),
                "word-break" | "wb" => Some("word-break"),
                "word-spacing" | "wsp" => Some("word-spacing"),
                "word-wrap" | "ww" => Some("word-wrap"),
                "writing-mode" | "wm" => Some("writing-mode"),
                "z-index" | "z" => Some("z-index"),
                "zoom" | "zm" => Some("zoom"),
                // --- CUSTOM ---
                // generate built-in animation with all predefined rules
                "g-anim" => Some("g-anim"),

                _ => None,
            }
        };

        match css_property {
            Some(css_property) => {
                // adapt target
                let adapted_target = self.adapt_targets(&spell.component_target, self.config)?;
                // generate base css without any media queries (except for the mrs and mfs functions)
                let (base_css, additional_css) = self.generate_base_and_additional_css(
                    &adapted_target,
                    &css_class_name.0,
                    css_property,
                )?;

                if !spell.area.is_empty() {
                    return Ok(Some((
                        self.wrap_base_css_with_media_query(&spell.area, &base_css),
                        css_class_name,
                        additional_css,
                    )));
                }

                Ok(Some((base_css, css_class_name, additional_css)))
            }
            None => Ok(None),
        }
    }

    /// Wraps base CSS with a media query.
    ///
    /// # Arguments
    ///
    /// * `size_var` - A reference to a string specifying the size variant (e.g., "sm", "md").
    /// * `base_css` - A reference to the base CSS string.
    ///
    /// # Returns
    ///
    /// * A `String` containing the base CSS wrapped in the appropriate media query.
    pub fn wrap_base_css_with_media_query(&self, area: &str, base_css: &str) -> String {
        match area {
            "sm" => self.wrap_size_area("640px", base_css),
            "md" => self.wrap_size_area("768px", base_css),
            "lg" => self.wrap_size_area("1024px", base_css),
            "xl" => self.wrap_size_area("1280px", base_css),
            "2xl" => self.wrap_size_area("1536px", base_css),
            _ => format!(
                "@media {}{{{}}}",
                area.split('_').collect::<Vec<&str>>().join(" "),
                base_css
            ),
        }
    }

    fn wrap_size_area(&self, area: &str, base_css: &str) -> String {
        format!("@media (min-width: {}){{{}}}", area, base_css)
    }

    pub fn generate_css_class_name(
        &self,
        raw_spell: &str,
        effects: &str,
        raw_spell_focus: &str,
        with_template: bool,
    ) -> Result<(String, String), GrimoireCSSError> {
        let spell_focus = raw_spell_focus.split('_').collect::<Vec<&str>>().join(" ");
        let mut escaped_class_name = self.escape_css_class_name(raw_spell)?;

        if with_template {
            escaped_class_name = format!(".g\\!{}\\;", escaped_class_name);
        } else {
            escaped_class_name = format!(".{}", escaped_class_name);
        }

        let effects_string = Self::generate_effect(effects)?;

        let base_class_name = escaped_class_name.clone();

        if !effects_string.is_empty() {
            escaped_class_name.push_str(&format!(":{}", effects_string));
        }

        if !spell_focus.is_empty() {
            escaped_class_name.push_str(&spell_focus);
        }

        Ok((escaped_class_name, base_class_name))
    }

    /// Escapes special characters in the CSS class name.
    ///
    /// # Arguments
    ///
    /// * `class_name` - A reference to the class name string to be escaped.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the escaped class name.
    /// * `Err(GrimoireCSSError)` if the input is invalid.
    fn escape_css_class_name(&self, class_name: &str) -> Result<String, GrimoireCSSError> {
        let escaped = class_name
            .chars()
            .map(|c| match c {
                '!' | '"' | '#' | '$' | '%' | '&' | '\'' | '(' | ')' | '*' | '+' | ',' | '.'
                | '/' | ':' | ';' | '<' | '=' | '>' | '?' | '@' | '[' | '\\' | ']' | '^' | '_'
                | '`' | '{' | '|' | '}' | '~' => format!("\\{}", c),
                ' ' => {
                    add_message("HTML does not support spaces. To separate values use underscore ('_') instead".to_string());
                    c.to_string()
                }
                _ => c.to_string(),
            })
            .collect::<String>();

        if escaped.is_empty() {
            return Err(GrimoireCSSError::InvalidSpellFormat(class_name.to_string()));
        }

        Ok(escaped)
    }

    /// Generates a string representing the effects of the spell.
    ///
    /// # Arguments
    ///
    /// * `effect` - A reference to the effects string.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the formatted effects string.
    /// * `Err(GrimoireCSSError)` if there is an error during effect generation.
    fn generate_effect(effect: &str) -> Result<String, GrimoireCSSError> {
        Ok(effect.split(",").collect::<Vec<&str>>().join(":"))
    }

    /// Adapts the target string based on the configuration.
    ///
    /// # Arguments
    ///
    /// * `target` - A reference to the target string.
    /// * `config` - A reference to the `GrimoireCSSConfig` object containing CSS configuration.
    ///
    /// # Returns
    ///
    /// * `Ok(String)` containing the adapted target string.
    /// * `Err(GrimoireCSSError)` if there is an error during target adaptation.
    fn adapt_targets(&self, target: &str, config: &Config) -> Result<String, GrimoireCSSError> {
        let mut result = String::new();

        let formatted_target = target.split('_').collect::<Vec<&str>>().join(" ");

        let variables = config.variables.as_ref();
        let mut replaced_target = formatted_target.clone();

        if let Some(v) = variables {
            for (key, value) in v {
                let placeholder = format!("${}", key);
                replaced_target = replaced_target.replace(&placeholder, value);
            }
        }

        result.push_str(&replaced_target);

        Ok(result)
    }

    /// Generates the base CSS and optional additional CSS (keyframes) based on the given property.
    ///
    /// This function delegates the generation logic to specific handlers depending on the property
    /// (e.g., `g-anim`, `animation`, `animation-name`). For other properties, it generates a generic CSS string.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the adapted target string, representing the value for the CSS property.
    /// * `css_class_name` - A reference to the CSS class name.
    /// * `property` - A reference to the CSS property name (e.g., `g-anim`, `animation`, `animation-name`).
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS string and an optional string containing additional keyframes CSS.
    /// * `Err(GrimoireCSSError)` - If an error occurs during the CSS generation process.
    fn generate_base_and_additional_css(
        &self,
        adapted_target: &str,
        css_class_name: &str,
        property: &str,
    ) -> Result<(String, Option<String>), GrimoireCSSError> {
        match property {
            "g-anim" => self.handle_g_anim(adapted_target, css_class_name),
            "animation" => self.handle_animation(adapted_target, css_class_name),
            "animation-name" => self.handle_animation_name(adapted_target, css_class_name),
            _ => self.handle_generic_css(adapted_target, css_class_name, property),
        }
    }

    /// Handles CSS generation for `g-anim` property.
    ///
    /// This function retrieves the corresponding animation from `ANIMATIONS`, replaces the
    /// placeholder with the actual class name, and returns both the base CSS and keyframes.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the target animation name (e.g., `heart-beat`).
    /// * `css_class_name` - A reference to the CSS class name that will replace the placeholder in the animation.
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS and additional keyframes CSS.
    /// * `Err(GrimoireCSSError)` - If the target animation is not found or any error occurs during processing.
    fn handle_g_anim(
        &self,
        adapted_target: &str,
        css_class_name: &str,
    ) -> Result<(String, Option<String>), GrimoireCSSError> {
        if let Some(animation) = ANIMATIONS.get(adapted_target) {
            let (keyframes, class) =
                self.get_keyframe_class_from_animation(animation, adapted_target)?;
            let base_css = class.replace(".GRIMOIRE_CSS_ANIMATION", css_class_name);
            return Ok((base_css, Some(keyframes)));
        }

        if let Some(animation) = self.config.custom_animations.get(adapted_target) {
            let (keyframes, class) =
                self.get_keyframe_class_from_animation(animation, adapted_target)?;
            let base_css = class.replace(".GRIMOIRE_CSS_ANIMATION", css_class_name);
            return Ok((base_css, Some(keyframes)));
        }

        Err(GrimoireCSSError::InvalidSpellFormat(
            adapted_target.to_string(),
        ))
    }

    /// Handles CSS generation for the `animation` property.
    ///
    /// This function checks for the presence of keyframes in the animation name and generates the
    /// appropriate base CSS and optional additional keyframes.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the animation value (e.g., `3s linear wobble`).
    /// * `css_class_name` - A reference to the CSS class name used for generating the base CSS.
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS and optional keyframes.
    /// * `Err(GrimoireCSSError)` - If any error occurs during processing.
    fn handle_animation(
        &self,
        adapted_target: &str,
        css_class_name: &str,
    ) -> Result<(String, Option<String>), GrimoireCSSError> {
        let additional_css = self.get_additional_css(adapted_target)?;
        let base_css = format!("{}{{animation:{};}}", css_class_name, adapted_target);
        Ok((base_css, additional_css))
    }

    /// Handles CSS generation for the `animation-name` property.
    ///
    /// This function retrieves keyframes from the animation name and generates the base CSS for the `animation-name` property.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the animation name (e.g., `tada`).
    /// * `css_class_name` - A reference to the CSS class name for generating the base CSS.
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS and optional additional keyframes.
    /// * `Err(GrimoireCSSError)` - If any error occurs during processing.
    fn handle_animation_name(
        &self,
        adapted_target: &str,
        css_class_name: &str,
    ) -> Result<(String, Option<String>), GrimoireCSSError> {
        let additional_css = self.get_additional_css(adapted_target)?;
        let base_css = format!("{}{{animation-name:{};}}", css_class_name, adapted_target);
        Ok((base_css, additional_css))
    }

    /// Generates generic CSS for properties not specifically handled (`g-anim`, `animation`, or `animation-name`).
    ///
    /// This function uses regular expressions to capture patterns in the target string and apply any necessary transformations.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the target value for the CSS property.
    /// * `css_class_name` - A reference to the CSS class name.
    /// * `property` - A reference to the CSS property name.
    ///
    /// # Returns
    ///
    /// * `Ok((String, Option<String>))` - The base CSS string and no additional CSS.
    /// * `Err(GrimoireCSSError)` - If an error occurs during processing.
    fn handle_generic_css(
        &self,
        adapted_target: &str,
        css_class_name: &str,
        property: &str,
    ) -> Result<(String, Option<String>), GrimoireCSSError> {
        let base_css = format!("{}{{{}:{};}}", css_class_name, property, adapted_target);
        let captures = self
            .base_css_regex
            .captures_iter(adapted_target)
            .collect::<Vec<_>>();

        if !captures.is_empty() {
            if let Some((base, media)) =
                self.handle_grimoire_functions(adapted_target, captures, property, css_class_name)?
            {
                Ok((
                    format!("{}{{{}:{};}}{}", css_class_name, property, base, media),
                    None,
                ))
            } else {
                Ok((base_css, None))
            }
        } else {
            Ok((base_css, None))
        }
    }

    /// Retrieves additional CSS (keyframes) based on the animation name.
    ///
    /// This function checks if the given animation name corresponds to any predefined animation in the `ANIMATIONS` list
    /// or in the user's `custom_animations`.
    /// If a matching animation is found, the corresponding keyframes are returned.
    ///
    /// # Arguments
    ///
    /// * `adapted_target` - A reference to the target animation value or name.
    ///
    /// # Returns
    ///
    /// * `Ok(Some(String))` - The keyframes CSS if a matching animation is found.
    /// * `Ok(None)` - If no matching keyframes are found.
    /// * `Err(GrimoireCSSError)` - If an error occurs during processing.
    fn get_additional_css(&self, adapted_target: &str) -> Result<Option<String>, GrimoireCSSError> {
        if let Some(grimoire_animation_name) = Self::find_grimoire_animation_name(adapted_target) {
            if let Some(animation) = ANIMATIONS.get(grimoire_animation_name) {
                let (keyframes, _) =
                    self.get_keyframe_class_from_animation(animation, grimoire_animation_name)?;
                return Ok(Some(keyframes));
            }
        }

        if let Some(custom_animation) = self.config.custom_animations.get(adapted_target) {
            let (keyframes, _) =
                self.get_keyframe_class_from_animation(custom_animation, adapted_target)?;
            return Ok(Some(keyframes));
        }

        Ok(None)
    }

    /// Handles specific grimoire functions in the target string.
    ///
    /// # Arguments
    ///
    /// * `target` - A reference to the target string.
    /// * `captures` - A vector of regex captures from the target string.
    /// * `property` - A reference to the CSS property string.
    /// * `css_class_name` - A reference to the CSS class name string.
    ///
    /// # Returns
    ///
    /// * `Ok(Some((String, String)))` containing the base and media query CSS strings if functions are handled.
    /// * `Ok(None)` if no functions are found.
    /// * `Err(GrimoireCSSError)` if there is an error during function handling.
    fn handle_grimoire_functions(
        &self,
        target: &str,
        captures: Vec<regex::Captures>,
        property: &str,
        css_class_name: &str,
    ) -> Result<Option<(String, String)>, GrimoireCSSError> {
        #[derive(Debug)]
        struct Media {
            size: String,
            value: Vec<String>,
        }

        let mut base = target.to_owned();
        let mut media: Vec<Media> = Vec::new();
        let mut screen_sizes_state: HashSet<String> = HashSet::with_capacity(2);
        let mut calculations_base_count = 0;

        #[derive(Debug)]
        struct CalculationInfo {
            calculated: String,
            media_queries: [(String, String); 2],
        }
        let mut calculation_map: HashMap<String, CalculationInfo> = HashMap::new();

        for capture in captures {
            let function_name = &capture[1];
            let args = &capture[2];

            if function_name == "mrs" {
                if let Some((base_value, media_queries)) =
                    self.handle_mrs(args, &mut screen_sizes_state)?
                {
                    let key = format!("mrs_{}", calculations_base_count);
                    calculations_base_count += 1;

                    for (media_size, _) in &media_queries {
                        if !media.iter().any(|m| m.size == *media_size) {
                            media.push(Media {
                                size: media_size.to_owned(),
                                value: Vec::new(),
                            });
                        }
                    }

                    calculation_map.insert(
                        key.to_owned(),
                        CalculationInfo {
                            calculated: base_value.to_owned(),
                            media_queries,
                        },
                    );

                    base = base.replace(&capture[0], &key);
                }
            }
        }

        let binding = base.clone();
        let parts = binding.split_whitespace().collect::<Vec<&str>>();

        base = parts
            .iter()
            .map(|&p| {
                calculation_map
                    .get(p)
                    .map_or(p.to_string(), |info| info.calculated.clone())
            })
            .collect::<Vec<String>>()
            .join(" ");

        for media_item in &mut media {
            let media_value: Vec<String> = parts
                .iter()
                .map(|&p| {
                    calculation_map.get(p).map_or(p.to_string(), |info| {
                        info.media_queries
                            .iter()
                            .find(|(size, _)| size == &media_item.size)
                            .map_or(p.to_string(), |(_, value)| value.clone())
                    })
                })
                .collect();
            media_item.value = media_value;
        }

        if base != target {
            let media_queries_str = media
                .into_iter()
                .map(|media_item| {
                    let values_str = media_item.value.join(" ");
                    format!(
                        "@media screen and (min-width: {}) {{{}{{{}: {};}}}}",
                        media_item.size, css_class_name, property, values_str
                    )
                })
                .collect::<Vec<_>>()
                .join("");
            Ok(Some((base, media_queries_str)))
        } else {
            Ok(None)
        }
    }

    /// Handles the `mrs` function for responsive sizing.
    ///
    /// # Arguments
    ///
    /// * `args` - A reference to the arguments string.
    /// * `screen_sizes_state` - A mutable reference to a set of screen sizes.
    ///
    /// # Returns
    ///
    /// * `Ok(Some((String, [(String, String); 2])))` containing the base size and media queries.
    /// * `Ok(None)` if no valid responsive size is found.
    /// * `Err(GrimoireCSSError)` if there is an error during responsive size handling.
    fn handle_mrs(
        &self,
        args: &str,
        screen_sizes_state: &mut HashSet<String>,
    ) -> Result<Option<MRSRes>, GrimoireCSSError> {
        let mut parts = args.split(' ');

        let min_size = parts.next().unwrap_or("0px");
        let max_size = parts.next().unwrap_or("0px");
        let min_vw = parts.next();
        let max_vw = parts.next();

        self.make_responsive_size(min_size, max_size, min_vw, max_vw, screen_sizes_state)
    }

    /// Generates a responsive size and corresponding media queries based on the given parameters.
    ///
    /// # Arguments
    ///
    /// * `min_size` - A reference to the minimum size string (e.g., "100px").
    /// * `max_size` - A reference to the maximum size string (e.g., "200px").
    /// * `min_vw` - An optional reference to the minimum viewport width (e.g., "480px").
    /// * `max_vw` - An optional reference to the maximum viewport width (e.g., "1280px").
    /// * `screen_sizes_state` - A mutable reference to a `HashSet` tracking the current screen sizes used.
    ///
    /// # Returns
    ///
    /// * `Ok(Some((String, [(String, String); 2])))` containing the base size and an array of media queries.
    /// * `Ok(None)` if the input sizes and viewport widths are incompatible.
    /// * `Err(GrimoireCSSError)` if there is an error in processing the sizes or if the screen sizes state is invalid.
    fn make_responsive_size(
        &self,
        min_size: &str,
        max_size: &str,
        min_vw: Option<&str>,
        max_vw: Option<&str>,
        screen_sizes_state: &mut HashSet<String>,
    ) -> Result<Option<MRSRes>, GrimoireCSSError> {
        let min_size_value: u32 = self.strip_unit(min_size)?;
        let max_size_value: u32 = self.strip_unit(max_size)?;
        let min_vw_value: u32 = match min_vw {
            Some(i) => self.strip_unit(i)?,
            None => 480,
        };
        let max_vw_value: u32 = match max_vw {
            Some(i) => self.strip_unit(i)?,
            None => 1280,
        };

        let min_size_unit = self.mrs_regex.find(min_size).map_or("", |m| m.as_str());
        let max_size_unit = self.mrs_regex.find(max_size).map_or("", |m| m.as_str());
        let min_vw_unit = match min_vw {
            Some(i) => self.mrs_regex.find(i).map_or("", |m| m.as_str()),
            None => "px",
        };
        let max_vw_unit = match max_vw {
            Some(i) => self.mrs_regex.find(i).map_or("", |m| m.as_str()),
            None => "px",
        };

        let full_min_vw = format!("{}{}", min_vw_value, min_vw_unit);
        let full_max_vw = format!("{}{}", max_vw_value, max_vw_unit);

        // update state and handle different screen sizes
        if screen_sizes_state.is_empty() {
            screen_sizes_state.insert(full_min_vw.clone());
            screen_sizes_state.insert(full_max_vw.clone());
        } else if screen_sizes_state.len() == 2
            && (screen_sizes_state.get(&full_min_vw).is_none()
                || screen_sizes_state.get(&full_max_vw).is_none())
        {
            return Err(GrimoireCSSError::InvalidInput(
                "Different screen sizes are not allowed in one rule".to_string(),
            ));
        } else if screen_sizes_state.len() != 2 {
            return Err(GrimoireCSSError::InvalidInput(format!(
                "Unexpected screen size state: {:?}",
                screen_sizes_state
            )));
        }

        if min_vw_unit == max_vw_unit
            && min_vw_unit == min_size_unit
            && min_vw_unit == max_size_unit
        {
            let vw_diff = max_vw_value - min_vw_value;
            let size_diff = max_size_value - min_size_value;

            let base = min_size.to_owned();
            let media: [(String, String); 2] = [
                (
                    format!("{}{}", min_vw_value, min_vw_unit),
                    format!(
                        "calc({} + {} * ((100vw - {}{}) / {}))",
                        min_size, size_diff, min_vw_value, min_vw_unit, vw_diff
                    ),
                ),
                (
                    format!("{}{}", max_vw_value, max_vw_unit),
                    max_size.to_string(),
                ),
            ];

            Ok(Some((base, media)))
        } else {
            Ok(None)
        }
    }

    /// Strips the unit from a CSS size value and returns the numeric part.
    ///
    /// # Arguments
    ///
    /// * `value` - A reference to the value string containing the unit.
    ///
    /// # Returns
    ///
    /// * `Ok(u32)` containing the numeric part of the value.
    /// * `Err(GrimoireCSSError)` if there is an error during unit stripping.
    fn strip_unit(&self, value: &str) -> Result<u32, GrimoireCSSError> {
        if let Some(captures) = self.unit_regex.captures(value) {
            captures[1].parse::<u32>().map_err(|_| {
                GrimoireCSSError::InvalidInput(format!(
                    "Failed to parse unit from value: {}",
                    value
                ))
            })
        } else {
            Err(GrimoireCSSError::InvalidInput(format!(
                "No numeric value found in: {}",
                value
            )))
        }
    }

    pub fn replace_class_name(
        &self,
        old_class_name: &str,
        new_class_name: &str,
        generated_css: &str,
    ) -> String {
        generated_css.replace(old_class_name, new_class_name)
    }

    pub fn find_grimoire_animation_name(adapted_target: &str) -> Option<&str> {
        adapted_target
            .split_whitespace()
            .find(|&target| ANIMATIONS.contains_key(target))
    }

    pub fn get_keyframe_class_from_animation(
        &self,
        animation: &str,
        animation_name: &str,
    ) -> Result<(String, String), GrimoireCSSError> {
        let mut keyframes = animation.to_string();

        if let Some(class_block_match) = self.animation_block_regex.find(&keyframes) {
            let class_block = class_block_match.as_str().to_string();
            keyframes.replace_range(class_block_match.range(), "");

            return Ok((keyframes.trim().to_string(), class_block));
        } else {
            Err(GrimoireCSSError::InvalidInput(format!(
                "No keyframes found in animation: {}",
                animation_name
            )))
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::core::{css_generator::CSSGenerator, spell::Spell, Config, GrimoireCSSError};

    #[test]
    fn test_escape_css_class_name() {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let class_name = "g!font-size=mrs(14px_16px_380px_800px);";
        let result = generator.escape_css_class_name(class_name);

        assert!(result.is_ok());
        let escaped_name = result.unwrap();

        assert_eq!(
            escaped_name,
            r"g\!font-size\=mrs\(14px\_16px\_380px\_800px\)\;"
        );
    }

    #[test]
    fn test_generate_css_class_name() {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let raw_spell = "md__{_>_p}hover:h=100px";
        let effects = "hover".to_string();
        let raw_spell_focus = "_>_p";
        let with_template = false;

        let result =
            generator.generate_css_class_name(raw_spell, &effects, raw_spell_focus, with_template);

        assert!(result.is_ok());
        let (class_name, base_name) = result.unwrap();

        assert_eq!(class_name, r".md\_\_\{\_\>\_p\}hover\:h\=100px:hover > p");
        assert_eq!(base_name, r".md\_\_\{\_\>\_p\}hover\:h\=100px");
    }

    #[test]
    fn test_generate_base_and_additional_css_g_anim() -> Result<(), GrimoireCSSError> {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let raw_spell = "md__{_>_p}hover:g-anim=bounce-in";
        let effects = "hover".to_string();
        let raw_spell_focus = "_>_p";
        let with_template = false;
        let adapted_target = "bounce-in";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            raw_spell_focus,
            with_template,
        )?;

        let result =
            generator.generate_base_and_additional_css(adapted_target, &class_name, "g-anim");

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        let expect = r#".md\_\_\{\_\>\_p\}hover\:g-anim\=bounce-in:hover > p {
  animation-duration: 0.75s;
  animation-name: bounce-in;
}"#;

        assert_eq!(base_css, expect);
        assert!(additional_css.is_some());
        assert!(additional_css.unwrap().starts_with("@keyframes bounce-in"));

        Ok(())
    }

    #[test]
    fn test_generate_base_and_additional_css_anim_n() -> Result<(), GrimoireCSSError> {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let raw_spell = "md__{_>_p}hover:anim-n=swing";
        let effects = "hover".to_string();
        let raw_spell_focus = "_>_p";
        let with_template = false;
        let adapted_target = "swing";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            raw_spell_focus,
            with_template,
        )?;

        let result = generator.generate_base_and_additional_css(
            adapted_target,
            &class_name,
            "animation-name",
        );

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(
            base_css,
            r".md\_\_\{\_\>\_p\}hover\:anim-n\=swing:hover > p{animation-name:swing;}"
        );
        assert!(additional_css.is_some());
        assert!(additional_css.unwrap().starts_with("@keyframes swing"));

        Ok(())
    }

    #[test]
    fn test_generate_base_and_additional_css_animation() -> Result<(), GrimoireCSSError> {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let raw_spell = "anim=3s_linear_wobble";
        let effects = String::new();
        let raw_spell_focus = String::new();
        let with_template = false;
        let adapted_target = "3s linear wobble";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            &raw_spell_focus,
            with_template,
        )?;

        let result =
            generator.generate_base_and_additional_css(adapted_target, &class_name, "animation");

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(
            base_css,
            r".anim\=3s\_linear\_wobble{animation:3s linear wobble;}"
        );
        assert!(additional_css.is_some());
        assert!(additional_css.unwrap().starts_with("@keyframes wobble"));

        Ok(())
    }

    #[test]
    fn test_generate_base_and_additional_css_regular_spell() -> Result<(), GrimoireCSSError> {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let raw_spell = "d=grid";
        let effects = String::new();
        let raw_spell_focus = String::new();
        let with_template = false;
        let adapted_target = "grid";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            &raw_spell_focus,
            with_template,
        )?;

        let result =
            generator.generate_base_and_additional_css(adapted_target, &class_name, "display");

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(base_css, r".d\=grid{display:grid;}");
        assert!(additional_css.is_none());

        Ok(())
    }

    #[test]
    fn test_generate_base_and_additional_css_templated_spell() -> Result<(), GrimoireCSSError> {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let raw_spell = "d=grid";
        let effects = String::new();
        let raw_spell_focus = String::new();
        let with_template = true;
        let adapted_target = "grid";

        let (class_name, _) = generator.generate_css_class_name(
            raw_spell,
            &effects,
            &raw_spell_focus,
            with_template,
        )?;

        let result =
            generator.generate_base_and_additional_css(adapted_target, &class_name, "display");

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(base_css, r".g\!d\=grid\;{display:grid;}");
        assert!(additional_css.is_none());

        Ok(())
    }

    #[test]
    fn test_handle_generic_css() {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let adapted_target = "100px";
        let css_class_name = ".test-class";
        let property = "width";

        let result = generator.handle_generic_css(adapted_target, css_class_name, property);

        assert!(result.is_ok());
        let (base_css, additional_css) = result.unwrap();

        assert_eq!(base_css, ".test-class{width:100px;}");
        assert!(additional_css.is_none());
    }

    #[test]
    fn test_wrap_base_css_with_media_query() {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let base_css = ".test-class{width:100px;}";

        let result = generator.wrap_base_css_with_media_query("sm", base_css);

        assert_eq!(
            result,
            "@media (min-width: 640px){.test-class{width:100px;}}"
        );
    }

    #[test]
    fn test_generate_css() {
        let config = Config::default();
        let generator = CSSGenerator::new(&config).unwrap();

        let spell = Spell {
            raw_spell: "bgc=pink".to_string(),
            component: "bgc".to_string(),
            component_target: "pink".to_string(),
            effects: "".to_string(),
            area: "".to_string(),
            focus: "".to_string(),
            with_template: false,
            scroll_spells: None,
        };

        let result = generator.generate_css(&spell);

        assert!(result.is_ok());

        let option_value = result.unwrap();

        assert!(option_value.is_some());

        let (css, _, _) = option_value.unwrap();

        assert_eq!(css, ".bgc\\=pink{background-color:pink;}");

        // --- COMPLEX ---

        let spell_complex = Spell {
            raw_spell: "{[data-theme='light']_p}fs=mrs(14px_16px_380px_800px)".to_string(),
            component: "fs".to_string(),
            component_target: "mrs(14px_16px_380px_800px)".to_string(),
            effects: "".to_string(),
            area: "".to_string(),
            focus: "[data-theme='light']_p".to_string(),
            with_template: true,
            scroll_spells: None,
        };

        let result = generator.generate_css(&spell_complex);

        assert!(result.is_ok());

        let option_value = result.unwrap();

        assert!(option_value.is_some());

        let (css, _, _) = option_value.unwrap();

        assert_eq!(
            css,
            r".g\!\{\[data-theme\=\'light\'\]\_p\}fs\=mrs\(14px\_16px\_380px\_800px\)\;[data-theme='light'] p{font-size:14px;}@media screen and (min-width: 380px) {.g\!\{\[data-theme\=\'light\'\]\_p\}fs\=mrs\(14px\_16px\_380px\_800px\)\;[data-theme='light'] p{font-size: calc(14px + 2 * ((100vw - 380px) / 420));}}@media screen and (min-width: 800px) {.g\!\{\[data-theme\=\'light\'\]\_p\}fs\=mrs\(14px\_16px\_380px\_800px\)\;[data-theme='light'] p{font-size: 16px;}}"
        );
    }
}
