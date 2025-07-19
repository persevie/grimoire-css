use crate::core::Color;

/// A handler function type for spell color transformations.
///
/// Each handler receives a slice of string arguments and attempts to
/// return a `Color` if all arguments are valid.
type SpellColorFunc = fn(&[&str]) -> Option<Color>;

/// A global array of all supported spell color functions and their handlers.
///
/// Each tuple consists of:
/// 1. The name of the spell function (e.g., `"g-lighten"`).
/// 2. A function pointer that processes that spell function.
static SPELL_COLOR_FUNCTIONS: &[(&str, SpellColorFunc)] = &[
    ("g-grayscale", handle_grayscale),
    ("g-complement", handle_complement),
    ("g-invert", handle_invert),
    ("g-mix", handle_mix),
    ("g-adjust-hue", handle_adjust_hue),
    ("g-adjust-color", handle_adjust_color),
    ("g-change-color", handle_change_color),
    ("g-scale-color", handle_scale_color),
    ("g-rgba", handle_rgba),
    ("g-lighten", handle_lighten),
    ("g-darken", handle_darken),
    ("g-saturate", handle_saturate),
    ("g-desaturate", handle_desaturate),
    ("g-opacify", handle_opacify),
    ("g-fade-in", handle_fade_in),
    ("g-transparentize", handle_transparentize),
    ("g-fade-out", handle_fade_out),
];

/// Attempts to parse the input string (e.g. `g-lighten(#ff0000 10)`) as a spell color function call.
/// If parsing succeeds, it invokes the corresponding color handler and returns a hex representation
/// of the resulting color.
///
/// # Arguments
///
/// * `adapted_target` - A string slice in the form of `"g-func(...)"`,
///   for example: `"g-lighten(#ff0000 10)"`.
///
/// # Returns
///
/// * `Some(String)` containing the resulting color in hex form (e.g., `"#808080"`),
///   if parsing and the color transformation succeed.
/// * `None` if the string is not in a valid format, or the color transformation failed.
pub fn try_handle_color_function(adapted_target: &str) -> Option<String> {
    let (func_name, args_str) = parse_function_call(adapted_target)?;
    // Split arguments by spaces.
    let args: Vec<&str> = args_str.split(' ').map(|s| s.trim()).collect();

    // Find the corresponding handler.
    if let Some((_, handler)) = SPELL_COLOR_FUNCTIONS
        .iter()
        .find(|(name, _)| *name == func_name)
    {
        let result_color = handler(&args)?;
        Some(result_color.to_hex_string())
    } else {
        None
    }
}

/// Parses a string like `"g-func(arg1 arg2)"` and returns a tuple `( "g-func", "arg1 arg2" )`.
///
/// # Arguments
///
/// * `input` - The raw string to parse.
///
/// # Returns
///
/// * `Some((func_name, args_str))` if the string contains a function call with parentheses.
/// * `None` otherwise.
fn parse_function_call(input: &str) -> Option<(&str, &str)> {
    let open_paren = input.find('(')?;
    let close_paren = input.rfind(')')?;
    if close_paren <= open_paren {
        return None;
    }
    let func_name = &input[..open_paren].trim();
    let args_str = &input[open_paren + 1..close_paren].trim();
    Some((func_name, args_str))
}

// ------------------------------------------------------------------------
// Handlers for each spell-function below.
// Each returns an Option<Color>, returning None on invalid arguments.
// ------------------------------------------------------------------------

/// Processes `g-grayscale(color)`.
///
/// Expects exactly one argument: a color string (e.g., `"#ff0000"` or `"red"`).
/// Returns a new color with saturation forced to zero (grayscale).
fn handle_grayscale(args: &[&str]) -> Option<Color> {
    if args.len() != 1 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    Some(base_color.grayscale())
}

/// Processes `g-complement(color)`.
///
/// Expects exactly one argument: a color string.
/// Returns a new color by adding 180° to its hue.
fn handle_complement(args: &[&str]) -> Option<Color> {
    if args.len() != 1 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    Some(base_color.complement())
}

/// Processes `g-invert(color [weight])`.
///
/// * First argument is a color.
/// * Second optional argument is a weight in `%` (0..100).
///
/// If not provided, defaults to 100%.
fn handle_invert(args: &[&str]) -> Option<Color> {
    if args.is_empty() {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let weight = if args.len() > 1 {
        args[1].parse::<f32>().ok()
    } else {
        None
    };
    Some(base_color.invert(weight))
}

/// Processes `g-mix(color1 color2 weight)`.
///
/// * `color1`, `color2`: color strings
/// * `weight`: f32 in the range (0..100).
fn handle_mix(args: &[&str]) -> Option<Color> {
    if args.len() != 3 {
        return None;
    }
    let c1 = Color::try_from_str(args[0])?;
    let c2 = Color::try_from_str(args[1])?;
    let w = args[2].parse::<f32>().ok()?;

    Some(Color::mix(c1, c2, w))
}

/// Processes `g-adjust-hue(color degrees)`.
///
/// * `color`: a color string
/// * `degrees`: hue shift in degrees
fn handle_adjust_hue(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let deg = args[1].parse::<f32>().ok()?;
    Some(base_color.adjust_hue(deg))
}

/// Processes `g-adjust-color(color [red_delta green_delta blue_delta hue_delta sat_delta light_delta alpha_delta])`.
///
/// Each delta is optional and parsed as an integer (for RGB deltas) or float (for HSL/alpha deltas).
fn handle_adjust_color(args: &[&str]) -> Option<Color> {
    if args.is_empty() {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;

    let red_delta = args.get(1).and_then(|v| v.parse::<i32>().ok());
    let green_delta = args.get(2).and_then(|v| v.parse::<i32>().ok());
    let blue_delta = args.get(3).and_then(|v| v.parse::<i32>().ok());
    let hue_delta = args.get(4).and_then(|v| v.parse::<f32>().ok());
    let sat_delta = args.get(5).and_then(|v| v.parse::<f32>().ok());
    let light_delta = args.get(6).and_then(|v| v.parse::<f32>().ok());
    let alpha_delta = args.get(7).and_then(|v| v.parse::<f32>().ok());

    Some(base_color.adjust_color(
        red_delta,
        green_delta,
        blue_delta,
        hue_delta,
        sat_delta,
        light_delta,
        alpha_delta,
    ))
}

/// Processes `g-change-color(color [red green blue hue sat light alpha])`.
///
/// Each argument is optional and represents an absolute value rather than a delta.
fn handle_change_color(args: &[&str]) -> Option<Color> {
    if args.is_empty() {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;

    let red = args.get(1).and_then(|v| v.parse::<u8>().ok());
    let green = args.get(2).and_then(|v| v.parse::<u8>().ok());
    let blue = args.get(3).and_then(|v| v.parse::<u8>().ok());
    let hue_val = args.get(4).and_then(|v| v.parse::<f32>().ok());
    let sat_val = args.get(5).and_then(|v| v.parse::<f32>().ok());
    let light_val = args.get(6).and_then(|v| v.parse::<f32>().ok());
    let alpha_val = args.get(7).and_then(|v| v.parse::<f32>().ok());

    Some(base_color.change_color(red, green, blue, hue_val, sat_val, light_val, alpha_val))
}

/// Processes `g-scale-color(color [red_scale green_scale blue_scale saturation_scale lightness_scale alpha_scale])`.
///
/// Each argument is optional and is a percentage scale factor.
fn handle_scale_color(args: &[&str]) -> Option<Color> {
    if args.is_empty() {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;

    let red_scale = args.get(1).and_then(|v| v.parse::<f32>().ok());
    let green_scale = args.get(2).and_then(|v| v.parse::<f32>().ok());
    let blue_scale = args.get(3).and_then(|v| v.parse::<f32>().ok());
    let saturation_scale = args.get(4).and_then(|v| v.parse::<f32>().ok());
    let lightness_scale = args.get(5).and_then(|v| v.parse::<f32>().ok());
    let alpha_scale = args.get(6).and_then(|v| v.parse::<f32>().ok());

    Some(base_color.scale_color(
        red_scale,
        green_scale,
        blue_scale,
        saturation_scale,
        lightness_scale,
        alpha_scale,
    ))
}

/// Processes `g-rgba(color alpha)`.
///
/// * `color`: a color string
/// * `alpha`: new alpha value in [0..1]
fn handle_rgba(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let alpha = args[1].parse::<f32>().ok()?;
    Some(base_color.rgba(alpha))
}

/// Processes `g-lighten(color amount)`.
///
/// * `color`: a color string
/// * `amount`: a percentage by which to increase lightness
fn handle_lighten(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let amt = args[1].parse::<f32>().ok()?;
    Some(base_color.lighten(amt))
}

/// Processes `g-darken(color amount)`.
///
/// * `color`: a color string
/// * `amount`: a percentage by which to decrease lightness
fn handle_darken(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let amt = args[1].parse::<f32>().ok()?;
    Some(base_color.darken(amt))
}

/// Processes `g-saturate(color amount)`.
///
/// * `color`: a color string
/// * `amount`: a percentage by which to increase saturation
fn handle_saturate(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let amt = args[1].parse::<f32>().ok()?;
    Some(base_color.saturate(amt))
}

/// Processes `g-desaturate(color amount)`.
///
/// * `color`: a color string
/// * `amount`: a percentage by which to decrease saturation
fn handle_desaturate(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let amt = args[1].parse::<f32>().ok()?;
    Some(base_color.desaturate(amt))
}

/// Processes `g-opacify(color amount)`.
///
/// * `color`: a color string
/// * `amount`: how much to increase alpha (decrease transparency)
fn handle_opacify(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let amt = args[1].parse::<f32>().ok()?;
    Some(base_color.opacify(amt))
}

/// Processes `g-fade-in(color amount)`.
///
/// * `color`: a color string
/// * `amount`: how much to increase alpha
fn handle_fade_in(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let amt = args[1].parse::<f32>().ok()?;
    Some(base_color.fade_in(amt))
}

/// Processes `g-transparentize(color amount)`.
///
/// * `color`: a color string
/// * `amount`: how much to decrease alpha
fn handle_transparentize(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let amt = args[1].parse::<f32>().ok()?;
    Some(base_color.transparentize(amt))
}

/// Processes `g-fade-out(color amount)`.
///
/// * `color`: a color string
/// * `amount`: how much to decrease alpha
fn handle_fade_out(args: &[&str]) -> Option<Color> {
    if args.len() != 2 {
        return None;
    }
    let base_color = Color::try_from_str(args[0])?;
    let amt = args[1].parse::<f32>().ok()?;
    Some(base_color.fade_out(amt))
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A helper to compare Option<String> equality with Some("#rrggbb").
    /// This is just to reduce boilerplate in our tests.
    fn assert_hex_eq(got: Option<String>, expected_hex: &str) {
        assert_eq!(got, Some(expected_hex.to_string()));
    }

    #[test]
    fn test_grayscale_red() {
        // #ff0000 => h=0, s=100, l=50 => grayscale => #808080
        let res = try_handle_color_function("g-grayscale(#ff0000)");
        assert_hex_eq(res, "#808080");
    }

    #[test]
    fn test_complement_red() {
        // #ff0000 => complement => h=180 => #00ffff
        let res = try_handle_color_function("g-complement(#ff0000)");
        assert_hex_eq(res, "#00ffff");
    }

    #[test]
    fn test_invert_default() {
        // invert(#ffffff) => black => #000000
        let res = try_handle_color_function("g-invert(#ffffff)");
        assert_hex_eq(res, "#000000");
    }

    #[test]
    fn test_invert_with_weight() {
        // invert(#ffffff 50) => halfway between white and black => #808080
        let res = try_handle_color_function("g-invert(#ffffff 50)");
        assert_hex_eq(res, "#808080");
    }

    #[test]
    fn test_mix_half() {
        // mix(#ff0000 #0000ff 50) => halfway => #800080
        let res = try_handle_color_function("g-mix(#ff0000 #0000ff 50)");
        assert_hex_eq(res, "#800080");
    }

    #[test]
    fn test_lighten_red() {
        // lighten(#ff0000 10) => increases lightness => #ff3333
        let res = try_handle_color_function("g-lighten(#ff0000 10)");
        assert_hex_eq(res, "#ff3333");
    }

    #[test]
    fn test_invalid_spell_function() {
        let res = try_handle_color_function("g-unknown(#fff)");
        assert_eq!(res, None);
    }

    #[test]
    fn test_invalid_args() {
        // e.g. "g-grayscale()" with no args
        let res = try_handle_color_function("g-grayscale()");
        assert_eq!(res, None);
    }
}
