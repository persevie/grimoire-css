//! This module provides color parsing and manipulation strictly following the
//! [CSS Color Module Level 4](https://www.w3.org/TR/css-color-4/) specification.
//!
//! Unlike SASS, which sometimes deviates or extends CSS behaviors, this module
//! implements color functions and parsing rules in accordance with the standard CSS spec,
//! ensuring predictable and interoperable color handling for web-oriented applications.
//!
//! # Overview
//! - `Color` struct: Represents an RGBA color with optional transparency.
//! - Named colors: Predefined according to the CSS spec (e.g., `"red"`, `"blue"`).
//! - CSS-like parsing: Supports hex codes, `rgb()/hsl()/hwb()` notations, etc.
//! - Color transformations: `grayscale()`, `lighten()`, `darken()`, etc. (modeled after
//!   CSS, not SASS).
//!
//! # Examples
//! ```rust
//! use grimoire_css_lib::core::Color;
//! let c = Color::try_from_str("rgb(255, 0, 0)").unwrap(); // strictly CSS parsing
//! ```

use once_cell::sync::Lazy;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};

/// A color in RGBA form, optionally with an alpha channel.
///
/// Internally, stores:
/// - `r`, `g`, `b` as 8-bit values.
/// - `a` as an f32 in [0..1].
/// - a boolean `has_alpha` indicating whether the color has transparency.
///
/// This struct offers methods to create and manipulate colors in a
/// CSS/Sass-like manner (e.g. `grayscale()`, `invert()`, `mix()`, etc.).
#[derive(Debug, Clone, Copy)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: f32,
    has_alpha: bool,
}

impl PartialEq for Color {
    fn eq(&self, other: &Self) -> bool {
        self.r == other.r
            && self.g == other.g
            && self.b == other.b
            && self.a.to_bits() == other.a.to_bits()
            && self.has_alpha == other.has_alpha
    }
}

impl Eq for Color {}

impl Hash for Color {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.r.hash(state);
        self.g.hash(state);
        self.b.hash(state);
        self.a.to_bits().hash(state);
        self.has_alpha.hash(state);
    }
}

// Named colors: According to CSS spec, these are predefined.
static NAME_TO_COLOR: Lazy<HashMap<&'static str, Color>> = Lazy::new(|| {
    let mut m = HashMap::new();

    m.insert("aliceblue", Color::new_internal(240, 248, 255, 1.0, false));
    m.insert(
        "antiquewhite",
        Color::new_internal(250, 235, 215, 1.0, false),
    );
    m.insert("aqua", Color::new_internal(0, 255, 255, 1.0, false));
    m.insert("aquamarine", Color::new_internal(127, 255, 212, 1.0, false));
    m.insert("azure", Color::new_internal(240, 255, 255, 1.0, false));
    m.insert("beige", Color::new_internal(245, 245, 220, 1.0, false));
    m.insert("bisque", Color::new_internal(255, 228, 196, 1.0, false));
    m.insert("black", Color::new_internal(0, 0, 0, 1.0, false));
    m.insert(
        "blanchedalmond",
        Color::new_internal(255, 235, 205, 1.0, false),
    );
    m.insert("blue", Color::new_internal(0, 0, 255, 1.0, false));
    m.insert("blueviolet", Color::new_internal(138, 43, 226, 1.0, false));
    m.insert("brown", Color::new_internal(165, 42, 42, 1.0, false));
    m.insert("burlywood", Color::new_internal(222, 184, 135, 1.0, false));
    m.insert("cadetblue", Color::new_internal(95, 158, 160, 1.0, false));
    m.insert("chartreuse", Color::new_internal(127, 255, 0, 1.0, false));
    m.insert("chocolate", Color::new_internal(210, 105, 30, 1.0, false));
    m.insert("coral", Color::new_internal(255, 127, 80, 1.0, false));
    m.insert(
        "cornflowerblue",
        Color::new_internal(100, 149, 237, 1.0, false),
    );
    m.insert("cornsilk", Color::new_internal(255, 248, 220, 1.0, false));
    m.insert("crimson", Color::new_internal(220, 20, 60, 1.0, false));
    m.insert("cyan", Color::new_internal(0, 255, 255, 1.0, false));
    m.insert("darkblue", Color::new_internal(0, 0, 139, 1.0, false));
    m.insert("darkcyan", Color::new_internal(0, 139, 139, 1.0, false));
    m.insert(
        "darkgoldenrod",
        Color::new_internal(184, 134, 11, 1.0, false),
    );
    m.insert("darkgray", Color::new_internal(169, 169, 169, 1.0, false));
    m.insert("darkgreen", Color::new_internal(0, 100, 0, 1.0, false));
    m.insert("darkgrey", Color::new_internal(169, 169, 169, 1.0, false));
    m.insert("darkkhaki", Color::new_internal(189, 183, 107, 1.0, false));
    m.insert("darkmagenta", Color::new_internal(139, 0, 139, 1.0, false));
    m.insert(
        "darkolivegreen",
        Color::new_internal(85, 107, 47, 1.0, false),
    );
    m.insert("darkorange", Color::new_internal(255, 140, 0, 1.0, false));
    m.insert("darkorchid", Color::new_internal(153, 50, 204, 1.0, false));
    m.insert("darkred", Color::new_internal(139, 0, 0, 1.0, false));
    m.insert("darksalmon", Color::new_internal(233, 150, 122, 1.0, false));
    m.insert(
        "darkseagreen",
        Color::new_internal(143, 188, 143, 1.0, false),
    );
    m.insert(
        "darkslateblue",
        Color::new_internal(72, 61, 139, 1.0, false),
    );
    m.insert("darkslategray", Color::new_internal(47, 79, 79, 1.0, false));
    m.insert("darkslategrey", Color::new_internal(47, 79, 79, 1.0, false));
    m.insert(
        "darkturquoise",
        Color::new_internal(0, 206, 209, 1.0, false),
    );
    m.insert("darkviolet", Color::new_internal(148, 0, 211, 1.0, false));
    m.insert("deeppink", Color::new_internal(255, 20, 147, 1.0, false));
    m.insert("deepskyblue", Color::new_internal(0, 191, 255, 1.0, false));
    m.insert("dimgray", Color::new_internal(105, 105, 105, 1.0, false));
    m.insert("dimgrey", Color::new_internal(105, 105, 105, 1.0, false));
    m.insert("dodgerblue", Color::new_internal(30, 144, 255, 1.0, false));
    m.insert("firebrick", Color::new_internal(178, 34, 34, 1.0, false));
    m.insert(
        "floralwhite",
        Color::new_internal(255, 250, 240, 1.0, false),
    );
    m.insert("forestgreen", Color::new_internal(34, 139, 34, 1.0, false));
    m.insert("fuchsia", Color::new_internal(255, 0, 255, 1.0, false));
    m.insert("gainsboro", Color::new_internal(220, 220, 220, 1.0, false));
    m.insert("ghostwhite", Color::new_internal(248, 248, 255, 1.0, false));
    m.insert("gold", Color::new_internal(255, 215, 0, 1.0, false));
    m.insert("goldenrod", Color::new_internal(218, 165, 32, 1.0, false));
    m.insert("gray", Color::new_internal(128, 128, 128, 1.0, false));
    m.insert("green", Color::new_internal(0, 128, 0, 1.0, false));
    m.insert("greenyellow", Color::new_internal(173, 255, 47, 1.0, false));
    m.insert("grey", Color::new_internal(128, 128, 128, 1.0, false));
    m.insert("honeydew", Color::new_internal(240, 255, 240, 1.0, false));
    m.insert("hotpink", Color::new_internal(255, 105, 180, 1.0, false));
    m.insert("indianred", Color::new_internal(205, 92, 92, 1.0, false));
    m.insert("indigo", Color::new_internal(75, 0, 130, 1.0, false));
    m.insert("ivory", Color::new_internal(255, 255, 240, 1.0, false));
    m.insert("khaki", Color::new_internal(240, 230, 140, 1.0, false));
    m.insert("lavender", Color::new_internal(230, 230, 250, 1.0, false));
    m.insert(
        "lavenderblush",
        Color::new_internal(255, 240, 245, 1.0, false),
    );
    m.insert("lawngreen", Color::new_internal(124, 252, 0, 1.0, false));
    m.insert(
        "lemonchiffon",
        Color::new_internal(255, 250, 205, 1.0, false),
    );
    m.insert("lightblue", Color::new_internal(173, 216, 230, 1.0, false));
    m.insert("lightcoral", Color::new_internal(240, 128, 128, 1.0, false));
    m.insert("lightcyan", Color::new_internal(224, 255, 255, 1.0, false));
    m.insert(
        "lightgoldenrodyellow",
        Color::new_internal(250, 250, 210, 1.0, false),
    );
    m.insert("lightgray", Color::new_internal(211, 211, 211, 1.0, false));
    m.insert("lightgreen", Color::new_internal(144, 238, 144, 1.0, false));
    m.insert("lightgrey", Color::new_internal(211, 211, 211, 1.0, false));
    m.insert("lightpink", Color::new_internal(255, 182, 193, 1.0, false));
    m.insert(
        "lightsalmon",
        Color::new_internal(255, 160, 122, 1.0, false),
    );
    m.insert(
        "lightseagreen",
        Color::new_internal(32, 178, 170, 1.0, false),
    );
    m.insert(
        "lightskyblue",
        Color::new_internal(135, 206, 250, 1.0, false),
    );
    m.insert(
        "lightslategray",
        Color::new_internal(119, 136, 153, 1.0, false),
    );
    m.insert(
        "lightslategrey",
        Color::new_internal(119, 136, 153, 1.0, false),
    );
    m.insert(
        "lightsteelblue",
        Color::new_internal(176, 196, 222, 1.0, false),
    );
    m.insert(
        "lightyellow",
        Color::new_internal(255, 255, 224, 1.0, false),
    );
    m.insert("lime", Color::new_internal(0, 255, 0, 1.0, false));
    m.insert("limegreen", Color::new_internal(50, 205, 50, 1.0, false));
    m.insert("linen", Color::new_internal(250, 240, 230, 1.0, false));
    m.insert("magenta", Color::new_internal(255, 0, 255, 1.0, false));
    m.insert("maroon", Color::new_internal(128, 0, 0, 1.0, false));
    m.insert(
        "mediumaquamarine",
        Color::new_internal(102, 205, 170, 1.0, false),
    );
    m.insert("mediumblue", Color::new_internal(0, 0, 205, 1.0, false));
    m.insert(
        "mediumorchid",
        Color::new_internal(186, 85, 211, 1.0, false),
    );
    m.insert(
        "mediumpurple",
        Color::new_internal(147, 112, 219, 1.0, false),
    );
    m.insert(
        "mediumseagreen",
        Color::new_internal(60, 179, 113, 1.0, false),
    );
    m.insert(
        "mediumslateblue",
        Color::new_internal(123, 104, 238, 1.0, false),
    );
    m.insert(
        "mediumspringgreen",
        Color::new_internal(0, 250, 154, 1.0, false),
    );
    m.insert(
        "mediumturquoise",
        Color::new_internal(72, 209, 204, 1.0, false),
    );
    m.insert(
        "mediumvioletred",
        Color::new_internal(199, 21, 133, 1.0, false),
    );
    m.insert("midnightblue", Color::new_internal(25, 25, 112, 1.0, false));
    m.insert("mintcream", Color::new_internal(245, 255, 250, 1.0, false));
    m.insert("mistyrose", Color::new_internal(255, 228, 225, 1.0, false));
    m.insert("moccasin", Color::new_internal(255, 228, 181, 1.0, false));
    m.insert(
        "navajowhite",
        Color::new_internal(255, 222, 173, 1.0, false),
    );
    m.insert("navy", Color::new_internal(0, 0, 128, 1.0, false));
    m.insert("oldlace", Color::new_internal(253, 245, 230, 1.0, false));
    m.insert("olive", Color::new_internal(128, 128, 0, 1.0, false));
    m.insert("olivedrab", Color::new_internal(107, 142, 35, 1.0, false));
    m.insert("orange", Color::new_internal(255, 165, 0, 1.0, false));
    m.insert("orangered", Color::new_internal(255, 69, 0, 1.0, false));
    m.insert("orchid", Color::new_internal(218, 112, 214, 1.0, false));
    m.insert(
        "palegoldenrod",
        Color::new_internal(238, 232, 170, 1.0, false),
    );
    m.insert("palegreen", Color::new_internal(152, 251, 152, 1.0, false));
    m.insert(
        "paleturquoise",
        Color::new_internal(175, 238, 238, 1.0, false),
    );
    m.insert(
        "palevioletred",
        Color::new_internal(219, 112, 147, 1.0, false),
    );
    m.insert("papayawhip", Color::new_internal(255, 239, 213, 1.0, false));
    m.insert("peachpuff", Color::new_internal(255, 218, 185, 1.0, false));
    m.insert("peru", Color::new_internal(205, 133, 63, 1.0, false));
    m.insert("pink", Color::new_internal(255, 192, 203, 1.0, false));
    m.insert("plum", Color::new_internal(221, 160, 221, 1.0, false));
    m.insert("powderblue", Color::new_internal(176, 224, 230, 1.0, false));
    m.insert("purple", Color::new_internal(128, 0, 128, 1.0, false));
    m.insert(
        "rebeccapurple",
        Color::new_internal(102, 51, 153, 1.0, false),
    );
    m.insert("red", Color::new_internal(255, 0, 0, 1.0, false));
    m.insert("rosybrown", Color::new_internal(188, 143, 143, 1.0, false));
    m.insert("royalblue", Color::new_internal(65, 105, 225, 1.0, false));
    m.insert("saddlebrown", Color::new_internal(139, 69, 19, 1.0, false));
    m.insert("salmon", Color::new_internal(250, 128, 114, 1.0, false));
    m.insert("sandybrown", Color::new_internal(244, 164, 96, 1.0, false));
    m.insert("seagreen", Color::new_internal(46, 139, 87, 1.0, false));
    m.insert("seashell", Color::new_internal(255, 245, 238, 1.0, false));
    m.insert("sienna", Color::new_internal(160, 82, 45, 1.0, false));
    m.insert("silver", Color::new_internal(192, 192, 192, 1.0, false));
    m.insert("skyblue", Color::new_internal(135, 206, 235, 1.0, false));
    m.insert("slateblue", Color::new_internal(106, 90, 205, 1.0, false));
    m.insert("slategray", Color::new_internal(112, 128, 144, 1.0, false));
    m.insert("slategrey", Color::new_internal(112, 128, 144, 1.0, false));
    m.insert("snow", Color::new_internal(255, 250, 250, 1.0, false));
    m.insert("springgreen", Color::new_internal(0, 255, 127, 1.0, false));
    m.insert("steelblue", Color::new_internal(70, 130, 180, 1.0, false));
    m.insert("tan", Color::new_internal(210, 180, 140, 1.0, false));
    m.insert("teal", Color::new_internal(0, 128, 128, 1.0, false));
    m.insert("thistle", Color::new_internal(216, 191, 216, 1.0, false));
    m.insert("tomato", Color::new_internal(255, 99, 71, 1.0, false));
    m.insert("turquoise", Color::new_internal(64, 224, 208, 1.0, false));
    m.insert("violet", Color::new_internal(238, 130, 238, 1.0, false));
    m.insert("wheat", Color::new_internal(245, 222, 179, 1.0, false));
    m.insert("white", Color::new_internal(255, 255, 255, 1.0, false));
    m.insert("whitesmoke", Color::new_internal(245, 245, 245, 1.0, false));
    m.insert("yellow", Color::new_internal(255, 255, 0, 1.0, false));
    m.insert("yellowgreen", Color::new_internal(154, 205, 50, 1.0, false));
    m.insert("transparent", Color::new_internal(0, 0, 0, 0.0, false));

    m
});

static COLOR_TO_NAME: Lazy<HashMap<Color, &'static str>> = Lazy::new(|| {
    let mut m = HashMap::new();
    for (name, color) in NAME_TO_COLOR.iter() {
        m.insert(*color, *name);
    }
    m
});

impl Color {
    // -----------------------------------------
    // Constructors
    // -----------------------------------------

    // Internal constructor ensuring correct alpha handling (private).
    fn new_internal(r: u8, g: u8, b: u8, a: f32, has_alpha: bool) -> Self {
        let a_clamped = clamp(a, 0.0, 1.0);
        // If we previously didn't have alpha and after clamp a=1.0, then no alpha.
        // If we previously had alpha or a!=1.0 now, has_alpha=true.
        let final_has_alpha = if has_alpha { true } else { a_clamped != 1.0 };

        Color {
            r,
            g,
            b,
            a: a_clamped,
            has_alpha: final_has_alpha,
        }
    }

    /// Creates a new color from RGBA components.
    ///
    /// # Arguments
    ///
    /// * `r` - Red channel (0..255).
    /// * `g` - Green channel (0..255).
    /// * `b` - Blue channel (0..255).
    /// * `a` - Alpha channel (0..1). If `a` < 1.0, color is considered to have alpha.
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let c = Color::new(255, 0, 0, 1.0); // Fully opaque red
    /// ```
    pub fn new(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::new_internal(r, g, b, a, a != 1.0)
    }

    /// Creates a color from RGB and alpha components.
    ///
    /// Equivalent to [`Color::new`], provided for clarity.
    pub fn from_rgb(r: u8, g: u8, b: u8, a: f32) -> Self {
        Self::new_internal(r, g, b, a, a != 1.0)
    }

    /// Creates a color from [HSL](https://www.w3.org/TR/css-color-4/#the-hsl-notation) values, plus alpha.
    ///
    /// # Arguments
    ///
    /// * `h` - Hue, in degrees (0..360).
    /// * `s` - Saturation, in percent (0..100).
    /// * `l` - Lightness, in percent (0..100).
    /// * `a` - Alpha channel (0..1).
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// // Pure red via HSL
    /// let c = Color::from_hsl(0.0, 100.0, 50.0, 1.0);
    /// ```
    pub fn from_hsl(h: f32, s: f32, l: f32, a: f32) -> Self {
        let h_norm = normalize_hue(h);
        let s_f = clamp(s / 100.0, 0.0, 1.0);
        let l_f = clamp(l / 100.0, 0.0, 1.0);
        let (r, g, b, a_cl) = hsl_to_srgb(h_norm, s_f, l_f, clamp(a, 0.0, 1.0));
        Self::new_internal(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            a_cl,
            a_cl != 1.0,
        )
    }

    /// Creates a color from [HWB](https://www.w3.org/TR/css-color-4/#the-hwb-notation) values, plus alpha.
    ///
    /// # Arguments
    ///
    /// * `h` - Hue in degrees (0..360).
    /// * `w` - Whiteness (0..100).
    /// * `bk` - Blackness (0..100).
    /// * `a` - Alpha channel (0..1).
    ///
    /// See [the CSS spec](https://www.w3.org/TR/css-color-4/#the-hwb-notation).
    pub fn from_hwb(h: f32, w: f32, bk: f32, a: f32) -> Self {
        // h is in degrees already
        let (r, g, b, a_cl) = Self::hwb_to_srgb(h, w, bk, a);
        Self::new_internal(
            clamp((r * 255.0).round() as u8, 0, 255),
            clamp((g * 255.0).round() as u8, 0, 255),
            clamp((b * 255.0).round() as u8, 0, 255),
            a_cl,
            a_cl != 1.0,
        )
    }

    /// Attempts to create a color from a hex string (e.g. `"#ff00ff"`, `"#fff"`).
    /// Returns `None` if the string is invalid.
    ///
    /// Supports 3-, 4-, 6-, and 8-digit hex forms (including alpha).
    pub fn from_hex(hex: &str) -> Option<Self> {
        Self::try_from_hex_str(hex)
    }

    /// Attempts to create a color from a general CSS-like string:
    /// - Named color (e.g. `"red"`, `"aliceblue"`)
    /// - Hex code (e.g. `"#fff"`, `"#ff00ff"`, `"#ffffffff"`)
    /// - Functional notation (e.g. `"rgb(...)"`, `"hsl(...)"`, `"hwb(...)"`)
    ///
    /// Returns `None` if the color cannot be parsed.
    pub fn try_from_str(input: &str) -> Option<Self> {
        let input = input.trim();

        // Try hex
        if input.starts_with('#') {
            if let Some(c) = Self::try_from_hex_str(input) {
                return Some(c);
            }
        }

        // Try functional syntax (rgb/hsl/hwb)
        if let Some(c) = Self::try_from_function_syntax(input) {
            return Some(c);
        }

        // Try named color
        if let Some(c) = Self::try_from_named_str(input) {
            return Some(c);
        }

        None
    }

    // -----------------------------------------
    // Public getters
    // -----------------------------------------

    /// Returns the red channel (0..255)
    pub fn red(&self) -> u8 {
        self.r
    }

    /// Returns the green channel (0..255)
    pub fn green(&self) -> u8 {
        self.g
    }

    /// Returns the blue channel (0..255)
    pub fn blue(&self) -> u8 {
        self.b
    }

    /// Returns the alpha channel (0..1)
    pub fn alpha(&self) -> f32 {
        self.a
    }

    /// Alias for `alpha()`
    pub fn opacity(&self) -> f32 {
        self.a
    }

    // -----------------------------------------
    // Color space conversions
    // -----------------------------------------

    /// Converts the current color to [HSL](https://www.w3.org/TR/css-color-4/#the-hsl-notation) and
    /// returns `(h, s, l)` where:
    /// - `h` is in [0..360] degrees,
    /// - `s` and `l` are in [0..100] percent.
    ///
    /// This does not change the alpha channel.
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let c = Color::new(255, 0, 0, 1.0);
    /// let (h, s, l) = c.to_hsl();
    /// assert_eq!(h, 0.0);
    /// assert_eq!(s, 100.0);
    /// assert_eq!(l, 50.0);
    /// ```
    pub fn to_hsl(&self) -> (f32, f32, f32) {
        Self::rgb_to_hsl(*self)
    }

    /// Returns `(r,g,b,a)` with:
    /// - `r,g,b` in [0..255]
    /// - `a` in [0..1]
    pub fn to_rgba(&self) -> (u8, u8, u8, f32) {
        (self.r, self.g, self.b, self.a)
    }

    /// Returns a CSS hex string (e.g. `"#7fffd4"` or `"#7fffd480"` if alpha < 1.0).
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let c = Color::new(127, 255, 212, 1.0);
    /// assert_eq!(c.to_hex_string(), "#7fffd4");
    /// ```
    pub fn to_hex_string(&self) -> String {
        if self.has_alpha {
            let a = (self.a * 255.0).round() as u8;
            format!("#{:02x}{:02x}{:02x}{:02x}", self.r, self.g, self.b, a)
        } else {
            format!("#{:02x}{:02x}{:02x}", self.r, self.g, self.b)
        }
    }

    /// Returns the named color string (e.g. `"red"`, `"blue"`) if this color
    /// matches one of the predefined colors exactly, otherwise `None`.
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let c = Color::new(255, 0, 0, 1.0);
    /// assert_eq!(c.to_named_color_str(), Some("red"));
    /// ```
    pub fn to_named_color_str(&self) -> Option<&'static str> {
        COLOR_TO_NAME.get(self).copied()
    }

    // -----------------------------------------
    // Color operations
    // -----------------------------------------

    /// Converts the color to grayscale by setting saturation to 0%.
    ///
    /// Other channels (hue, lightness, alpha) remain unchanged.
    pub fn grayscale(&self) -> Self {
        let (h, _s, l) = self.to_hsl();
        Self::from_hsl(h, 0.0, l, self.a)
    }

    /// Returns the complementary color by adding 180° to the hue.
    pub fn complement(&self) -> Self {
        let (h, s, l) = self.to_hsl();
        Self::from_hsl(h + 180.0, s, l, self.a)
    }

    /// Inverts the color.
    ///
    /// `weight` controls how much the color is inverted (0..100%, defaults to 100).
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let white = Color::new(255, 255, 255, 1.0);
    /// let black = white.invert(None);
    /// assert_eq!(black.to_hex_string(), "#000000");
    /// ```
    pub fn invert(&self, weight: Option<f32>) -> Self {
        let w = weight.unwrap_or(100.0) / 100.0;
        let inv_r = 255 - self.r;
        let inv_g = 255 - self.g;
        let inv_b = 255 - self.b;

        let r = (self.r as f32 * (1.0 - w) + inv_r as f32 * w).round() as u8;
        let g = (self.g as f32 * (1.0 - w) + inv_g as f32 * w).round() as u8;
        let b = (self.b as f32 * (1.0 - w) + inv_b as f32 * w).round() as u8;

        Color::new_internal(r, g, b, self.a, self.has_alpha || self.a != 1.0)
    }

    /// Mixes two colors by a given weight (0..100%).
    ///
    /// A `weight` of 50% returns an average of the two colors.
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let red = Color::new(255, 0, 0, 1.0);
    /// let blue = Color::new(0, 0, 255, 1.0);
    /// let purple = Color::mix(red, blue, 50.0);
    /// assert_eq!(purple.to_hex_string(), "#800080");
    /// ```
    pub fn mix(c1: Color, c2: Color, weight: f32) -> Self {
        let w = clamp(weight, 0.0, 100.0) / 100.0;
        let r = (c1.r as f32 * w + c2.r as f32 * (1.0 - w)).round() as u8;
        let g = (c1.g as f32 * w + c2.g as f32 * (1.0 - w)).round() as u8;
        let b = (c1.b as f32 * w + c2.b as f32 * (1.0 - w)).round() as u8;
        let a = c1.a * w + c2.a * (1.0 - w);
        // If either had alpha originally, result should keep has_alpha = true if a != 1.0
        let has_alpha = c1.has_alpha || c2.has_alpha || a != 1.0;
        Color::new_internal(r, g, b, a, has_alpha)
    }

    /// Adjusts the hue by `degrees`.
    ///
    /// If `degrees` is positive, hue rotates “forward”;
    /// if negative, it rotates “backward”. Values can wrap beyond 360°.
    pub fn adjust_hue(&self, degrees: f32) -> Self {
        let (h, s, l) = Self::rgb_to_hsl(*self);
        Self::from_hsl(h + degrees, s, l, self.a)
    }

    /// Adjusts color by optionally modifying RGB deltas or HSL deltas.
    ///
    /// # Arguments
    ///
    /// * `red_delta`, `green_delta`, `blue_delta` - The integer deltas to add to each channel.
    /// * `hue_delta`, `sat_delta`, `light_delta`, `alpha_delta` - The float deltas for hue, saturation, lightness, alpha.
    ///
    /// Missing arguments (i.e. `None`) leave that component unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let c = Color::new(128, 128, 128, 1.0);
    /// // Make it slightly redder
    /// let c2 = c.adjust_color(Some(10), None, None, None, None, None, None);
    /// ```
    #[allow(clippy::too_many_arguments)]
    pub fn adjust_color(
        &self,
        red_delta: Option<i32>,
        green_delta: Option<i32>,
        blue_delta: Option<i32>,
        hue_delta: Option<f32>,
        sat_delta: Option<f32>,
        light_delta: Option<f32>,
        alpha_delta: Option<f32>,
    ) -> Self {
        let (h, s, l) = self.to_hsl();
        let mut r = self.r as i32;
        let mut g = self.g as i32;
        let mut b = self.b as i32;

        if let Some(rd) = red_delta {
            r = clamp(r + rd, 0, 255);
        }
        if let Some(gd) = green_delta {
            g = clamp(g + gd, 0, 255);
        }
        if let Some(bd) = blue_delta {
            b = clamp(b + bd, 0, 255);
        }

        let h_new = h + hue_delta.unwrap_or(0.0);
        let s_new = s + sat_delta.unwrap_or(0.0);
        let l_new = l + light_delta.unwrap_or(0.0);
        let a_new = clamp(self.a + alpha_delta.unwrap_or(0.0), 0.0, 1.0);

        let mut out_color = Self::from_hsl(h_new, s_new, l_new, a_new);
        out_color.r = r as u8;
        out_color.g = g as u8;
        out_color.b = b as u8;
        out_color.has_alpha = out_color.has_alpha || a_new != 1.0;
        out_color
    }

    /// Changes color by setting absolute values (if provided) for RGB or HSL.
    ///
    /// # Arguments
    ///
    /// * `red`, `green`, `blue` - Final values for each channel (0..255).
    /// * `hue_val`, `sat_val`, `light_val` - Final HSL values.
    /// * `alpha_val` - Final alpha in [0..1].
    ///
    /// Missing arguments (i.e. `None`) leave that component unchanged.
    #[allow(clippy::too_many_arguments)]
    pub fn change_color(
        &self,
        red: Option<u8>,
        green: Option<u8>,
        blue: Option<u8>,
        hue_val: Option<f32>,
        sat_val: Option<f32>,
        light_val: Option<f32>,
        alpha_val: Option<f32>,
    ) -> Self {
        let (h, s, l) = self.to_hsl();

        let r = red.unwrap_or(self.r);
        let g = green.unwrap_or(self.g);
        let b = blue.unwrap_or(self.b);

        let h_new = hue_val.unwrap_or(h);
        let s_new = sat_val.unwrap_or(s);
        let l_new = light_val.unwrap_or(l);
        let a_new = clamp(alpha_val.unwrap_or(self.a), 0.0, 1.0);

        let mut out_color = Self::from_hsl(h_new, s_new, l_new, a_new);

        if red.is_some() {
            out_color.r = r;
        }
        if green.is_some() {
            out_color.g = g;
        }
        if blue.is_some() {
            out_color.b = b;
        }

        out_color.has_alpha = out_color.has_alpha || a_new != 1.0;
        out_color
    }

    /// Scales color channels by given percentages.
    ///
    /// Positive scale values increase the channel, negative decrease.
    /// E.g. `red_scale=10.0` => +10% red from current value.
    ///
    /// Missing arguments (i.e. `None`) leave that channel unchanged.
    pub fn scale_color(
        &self,
        red_scale: Option<f32>,
        green_scale: Option<f32>,
        blue_scale: Option<f32>,
        saturation_scale: Option<f32>,
        lightness_scale: Option<f32>,
        alpha_scale: Option<f32>,
    ) -> Self {
        let (mut h, mut s, mut l) = self.to_hsl();
        let mut a = self.a;

        if let Some(ss) = saturation_scale {
            s = scale_hsl(s, ss);
            s = clamp(s, 0.0, 100.0);
        }

        if let Some(ls) = lightness_scale {
            l = scale_hsl(l, ls);
            l = clamp(l, 0.0, 100.0);
        }

        if let Some(as_) = alpha_scale {
            a = scale_alpha(a, as_);
        }

        h = normalize_hue(h);
        let mut new_color = Self::from_hsl(h, s, l, a);

        if let Some(rs) = red_scale {
            new_color.r = scale_channel(new_color.r, rs);
        }
        if let Some(gs) = green_scale {
            new_color.g = scale_channel(new_color.g, gs);
        }
        if let Some(bs) = blue_scale {
            new_color.b = scale_channel(new_color.b, bs);
        }

        new_color.has_alpha = new_color.has_alpha || a != 1.0;
        new_color
    }

    /// Returns a new color with the same RGB, but alpha set to `alpha`.
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let c = Color::new(255, 0, 0, 1.0);
    /// let half_transparent_red = c.rgba(0.5);
    /// ```
    pub fn rgba(&self, alpha: f32) -> Self {
        let new_a = clamp(alpha, 0.0, 1.0);
        // Once alpha introduced, keep has_alpha true
        let new_has_alpha = self.has_alpha || new_a != 1.0;
        Self::new_internal(self.r, self.g, self.b, new_a, new_has_alpha)
    }

    /// Lightens the color by `amount` percent.
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let red = Color::new(255, 0, 0, 1.0);
    /// let lighter_red = red.lighten(10.0); // ~ #ff3333
    /// ```
    pub fn lighten(&self, amount: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        Self::from_hsl(h, s, clamp(l + amount, 0.0, 100.0), self.a)
    }

    /// Darkens the color by `amount` percent.
    pub fn darken(&self, amount: f32) -> Self {
        let (h, s, l) = self.to_hsl();
        Self::from_hsl(h, s, clamp(l - amount, 0.0, 100.0), self.a)
    }

    /// Increases saturation by `amount` percent
    pub fn saturate(&self, amount: f32) -> Self {
        let (h, s, l) = Self::rgb_to_hsl(*self);
        Self::from_hsl(h, clamp(s + amount, 0.0, 100.0), l, self.a)
    }

    /// Decreases saturation by `amount` percent
    pub fn desaturate(&self, amount: f32) -> Self {
        let (h, s, l) = Self::rgb_to_hsl(*self);
        Self::from_hsl(h, clamp(s - amount, 0.0, 100.0), l, self.a)
    }

    /// Increases alpha by `amount` in [0..1], making the color more opaque.
    ///
    /// # Examples
    ///
    /// ```
    /// use grimoire_css_lib::core::Color;
    ///
    /// let mut c = Color::new(255, 0, 0, 0.5);
    /// c = c.opacify(0.3);  // new alpha = 0.8
    /// ```
    pub fn opacify(&self, amount: f32) -> Self {
        let new_a = clamp(self.a + amount, 0.0, 1.0);
        Self::new_internal(
            self.r,
            self.g,
            self.b,
            new_a,
            self.has_alpha || new_a != 1.0,
        )
    }

    /// Alias for `opacify`
    pub fn fade_in(&self, amount: f32) -> Self {
        self.opacify(amount)
    }

    /// Decreases alpha (increases transparency) by `amount` (0..1)
    pub fn transparentize(&self, amount: f32) -> Self {
        let new_a = clamp(self.a - amount, 0.0, 1.0);
        Self::new_internal(
            self.r,
            self.g,
            self.b,
            new_a,
            self.has_alpha || new_a != 1.0,
        )
    }

    /// Alias for `transparentize`
    pub fn fade_out(&self, amount: f32) -> Self {
        self.transparentize(amount)
    }

    // -----------------------------------------
    // Private helper methods
    // -----------------------------------------

    fn try_from_hex_str(s: &str) -> Option<Self> {
        let hex = s.trim_start_matches('#');
        match hex.len() {
            3 => {
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                Some(Self::new_internal(r, g, b, 1.0, false))
            }
            4 => {
                // #RGBA is allowed in CSS Color Level 4
                let r = u8::from_str_radix(&hex[0..1].repeat(2), 16).ok()?;
                let g = u8::from_str_radix(&hex[1..2].repeat(2), 16).ok()?;
                let b = u8::from_str_radix(&hex[2..3].repeat(2), 16).ok()?;
                let a = u8::from_str_radix(&hex[3..4].repeat(2), 16).ok()? as f32 / 255.0;
                Some(Self::new_internal(r, g, b, a, true))
            }
            6 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                Some(Self::new_internal(r, g, b, 1.0, false))
            }
            8 => {
                let r = u8::from_str_radix(&hex[0..2], 16).ok()?;
                let g = u8::from_str_radix(&hex[2..4], 16).ok()?;
                let b = u8::from_str_radix(&hex[4..6], 16).ok()?;
                let a = u8::from_str_radix(&hex[6..8], 16).ok()? as f32 / 255.0;
                Some(Self::new_internal(r, g, b, a, true))
            }
            _ => None,
        }
    }

    fn try_from_named_str(name: &str) -> Option<Self> {
        let lower = name.to_ascii_lowercase();
        NAME_TO_COLOR.get(lower.as_str()).copied()
    }

    fn try_from_function_syntax(s: &str) -> Option<Self> {
        let lower = s.to_ascii_lowercase();
        let input = lower.as_str().trim();

        if input.starts_with("rgb(") || input.starts_with("rgba(") {
            Self::try_from_rgb_func(input)
        } else if input.starts_with("hsl(") || input.starts_with("hsla(") {
            Self::try_from_hsl_func(input)
        } else if input.starts_with("hwb(") {
            Self::try_from_hwb_func(input)
        } else {
            None
        }
    }

    fn try_from_rgb_func(input: &str) -> Option<Self> {
        let mut s = input.trim();
        let mut has_alpha = false;
        if s.starts_with("rgba(") {
            s = &s[5..];
        } else {
            s = &s[4..];
        }
        if !s.ends_with(')') {
            return None;
        }
        s = s[..s.len() - 1].trim();

        let legacy = s.contains(',');

        let (r_val, mut s, rperc) = parse_number_or_percentage(s)?;
        if !consume_divider_required(&mut s, legacy) {
            return None;
        }
        let (g_val, mut s, gperc) = parse_number_or_percentage(s)?;
        if !consume_divider_required(&mut s, legacy) {
            return None;
        }
        let (b_val, s, bperc) = parse_number_or_percentage(s)?;

        // According to CSS:
        // If percentage => fraction * 255
        // If number => directly in 0..255
        let r = if rperc {
            (r_val * 255.0).round() as u8
        } else {
            clamp(r_val, 0.0, 255.0).round() as u8
        };
        let g = if gperc {
            (g_val * 255.0).round() as u8
        } else {
            clamp(g_val, 0.0, 255.0).round() as u8
        };
        let b = if bperc {
            (b_val * 255.0).round() as u8
        } else {
            clamp(b_val, 0.0, 255.0).round() as u8
        };

        let mut a = 1.0;

        let s = s.trim();
        if (!legacy && s.starts_with('/')) || (legacy && s.starts_with(',')) {
            has_alpha = true;
            let mut s2 = consume_divider(s, legacy);
            if let Some((val, rest)) = parse_alpha_value(s2) {
                a = val;
                s2 = rest;
                if !s2.trim().is_empty() {
                    return None;
                }
            } else if s2.to_lowercase().starts_with("none") {
                s2 = s2[4..].trim();
                if !s2.is_empty() {
                    return None;
                }
            } else {
                return None;
            }
        } else if !s.is_empty() {
            return None;
        }

        Some(Color::new_internal(r, g, b, a, has_alpha || a != 1.0))
    }

    fn try_from_hsl_func(input: &str) -> Option<Self> {
        let mut s = input.trim();
        let mut has_alpha = false;
        if s.starts_with("hsla(") {
            s = &s[5..];
        } else {
            s = &s[4..];
        }
        if !s.ends_with(')') {
            return None;
        }
        s = s[..s.len() - 1].trim();

        let legacy = s.contains(',');

        let (h_val, mut s) = parse_hue_or_none(s)?;
        if legacy && !consume_legacy_comma(&mut s) {
            return None;
        }
        let (sat_val, mut s) = parse_sat_light_none(s)?;
        if legacy && !consume_legacy_comma(&mut s) {
            return None;
        }
        let (l_val, s) = parse_sat_light_none(s)?;

        let mut a = 1.0;
        let s = s.trim();
        if (!legacy && s.starts_with('/')) || (legacy && s.starts_with(',')) {
            has_alpha = true;
            let mut s2 = consume_divider(s, legacy);
            if let Some((val, rest)) = parse_alpha_value(s2) {
                a = val;
                s2 = rest;
                if !s2.trim().is_empty() {
                    return None;
                }
            } else if s2.to_lowercase().starts_with("none") {
                s2 = s2[4..].trim();
                if !s2.is_empty() {
                    return None;
                }
                // none means a=1.0 but alpha specified => has_alpha=true
            } else {
                return None;
            }
        } else if !s.is_empty() {
            return None;
        }

        let h = normalize_hue(h_val);
        let s_frac = clamp(sat_val, 0.0, 1.0);
        let l_frac = clamp(l_val, 0.0, 1.0);

        let (r, g, b, a_cl) = hsl_to_srgb(h, s_frac, l_frac, a);
        Some(Color::new_internal(
            (r * 255.0).round() as u8,
            (g * 255.0).round() as u8,
            (b * 255.0).round() as u8,
            a_cl,
            has_alpha || a_cl != 1.0,
        ))
    }

    fn try_from_hwb_func(input: &str) -> Option<Self> {
        let mut s = input.trim();
        let mut has_alpha = false;
        s = &s[4..]; // skip "hwb("
        if !s.ends_with(')') {
            return None;
        }
        s = s[..s.len() - 1].trim();

        let (h_val, s) = parse_hue_or_none(s)?;
        let (w_val, s) = parse_wb_none(s)?;
        let (b_val, s) = parse_wb_none(s)?;

        let mut a = 1.0;
        let s = s.trim();

        if let Some(stripped) = s.strip_prefix('/') {
            has_alpha = true;
            let mut s2 = stripped.trim();
            if let Some((val, rest)) = parse_alpha_value(s2) {
                a = val;
                s2 = rest;
                if !s2.trim().is_empty() {
                    return None;
                }
            } else if s2.to_lowercase().starts_with("none") {
                s2 = s2[4..].trim();
                if !s2.is_empty() {
                    return None;
                }
            } else {
                return None;
            }
        } else if !s.is_empty() {
            return None;
        }

        let h = normalize_hue(h_val);
        let w = clamp(w_val, 0.0, 1.0);
        let bk = clamp(b_val, 0.0, 1.0);

        let (r, g, b, a_cl) = Self::hwb_to_srgb(h, w, bk, a);
        Some(Color::new_internal(
            clamp((r * 255.0).round() as u8, 0, 255),
            clamp((g * 255.0).round() as u8, 0, 255),
            clamp((b * 255.0).round() as u8, 0, 255),
            a_cl,
            has_alpha || a_cl != 1.0,
        ))
    }

    // -----------------------------------------
    // Private conversion functions
    // -----------------------------------------

    fn rgb_to_hsl(c: Color) -> (f32, f32, f32) {
        let r = c.r as f32 / 255.0;
        let g = c.g as f32 / 255.0;
        let b = c.b as f32 / 255.0;

        let max = r.max(g).max(b);
        let min = r.min(g).min(b);
        let delta = max - min;

        let l = (max + min) / 2.0;

        if delta.abs() < f32::EPSILON {
            (0.0, 0.0, l * 100.0)
        } else {
            let s = if l > 0.5 {
                delta / (2.0 - max - min)
            } else {
                delta / (max + min)
            };

            let h = if (max - r).abs() < f32::EPSILON {
                (g - b) / delta + if g < b { 6.0 } else { 0.0 }
            } else if (max - g).abs() < f32::EPSILON {
                (b - r) / delta + 2.0
            } else {
                (r - g) / delta + 4.0
            };

            (normalize_hue(h * 60.0), s * 100.0, l * 100.0)
        }
    }

    fn hwb_to_srgb(h: f32, mut w: f32, mut bk: f32, a: f32) -> (f32, f32, f32, f32) {
        if w + bk > 1.0 {
            let sum = w + bk;
            w /= sum;
            bk /= sum;
        }

        let (base_r, base_g, base_b, _) = hsl_to_srgb(h, 1.0, 0.5, 1.0);
        let r = base_r * (1.0 - w - bk) + w;
        let g = base_g * (1.0 - w - bk) + w;
        let b = base_b * (1.0 - w - bk) + w;

        (r, g, b, a)
    }
}

// -----------------------------------------
// Global helper functions
// -----------------------------------------

fn clamp<T: PartialOrd>(value: T, min_val: T, max_val: T) -> T {
    if value < min_val {
        min_val
    } else if value > max_val {
        max_val
    } else {
        value
    }
}

fn normalize_hue(h: f32) -> f32 {
    let mut hue = h % 360.0;
    if hue < 0.0 {
        hue += 360.0;
    }
    hue
}

// Converts HSL (h in degrees, s,l in 0..1) to sRGB (0..1)
fn hsl_to_srgb(h: f32, s: f32, l: f32, a: f32) -> (f32, f32, f32, f32) {
    let t2 = if l <= 0.5 {
        l * (s + 1.)
    } else {
        l + s - l * s
    };
    let t1 = 2. * l - t2;
    let hue2rgb = |mut h: f32| {
        if h < 0.0 {
            h += 1.0;
        } else if h > 1.0 {
            h -= 1.0;
        }
        if 6.0 * h < 1.0 {
            t1 + (t2 - t1) * 6.0 * h
        } else if 2.0 * h < 1.0 {
            t2
        } else if 3.0 * h < 2.0 {
            t1 + (t2 - t1) * (2.0 / 3.0 - h) * 6.0
        } else {
            t1
        }
    };

    let r = hue2rgb(h / 360.0 + 1.0 / 3.0);
    let g = hue2rgb(h / 360.0);
    let b = hue2rgb(h / 360.0 - 1.0 / 3.0);
    (r, g, b, a)
}

// -----------------------------------------
// Parsing utilities
// -----------------------------------------

fn parse_number(s: &str) -> Option<(f32, &str)> {
    let s = s.trim();
    let mut end = 0;
    let chars: Vec<char> = s.chars().collect();
    if chars.is_empty() {
        return None;
    }

    if chars[end] == '+' || chars[end] == '-' {
        end += 1;
    }

    let mut has_digit = false;
    while end < chars.len() && chars[end].is_ascii_digit() {
        has_digit = true;
        end += 1;
    }

    if end < chars.len() && chars[end] == '.' {
        end += 1;
        while end < chars.len() && chars[end].is_ascii_digit() {
            has_digit = true;
            end += 1;
        }
    }

    if end < chars.len() && (chars[end] == 'e' || chars[end] == 'E') {
        end += 1;
        if end < chars.len() && (chars[end] == '+' || chars[end] == '-') {
            end += 1;
        }
        let start_exp = end;
        while end < chars.len() && chars[end].is_ascii_digit() {
            end += 1;
        }
        if end == start_exp {
            return None;
        }
    }

    if !has_digit {
        return None;
    }

    let val_str: String = chars[..end].iter().collect();
    let val: f32 = val_str.parse().ok()?;
    Some((val, s[end..].trim_start()))
}

fn parse_percentage(s: &str) -> Option<(f32, &str)> {
    let (val, rest) = parse_number(s)?;
    let rest = rest.trim_start();

    rest.strip_prefix('%')
        .map(|stripped| (val / 100.0, stripped.trim_start()))
}

fn parse_number_or_percentage(s: &str) -> Option<(f32, &str, bool)> {
    if let Some((val, rest)) = parse_percentage(s) {
        // perc = true means the value is now a fraction of 1
        Some((val, rest, true))
    } else if let Some((val, rest)) = parse_number(s) {
        // perc = false means the value is as-is (could be fraction like 0.5 or just a number)
        Some((val, rest, false))
    } else {
        None
    }
}

fn parse_alpha_value(s: &str) -> Option<(f32, &str)> {
    if let Some((val, rest, _)) = parse_number_or_percentage(s) {
        Some((clamp(val, 0.0, 1.0), rest))
    } else {
        None
    }
}

fn consume_divider(s: &str, legacy: bool) -> &str {
    let s = s.trim_start();
    if !s.is_empty() {
        let c = s.chars().next().unwrap();

        if c == ',' && legacy || c == '/' && !legacy {
            return s[1..].trim_start();
        }
    }
    s
}

fn consume_divider_required(s: &mut &str, legacy: bool) -> bool {
    let original = *s;
    *s = consume_divider(s, legacy);
    if *s == original {
        !legacy
    } else {
        true
    }
}

fn consume_legacy_comma(s: &mut &str) -> bool {
    let ss = s.trim_start();
    if let Some(stripped) = ss.strip_prefix(',') {
        *s = stripped.trim_start();
        true
    } else {
        false
    }
}

// Always returns hue in degrees:
fn parse_hue_or_none(s: &str) -> Option<(f32, &str)> {
    let s = s.trim();
    if s.to_lowercase().starts_with("none") {
        return Some((0.0, &s[4..]));
    }

    let (val, rest) = parse_number(s)?;
    let rest = rest.trim_start();
    // According to CSS:
    // If unit is deg => val is degrees
    // If grad => val*(360/400)=val*0.9 degrees
    // If rad => val*(180/pi) degrees
    // If turn => val*360 degrees
    // If no unit => degrees by default
    if let Some(stripped) = rest.strip_prefix("deg") {
        Some((val, stripped))
    } else if let Some(stripped) = rest.strip_prefix("grad") {
        Some((val * (360.0 / 400.0), stripped))
    } else if let Some(stripped) = rest.strip_prefix("rad") {
        Some((val * (180.0 / std::f32::consts::PI), stripped))
    } else if let Some(stripped) = rest.strip_prefix("turn") {
        Some((val * 360.0, stripped))
    } else if is_ident_start(rest) {
        None
    } else {
        // no unit means degrees
        Some((val, rest))
    }
}

fn is_ident_start(s: &str) -> bool {
    if s.is_empty() {
        return false;
    }
    let c = s.chars().next().unwrap();
    c.is_ascii_alphabetic() || c == '-' || c == '_'
}

fn parse_sat_light_none(s: &str) -> Option<(f32, &str)> {
    let s = s.trim();
    if s.to_lowercase().starts_with("none") {
        return Some((0.0, &s[4..]));
    }
    if let Some((val, rest, _perc)) = parse_number_or_percentage(s) {
        // If perc is true, val is fraction of 1 (e.g. 50% -> val=0.5)
        // For HSL s/l in functional syntax:
        // - If percentage is given, we have fraction (0..1) already correct.
        // - If no unit is given, modern syntax also expects 0..1.
        // So we can return val as is, since both perc=true and perc=false
        // should yield a fraction 0..1 for s/l in modern syntax.
        // If the user gave a large number, it will be clamped later.
        Some((val, rest))
    } else {
        None
    }
}

fn parse_wb_none(s: &str) -> Option<(f32, &str)> {
    let s = s.trim();
    if s.to_lowercase().starts_with("none") {
        return Some((0.0, &s[4..]));
    }
    if let Some((val, rest, _perc)) = parse_number_or_percentage(s) {
        // w,b also treated as fractions 0..1
        Some((val, rest))
    } else {
        None
    }
}

// -----------------------------------------
// Additional utilities
// -----------------------------------------

fn scale_channel(val: u8, scale: f32) -> u8 {
    let val_f = val as f32;
    if scale > 0.0 {
        let diff = 255.0 - val_f;
        (val_f + diff * (scale / 100.0)).round().clamp(0.0, 255.0) as u8
    } else {
        let diff = val_f;
        (val_f - diff * (-scale / 100.0)).round().clamp(0.0, 255.0) as u8
    }
}

fn scale_hsl(val: f32, scale: f32) -> f32 {
    if scale > 0.0 {
        let diff = 100.0 - val;
        val + diff * (scale / 100.0)
    } else {
        let diff = val;
        val - diff * (-scale / 100.0)
    }
}

fn scale_alpha(val: f32, scale: f32) -> f32 {
    if scale > 0.0 {
        let diff = 1.0 - val;
        clamp(val + diff * (scale / 100.0), 0.0, 1.0)
    } else {
        let diff = val;
        clamp(val - diff * (-scale / 100.0), 0.0, 1.0)
    }
}

// -----------------------------------------
// Tests
// -----------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    // According to CSS spec, named colors must map correctly.
    // Test if we correctly parse and reverse-lookup named colors.
    #[test]
    fn test_named_colors_css_spec() {
        let c = Color::try_from_str("aliceblue").unwrap();
        assert_eq!(c.to_hex_string(), "#f0f8ff"); // aliceblue is #F0F8FF by CSS
        assert_eq!(c.to_named_color_str(), Some("aliceblue"));

        let c2 = Color::try_from_str("black").unwrap();
        assert_eq!(c2.to_hex_string(), "#000000");
        assert_eq!(c2.to_named_color_str(), Some("black"));
    }

    // Test hex parsing according to CSS:
    // - #RGB and #RRGGBB forms
    // - #RGBA and #RRGGBBAA forms for alpha
    #[test]
    fn test_hex_parsing_css_spec() {
        // #fff should equal #ffffff with a=1.0 and no alpha in output (has_alpha=false)
        let c = Color::try_from_str("#fff").unwrap();
        assert_eq!(c.to_hex_string(), "#ffffff");
        assert!(!c.has_alpha);

        // #ffffff is fully opaque white
        let c = Color::try_from_str("#ffffff").unwrap();
        assert_eq!(c.to_hex_string(), "#ffffff");

        // #ffff means #ffffff with alpha=1.0 (this is not a standard CSS form, but #FFF vs #FFFFFF are)
        // Actually, the CSS spec allows #RRGGBBAA in Level 4.
        // Let's test #RRGGBBAA: #ffffffff => white with alpha=1.0
        // This is valid in modern CSS (CSS Color Level 4).
        let c = Color::try_from_str("#ffffffff").unwrap();
        assert_eq!(c.to_hex_string(), "#ffffffff");
        // Since we parsed an 8-digit hex, has_alpha should be true
        assert!(c.has_alpha);
    }

    // Test rgb(...) parsing (according to CSS):
    #[test]
    fn test_rgb_parsing_css_spec() {
        // rgb(255, 0, 255) => #ff00ff (legacy syntax with commas)
        let c = Color::try_from_str("rgb(255, 0, 255)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff00ff");
        assert!(!c.has_alpha);

        // Modern syntax: rgb(100% 0% 100%)
        // 100% of 255 = 255, so this is also #ff00ff
        let c = Color::try_from_str("rgb(100% 0% 100%)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff00ff");

        // With alpha: rgb(128 128 128 / 0.5)
        // 0.5 means alpha=0.5
        // This should produce a mid-gray with half transparency
        let c = Color::try_from_str("rgb(128 128 128 / 0.5)").unwrap();
        assert_eq!(c.to_hex_string(), "#80808080");
        assert!(c.has_alpha);

        // Percentage alpha: rgb(0% 0% 100% / 50%)
        // 0% = 0, 100% = 255, alpha=0.5
        let c = Color::try_from_str("rgb(0% 0% 100% / 50%)").unwrap();
        assert_eq!(c.to_hex_string(), "#0000ff80");
        assert!(c.has_alpha);
    }

    // Test hsl(...) parsing according to CSS:
    // hsl(h, s%, l%) or hsl(h s l / a)
    // If s,l given as percentages, they are s/100 and l/100 for computations.
    // If no unit, modern syntax expects fractions in [0..1].
    #[test]
    fn test_hsl_parsing_css_spec() {
        // hsl(0, 100%, 50%) = pure red #ff0000
        let c = Color::try_from_str("hsl(0,100%,50%)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff0000");

        // Modern syntax: hsl(0 1 0.5) same as hsl(0deg,100%,50%)
        let c = Color::try_from_str("hsl(0 1 0.5)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff0000");

        // With alpha: hsla(0,100%,50%,0.5)
        let c = Color::try_from_str("hsla(0,100%,50%,0.5)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff000080");
        assert!(c.has_alpha);

        // Modern syntax with slash: hsl(0 1 0.5 / 0.5)
        let c = Color::try_from_str("hsl(0 1 0.5 / 0.5)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff000080");
        assert!(c.has_alpha);
    }

    // Test hwb(...) parsing according to CSS:
    // hwb(h w b / a)
    // w,b are fractions 0..1 (either given as percent or fraction)
    // h is a hue in degrees.
    #[test]
    fn test_hwb_parsing_css_spec() {
        // hwb(0 0% 0%) = pure red (#ff0000)
        let c = Color::try_from_str("hwb(0 0% 0%)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff0000");

        // Modern syntax: hwb(0 0 0) same as above if we treat 0 as fraction 0.0
        let c = Color::try_from_str("hwb(0 0 0)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff0000");

        // With alpha: hwb(0 0 0 / 0.5)
        let c = Color::try_from_str("hwb(0 0 0 / 0.5)").unwrap();
        assert_eq!(c.to_hex_string(), "#ff000080");
        assert!(c.has_alpha);
    }

    // Test hue normalization and clamping behavior:
    // Hue should wrap around: hsl(-30,100%,50%) = hsl(330,100%,50%)
    #[test]
    fn test_hue_normalization() {
        let c = Color::try_from_str("hsl(-30,100%,50%)").unwrap();
        // hsl(330,100%,50%) is a magenta-ish color (#ff00bf approximately)
        // Let's just check hue: expected #ff00bf or close
        // Since this code uses precise calculations, let's verify hex:
        // hsl_to_srgb(330°,1,0.5) => hue2rgb computations
        // At 330°, we get a color close to #ff00bf indeed.
        let hex = c.to_hex_string();
        // Accept a close match because floating math:
        // We can just assert hue & channel correctness:
        let (h, _, _) = c.to_hsl();
        assert!(
            (h.round() - 330.0).abs() < 0.1,
            "Hue should normalize to ~330, got {}",
            h.round()
        );
        // Check approximate color:
        assert_eq!(hex, "#ff007f");
    }

    // Test that alpha is handled correctly:
    // If alpha is 1.0 and was not specified initially, has_alpha=false.
    // If alpha is changed, has_alpha should become true if alpha != 1.0.
    #[test]
    fn test_alpha_handling() {
        let c = Color::try_from_str("#ff0000").unwrap();
        assert_eq!(c.to_hex_string(), "#ff0000");
        assert!(!c.has_alpha);

        let c2 = c.rgba(0.5);
        assert_eq!(c2.to_hex_string(), "#ff000080");
        assert!(c2.has_alpha);

        let c3 = c2.rgba(1.0);
        // This is a design choice: once alpha was introduced, we keep has_alpha
        assert!(
            c3.has_alpha,
            "Once alpha introduced, we keep has_alpha true."
        );
    }

    // Test some conversions:
    // CSS requires clamping channels to [0..255] and alpha to [0..1].
    #[test]
    fn test_clamping() {
        let c = Color::new(255, 255, 255, 2.0);
        // Clamped to 255,255,255 and alpha=1.0
        assert_eq!(c.to_hex_string(), "#ffffffff");

        let c2 = Color::new(0, 0, 0, -1.0);
        // alpha clamped to 0.0
        assert_eq!(c2.to_hex_string(), "#00000000");
        assert!(c2.has_alpha);
    }

    // Test mixing colors:
    // CSS does not fully define a "mix" function like SASS, but we can verify correctness of arithmetic.
    // According to our logic: mix(#ff0000,#0000ff,50%) => average of red and blue
    #[test]
    fn test_mix() {
        let red = Color::try_from_str("red").unwrap();
        let blue = Color::try_from_str("blue").unwrap();
        let mixed = Color::mix(red, blue, 50.0);
        // Mix should be #800080 (purple) if we do a simple linear blend.
        // (255*0.5=127.5 ~128, but rounding might give #800080)
        assert_eq!(mixed.to_hex_string(), "#800080");
    }

    // Test hsl adjustments:
    // hsl(0,100%,50%) is red. Adjust hue by +120deg:
    // hsl(120,100%,50%) = #00ff00 (green)
    #[test]
    fn test_adjust_hue() {
        let red = Color::try_from_str("red").unwrap();
        let green = red.adjust_hue(120.0);
        assert_eq!(green.to_hex_string(), "#00ff00");
    }

    // Test saturation changes:
    // Start with hsl(0,50%,50%) ~ #ff8080 (less saturated red)
    // Saturate by 50% => hsl(0, ~75%,50%) => more red
    #[test]
    fn test_saturate_desaturate() {
        let c = Color::try_from_str("hsl(0,50%,50%)").unwrap();
        // ~#ff8080 (check exact: s=50% l=50% red hue)
        assert_eq!(c.to_hex_string(), "#bf4040");
        let satur = c.saturate(50.0);
        // Now s=100% (50%+50% of difference = 50+(50)=100%)
        assert_eq!(satur.to_hex_string(), "#ff0000");

        let desat = satur.desaturate(100.0);
        // s=0% => gray: l=50% = #808080
        assert_eq!(desat.to_hex_string(), "#808080");
    }

    // Test lighten/darken:
    // hsl(0,100%,50%) = red
    // lighten by 10% => l=60% => hsl(0,100%,60%) ~ #ff3333
    // darken by 10% => l=40% => hsl(0,100%,40%) ~ #cc0000
    #[test]
    fn test_lighten_darken() {
        let red = Color::try_from_str("red").unwrap();
        let lighter = red.lighten(10.0);
        // l=60% => check result approximately #ff3333
        assert_eq!(lighter.to_hex_string(), "#ff3333");

        let darker = red.darken(10.0);
        // l=40% => ~ #cc0000
        assert_eq!(darker.to_hex_string(), "#cc0000");
    }

    // Test alpha operations:
    // from red (#ff0000, a=1.0) -> fade_out(0.5) => a=0.5 => #ff000080
    #[test]
    fn test_alpha_operations() {
        let red = Color::try_from_str("red").unwrap();
        let faded = red.fade_out(0.5);
        assert_eq!(faded.to_hex_string(), "#ff000080");
        assert!(faded.has_alpha);

        let opaque = faded.fade_in(0.5);
        // back to alpha=1.0, but has_alpha was true before
        assert_eq!(opaque.to_hex_string(), "#ff0000ff");
        assert!(opaque.has_alpha);
    }
}
