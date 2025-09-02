<!-- START HERO IMAGE -->

<!-- END HERO IMAGE -->

<!-- START BADGES -->

<!-- END BADGES -->

<!-- START NOTE -->

<!-- END NOTE -->

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

**Grimoire CSS** is a comprehensive CSS engine crafted in Rust, <br /> focusing on unmatched flexibility, reusable dynamic styling, and optimized performance for every environment. Whether you need filesystem-based CSS generation or pure in-memory processing, Grimoire CSS adapts to your needs without compromising on performance or features.

# Everything in Its Place

1. **True CSS engine.** Exceptionally powerful and endlessly flexible. Independent and self-sufficient. No bundlers, optimizers, deduplicators, preprocessors or postprocessors required — it outputs final CSS on its own.
2. **Performance.** Every part is optimized for maximum efficiency, outperforming any specialized tools. It processes almost **200k classes per second**. It is **5× faster** and **28× more efficient** than TailwindCSS v4.x. All while truly generating CSS.
3. **Universality.** The native parser handles any source file without plugins or configuration, running in both filesystem-based and in-memory modes. Available as a standalone binary, a Rust crate, and an npm library.
4. **Intelligent CSS Generation.** Respects the CSS cascade and applies necessary vendor prefixes. Uses your <code class="code">.browserslistrc</code> to guarantee target browser support.
5. **Spell and Scroll Systems.** Turn CSS into your personal styling language — no arbitrary class names, no hidden abstractions. Write clear <code class="code">property=value</code> Spells, complete with <code class="code">area\_\_</code>, <code class="code">&#123;focus&#125;</code> and <code class="code">effect:</code> modifiers for breakpoints, selectors and pseudo-classes. Bundle them into Scrolls — named, parameterized, inheritable style modules for consistent systems at any scale.
6. **Color Toolkit.** A powerful module compliant with CSS Color Module Level 4, enabling precise and high-performance color manipulations. Grimoire CSS also serves as a standalone color toolkit, with its color features available via a public API.
7. **Configuration.** Grimoire CSS uses a single JSON configuration file per repository. Its format is straightforward yet robust, supporting monorepos with hundreds of projects or individual configurations — variables, scrolls, generation modes, shared and critical CSS, external files — all out of the box.
8. **Effortless Migration.** The **Transmutator** available as a CLI tool or Web UI, simplifies converting any CSS to the Spell format. Migrate entire projects to Grimoire CSS without changing your component class names, benefiting from the engine's power immediately, even with a gradual transition.

# A Spell System

At the heart of Grimoire CSS lies the `Spell`, the foundational entity of the system. `Spell` takes a different approach from traditional utility classes, like those you’d find in Tailwind. While utilities in Tailwind feel like slightly enhanced Bootstrap classes, Grimoire CSS takes things to a new level. In Tailwind, you’re expected to memorize arbitrary names like `rounded-md` for `border-radius: 0.375rem` - which doesn’t even make things look rounded. And then there’s `tracking-tight` for `letter-spacing: -0.025em`. How are you supposed to know that’s related to letter spacing?

Grimoire CSS cuts through that confusion by introducing `Spell` - an approach that is both simple and infinitely flexible. At its core, a `Spell` is just a CSS declaration, written in a format everyone understands: `property=value`. For example, `border-radius: 0.375rem` in Grimoire CSS becomes `border-radius=0.375rem`. If you prefer something shorter, `bd-rad=0.375rem` works too, or even `bd-rad=.375rem` (yes, Grimoire CSS respects CSS's own shorthand capabilities). Unlike pre-baked utility classes, `Spells` follow the natural structure of CSS: `property: value` becomes `component=target`.
If you don’t know any shorthands yet, you can always write out full components (each full component directly maps to its corresponding CSS property) and then run the `shorten` command to convert all full components in your files (that defined in config) into their shorthand forms. Easy as it should be!

This isn't just another syntax. It’s the whole system reimagined. You’re free to write any value in the target, whether it's custom units, functions, or even complex animations. Everything CSS supports is fair game, and all you need to do is escape spaces with underscores (`_`). That’s it. Of course, we didn't stop at the basics. Spells also introduce **optional enhancements**: `area`, `focus`, and `effects`, which give you deeper control over media queries, pseudo-classes, attributes, and more.

- **`area`**: The `area` defines conditions like screen size and sits at the start of your spell, separated from the rest by double underscores (`__`). For example, `(width>=768px)__bd-rad=0.375rem` will activate the rule only for screens wider than 768px. Prefer a shorthand? You can use built-in names like `md__bd-rad=0.375rem`. It’s still valid CSS, but with all the magic of `Spell`.

- **`focus`**: Sometimes, you need more than a class or a media query. `focus` lets you wrap anything - attributes, pseudo-classes, or nested selectors - inside your spell. Placed as the second part of the spell (or first if there's no `area`), it’s enclosed in curly brackets. For example: `{[hidden]_>_p:hover:active}color=red` becomes this CSS:

  ```css
  ... [hidden] > p:hover:active {
    color: red;
  }
  ```

  It’s not just readable - it’s intuitive. What you see is exactly what you get.

- **`effects`**: Sometimes, you need quick pseudo-classes without the full complexity of `focus`. That’s where `effects` come in. Just add pseudo-classes directly in the spell like this: `hover,active:color=blue`. With `effect`, you keep it compact without losing any power. Simply separate it from the `component` and `target` with a colon (`:`).

The entire `Spell` system is built on clarity and explicitness. There are no magical, arbitrary strings for targets like you find in other systems. And we don’t compromise on clarity for the sake of brevity. Targets are full, valid CSS values - because that’s how it should be. Components mirror actual CSS properties, but they can be shortened if you want. In this way, Grimoire CSS is both a **CSS declaration** and a **methodology**. It’s so powerful because every `Spell` is valid CSS - there’s no abstraction that gets in the way of what you need to achieve.

So, why call it a `Spell`? Because, like magic, it’s composed of multiple elements: `area`, `focus`, `effect`, `component`, and `target`. And each of these pieces works together to create something far greater than the sum of its parts. With Grimoire CSS, you’re not just writing styles - you’re casting spells. The name **Grimoire** comes from ancient magical texts. Just as those books hold the knowledge to perform spells, Grimoire CSS provides you the knowledge and tools to perform CSS magic - without relying on pre-baked solutions. You’re in full control.

## Recap

- The structure of a spell follows this format: `area__{focus}component=target` or `area__effect:component=target`.
- Use dashes (`-`) to separate words.
- Use underscores (`_`) to escape spaces.

# Scroll: Crafting Reusable, Dynamic CSS with Infinite Flexibility

A `Scroll` is like a `Spell`, but with one crucial difference - it’s something you build from scratch. Think of it as a customized collection of styles, bundled into one reusable class. Sometimes, you need to combine multiple styles into a single class for consistency, reusability, or just to make your life easier. With `Scroll`, you can do just that. Combine spells, give your new creation a name, and you’ve got a `Scroll` ready to use across your projects.

And here's the best part: everything you love about `Spells` works seamlessly with `Scrolls` too - `area`, `focus`, `effect`, and even `target`. But there's even more: when you define a `Scroll`, you can introduce **variables** to make your styles dynamic. Just use the `$` symbol, and the `target` becomes a placeholder, waiting for the actual value to be filled in. Want to create a button class that accepts variable values? No problem. Here’s an example:

```json
"scrolls": [
  {
    "name": "btn",
    "spells": [
      "padding=6px",
      "border-radius=$",
      "accent-color=center",
      "background=none",
      "background-color=$",
      "cursor=pointer",
      "hover:background-color=$",
      "active:background-color=$",
      "xl__min-width=32px",
      "xl__padding=10px",
      "xl__hover:font-weight=bold"
    ]
  }
]
```

This `btn` scroll expects four target values, and if you pass fewer or more, Grimoire CSS will kindly let you know. The targets are applied in order, giving you incredible flexibility. But we're not done yet.

## Inheritance with `Scrolls`: The Power of Composition

One of the most exciting aspects of `Scrolls` is **inheritance**. Yes, you can extend a `Scroll` with another `Scroll`. Combine and compose them endlessly to create complex, reusable styles. Let's take a look:

```json
"scrolls": [
  {
    "name": "btn",
    "spells": [
      "padding=6px",
      "border-radius=$",
      "accent-color=center",
      "background=none",
      "background-color=$",
      "cursor=pointer",
      "hover:background-color=$",
      "active:background-color=$",
      "xl__min-width=32px",
      "xl__padding=10px",
      "xl__hover:font-weight=bold"
    ]
  },
  {
    "name": "danger-btn",
    "extends": [
      "btn"
    ],
    "spells": [
      "hover:g-anim=vibrate-3",
      "animation-iteration-count=infinite",
      "color=white"
    ]
  },
  {
    "name": "danger-btn-rnd",
    "extends": [
      "danger-btn",
      "round"
    ],
    "spells": []
  },
  {
    "name": "round",
    "spells": [
        "border-radius=999999px",
        "height=$",
        "weight=$"
    ]
  }
]
```

In this example, `danger-btn` extends `btn`, meaning it inherits all of `btn`'s spells plus its own. So, `danger-btn.spells` will look like `btn.spells` + `danger-btn.spells`, with the parent scroll's styles taking priority at the top.

But the fun doesn’t stop there - `danger-btn-rnd` extends both `danger-btn` and `round`. This means that `danger-btn-rnd.spells` equals `btn.spells` + `danger-btn.spells` + `round.spells`, combined in the correct order. And yes, the order matters. This layered inheritance allows you to build complex style structures effortlessly.

## Why Scrolls Matter: Unlimited Possibilities

The real magic of `Scrolls` lies in their **unlimited possibilities**. You can chain styles together, extend them endlessly, and define variables as placeholders to create flexible, reusable patterns across your entire project. With `Scrolls`, Grimoire CSS goes far beyond being Yet Another CSS Framework. In fact, you could even recreate the entire structure of Tailwind or Bootstrap using nothing but the flexibility of Spells and Scrolls.

It’s pure, beautiful madness - without limits.

# Variables and Built-in Functions: Total Control Over Styles and Sizes

Grimoire CSS allows you to define your own variables within its settings, making your styling even more dynamic and customizable. Unlike custom properties, these variables don’t compile and remain in your settings and are only compiled when used - keeping your CSS clean and efficient.

## How to Use Variables

You can define **any value** as a variable - font sizes, colors, dimensions, anything. To reference them in your styles, just add the `$` symbol before the variable name (you’ll remember this from the `Scroll` section). Here’s how you define and use a variable:

### Defining a Variable

```json
{
  "variables": {
    "hero-fs": "42px"
  }
}
```

### Using the Variable

```html
<h1 class="font-size=$hero-fs">Hero text</h1>
```

In this example, the `hero-fs` variable holds the value `42px`, which is then applied to the `font-size` of the `<h1>` element. Variables in Grimoire CSS offer a simple and effective way to maintain consistency across your styles, while keeping your code flexible and DRY.

## Built-in Areas: Responsive Design, Simplified

Grimoire CSS follows a mobile-first approach and comes with **built-in responsive areas**, including `sm`, `md`, `lg`, `xl`, and `2xl`. When you define a spell with one of these areas, like `md__width=100px`, the spell will apply only when the screen width is equal to or greater than the specified area.

For example, `md__width=100px` is equivalent to this media query:
`(width>=768px)__width=100px`.

Of course, you’re not limited to the built-in areas. You can define your own media queries just as easily, like this:

```css
(width>666px)__width=100px
```

With these areas, you have full control over your responsive design, but without the hassle of constantly writing and rewriting media queries.

## Adaptive Size Functions: `mrs` and `mfs`

Grimoire CSS takes responsive design even further with built-in functions like `mrs` (**M**ake **R**esponsive **S**ize) and `mfs` (**M**ake **F**luid **S**ize). These functions allow you to adapt font sizes, widths, and more based on the viewport size.

### `mrs`: Make Responsive Size

This function dynamically adjusts the size of an element between a minimum and maximum value, depending on the viewport width. Here are the arguments:

- `min_size`: The minimum size for the element.
- `max_size`: The maximum size for the element.
- `min_vw`: (Optional) The minimum viewport width.
- `max_vw`: (Optional) The maximum viewport width.

#### Example Usage of `mrs`

```html
<p class="font-size=mrs(12px_36px_480px_1280px)">
  Font size of this text will dynamically change based on the screen size
</p>
```

In this example, the font size will automatically adjust between 12px and 36px, depending on the screen size, with fluid adjustments in between. This makes responsive design not only easier but more precise, without the need for complex calculations or multiple breakpoints.

### `mfs`: Make Fluid Size – Creates fully fluid sizes without media queries for seamless scaling

Here are the arguments:

- `min_size`: The minimum size for the element.
- `max_size`: The maximum size for the element.

#### Example Usage of `mfs`

```html
<p class="font-size=mfs(12px_36px)">
  Font size smoothly scales between 12px and 36px based on the viewport size.
</p>
```

### The Power of Grimoire’s Variables and Functions

With Grimoire CSS, you don’t just write styles - you take control of them. By leveraging variables, responsive areas, and adaptive size functions, you can make your CSS dynamic, scalable, and ready for any device or screen size. It’s flexibility without the fuss, and it’s all built right in.

# Predefined Scrolls and Built-In Animations: Flexibility at Your Fingertips

Grimoire CSS doesn’t just give you the tools to build powerful styles from scratch - it also comes with a set of **predefined scrolls** to help you get started right away. All predefined scrolls follow the same convention: they begin with the prefix `g-`. This makes it easy to distinguish built-in scrolls from the ones you define yourself.

## Built-In Animations: Ready When You Are

Grimoire CSS comes loaded with **hundreds of built-in animations** (700+ at the moment). These animations are lightweight and efficient - they are only compiled if you actually use them. To trigger one, simply use its name in either the `animation-name` or `animation` CSS rule. But Grimoire CSS doesn’t stop at just applying animations; it also simplifies the process of adding associated rules.

For example, the predefined scroll `g-anim` allows you to apply an animation and its associated rules at the same time. Here, `g-` is the prefix, and `anim` is a short version of the spell `animation`. With this scroll, you can quickly inject an animation along with the necessary rules - saving time and keeping your styles clean and organized.

<!-- START ANIMATIONS SECTION -->
<!-- END ANIMATIONS SECTION -->

## Create Your Own Animations

Even though Grimoire CSS comes packed with animations, it also gives you the power to add your own, seamlessly integrating them into your projects. It’s as simple as creating a new subfolder called `animation` inside the `grimoire` folder, then adding your custom CSS file using the format `<name-of-animation>.css`.

Within that file, you define your animation using `@keyframes`, along with any custom styles. You can also use the class placeholder `GRIMOIRE_CSS_ANIMATION` to add specific styles tied to the animation itself. Let’s take a look at an example with a custom pulse animation:

```css
@keyframes pulse {
  from {
    transform: scale3d(1, 1, 1);
  }

  50% {
    transform: scale3d(1.05, 1.05, 1.05);
  }

  to {
    transform: scale3d(1, 1, 1);
  }
}

.GRIMOIRE_CSS_ANIMATION {
  animation-name: pulse;
  animation-timing-function: ease-in-out;
}
```

In this example, you’ve defined the pulse animation and set it up with ease using the `GRIMOIRE_CSS_ANIMATION` placeholder. Once this file is in your project, you can invoke the pulse animation as easily as any built-in animation, giving you complete control over custom animations.

# External Scrolls & Variables

In addition to defining scrolls and variables within `grimoire.config.json`, Grimoire CSS allows you to extend your configuration with external JSON files for scrolls and variables. These external JSON files follow the same structure as their corresponding properties in the config file. These files should be stored alongside your main configuration file.

Before generating CSS, Grimoire CSS checks for any external scrolls or variables and merges them into the main config (with the main config taking priority, so external scrolls/variables won't override your primary configuration settings). This adds flexibility, scalability, and convenience to your workflow.

This feature enables sharing your scrolls/spells independently from your main configuration, as well as using those created by others. For example, you can use the Tailwind CSS implementation via external scrolls. More information about where to share and find scrolls, variables, or complete configurations will be detailed below.

# Language-Agnostic Parser: Extracting Spells from Any File, Any Format

Grimoire CSS isn’t just tied to traditional CSS, JavaScript, or HTML files. The beauty of its **language-agnostic parser** is that it can parse spells from any file or extension. Whether you’re working with `.html`, `.tsx`, `.mdx`, or something else entirely, it can handle it.

This means you’re not limited by file types or formats - you define the `inputPaths`, and Grimoire takes care of the rest. Whether your project is built with React, Vue, or something entirely different, it seamlessly integrates and extracts the styles you need.

## Spells in Plain Text with Template Syntax

If you want to use spells outside the traditional `class` or `className` attributes, Grimoire CSS provides a clever solution with its **template syntax**: `g!<spell>;`. This syntax lets you wrap your spell in a template, enabling the parser to collect spells from any text-based content.

Let’s say you have both a classic spell and a templated spell that are essentially the same. Don’t worry - Grimoire CSS is smart enough to combine them into one, as long as it doesn’t affect the CSS cascade. The result? Clean, efficient CSS output like this:

```css
.classic,
.templated {
  /* CSS declaration */
}
```

Template syntax also supports multiple spells in a single template using the `&` symbol as a spells separator: `g!color=violet&display=flex;`. This enables CSS-in-JS–like scenarios in absolutely any files.

This flexibility means you can integrate Grimoire CSS in non-traditional environments, using it across various file types and even in plain text. It's not just tied to the web - it’s ready for any project, anywhere.

# CSS Optimization: Minification, Vendor Prefixes, and Deduplication - All with CSS Cascade in Mind

Grimoire CSS doesn’t just help you manage your styles - it ensures that only the CSS you actually need is generated. No duplicates, no wasted space. Whether it’s shared across multiple projects or inlined for critical loading, Grimoire makes sure your CSS is lean, efficient, and optimized for performance.

Grimoire CSS takes optimization seriously. It generates only the CSS that's actually used, and it monitors for duplicates right from the start, ensuring no unnecessary styles sneak through. This happens at the very **early stages** of generation, so by the time the process finishes, you've got a lean, clean stylesheet.

But it doesn't stop there. Just take a look:

- **Minification**: It shrinks your CSS without sacrificing readability or maintainability.
- **Vendor Prefixes**: Automatically adds necessary prefixes for cross-browser compatibility based on your browserslist configuration:
  - Uses `.browserslistrc` if it exists in your project
  - Falls back to 'defaults' if no configuration is found
  - Supports custom browserslist configuration in in-memory mode
- **Deduplication**: Duplicate CSS? Not here. Grimoire keeps a close watch and ensures that only the needed CSS is generated.
- **Modern CSS Features**: Automatically transforms modern CSS features for better browser compatibility

All of this happens while preserving the **CSS cascade** - no unintentional overwrites, no broken styles. Just clean, optimized CSS that's ready for any environment.

# Professional-Grade Color Toolkit

Grimoire CSS introduces a comprehensive suite of built-in color manipulation functions, compliant with the CSS Color Module Level 4 specification. These functions enable precise and dynamic color transformations:

- **`g-grayscale(color)`**: Converts a color to grayscale by setting its saturation to 0%.
- **`g-complement(color)`**: Generates the complementary color by adding 180° to the hue.
- **`g-invert(color_weight?)`**: Inverts a color. Optionally, the `weight` parameter controls the intensity of the inversion (default: 100%).
- **`g-mix(color1_color2_weight)`**: Blends two colors based on a specified weight (0% - 100%).
- **`g-adjust-hue(color_degrees)`**: Rotates the hue of a color by a specified number of degrees (positive or negative).
- **`g-adjust-color(color_red?_green?_blue?_hue-val?_sat-val?_light-val?_alpha-val?)`**: Adjusts individual components of a color using delta values for RGB or HSL channels.
- **`g-change-color(color_red?_green?_blue?_hue-val?_sat-val?_light-val?_alpha-val?)`**: Sets absolute values for RGB or HSL components.
- **`g-scale-color(color_red?_green?_blue?_sat-val?_light-val?_alpha-val?)`**: Scales RGB or HSL components by percentage values (positive to increase, negative to decrease).
- **`g-rgba(color_alpha)`**: Updates the alpha (opacity) of a color.
- **`g-lighten(color_amount)`**: Increases the lightness of a color by a specified percentage.
- **`g-darken(color_amount)`**: Decreases the lightness of a color by a specified percentage.
- **`g-saturate(color_amount)`**: Increases the saturation of a color by a specified percentage.
- **`g-desaturate(color_amount)`**: Decreases the saturation of a color by a specified percentage.
- **`g-opacify(color_amount)`** (Alias: `g-fade-in`): Increases the opacity of a color by a specified amount.
- **`g-transparentize(color_amount)`** (Alias: `g-fade-out`): Decreases the opacity of a color by a specified amount.

## Example Usage

**Usage Rules:**

1. All arguments are positional, and any optional arguments can be omitted if they are not being changed.
2. Do not include `%`, `deg`, or other units in the values - Grimoire handles these internally.

```html
<div class="bg=g-grayscale(#ff0000)">Grayscale Red Background</div>

<div class="bg=g-complement(#00ff00)">Complementary Green Background</div>

<div class="bg=g-invert(#123456)">Fully Inverted Background</div>
<div class="bg=g-invert(#123456_50)">Partially Inverted Background</div>

<div class="bg=g-mix(#ff0000_#0000ff_50)">Purple Background</div>

<div class="bg=g-adjust-hue(#ffcc00_45)">Hue Adjusted Background</div>

<div class="bg=g-adjust-color(#123456_0_0_12)">Adjust Blue Component</div>
<div class="bg=g-adjust-color(#123456_0_0_12_5)">Adjust Blue and Saturation</div>

<div class="bg=g-change-color(#123456_255_0)">Set Red and Green Components</div>
<div class="bg=g-change-color(#123456_0_0_0_180)">Set Hue Only</div>

<div class="bg=g-scale-color(#123456_10_-10)">Scale Red Up, Green Down</div>
<div class="bg=g-scale-color(#123456_0_0_0_20)">Scale Saturation Up</div>

<div class="bg=g-rgba(#123456_0.5)">Half Transparent Background</div>

<div class="bg=g-lighten(#123456_10)">Lightened Background</div>

<div class="bg=g-darken(#123456_10)">Darkened Background</div>

<div class="bg=g-saturate(#123456_20)">More Saturated Background</div>

<div class="bg=g-desaturate(#123456_20)">Less Saturated Background</div>

<div class="bg=g-opacify(#123456_0.2)">More Opaque Background</div>

<div class="bg=g-transparentize(#123456_0.2)">More Transparent Background</div>
```

These functions provide developers with an extensive toolkit for creating vibrant, dynamic, and flexible styles with ease.

# Projects

In Grimoire CSS, managing your projects is as flexible as the spells themselves. You define exactly which files need to be parsed (`inputPaths`, supporting glob patterns) and specify where the built CSS should go (`outputDirPath`).

You also have two powerful options for compiling your CSS:

1. **Single Output File**: Where all parsed spells from various files are compiled into a single CSS file.
2. **Individual Output Files**: Where each input file has its own corresponding CSS file.

For single output mode, you’ll just need to define the name of the final CSS file with `singleOutputFileName`. The flexibility here allows you to control the output method depending on your project’s needs. Every project configuration contains a `name` property and can include as many projects as you want. Whether you’re building a single-page application (SPA) or managing multiple projects, Grimoire CSS has you covered.

In essence, the **projects** section of your config is a list of projects, each with its own unique input and output settings. Here’s how that might look:

```json
"projects": [
  {
    "projectName": "personal",
    "inputPaths": [
      "personal/src/*.tsx"
    ],
    "outputDirPath": "personal",
    "singleOutputFileName": "personal.build.css"
  },
  {
    "projectName": "blog",
    "inputPaths": [
      "blog/*.html"
    ],
    "outputDirPath": "grimoire/build/blog"
  },
  {
    "projectName": "mix",
    "inputPaths": [
      "about/hero.tsx",
      "blog/index.html"
    ],
    "outputDirPath": "grimoire/build/mix",
    "singleOutputFileName": "mix.css"
  }
]
```

- In the **first** and **third** projects, we use the single output mode, where all the spells are compiled into one file. This is ideal for **SPAs** or projects that need consolidated CSS for optimization.
- In the **second** project, a static site, each page will have its own CSS file. This approach is perfect for projects where you want isolated styles for different parts of the website, ensuring that each page only loads what it needs.

## Projects on Your Terms

Grimoire CSS gives you full control over how you manage and compile your styles. You can configure projects for different output strategies depending on whether you're building large, single-page applications or static sites with multiple pages. The flexibility to switch between single or multiple output files means you’re never locked into one approach. Grimoire adapts to your needs, not the other way around.

## Locking

Grimoire CSS supports a **locking** mechanism for efficient builds. By enabling the `lock` option in `grimoire.config.json`, you can automatically track and clean up outdated built files

```json
{
  "projects": [
    {
      "projectName": "main",
      "inputPaths": []
    }
  ],
  "lock": true
}
```

# Shared and Critical CSS: Optimizing Your Styles for Maximum Efficiency

Grimoire CSS makes it easy to define **shared** and **critical** CSS alongside your project-specific styles, allowing you to optimize how styles are applied across your entire application.

## Shared CSS: One File, Multiple Uses

Shared CSS is exactly what it sounds like - a set of styles that you can build into a separate file and reuse across multiple projects or pages in your application. By defining shared styles, you ensure consistency and reduce repetition, improving performance and maintainability.

## Critical CSS: Inline for Faster Rendering

Critical CSS goes a step further. It automatically inlines essential styles directly into your HTML files, ensuring that key styles are loaded instantly. And here’s the clever part: if some spells are already used in your components or files, Grimoire won’t regenerate them - because they’re now part of your critical CSS. No duplicates, no unnecessary bloat - just efficient, fast-loading styles.

## How It Works

Both the `shared` and `critical` sections of the config are similar in structure. Each has:

- **`styles`**: An optional list of styles that are used in the shared or critical configuration. You can include any spells, scrolls, or even paths to existing CSS files. Grimoire will extract and optimize the content during compilation.
- **`cssCustomProperties`**: An optional list of custom CSS properties, which gives you the flexibility to define your own properties and pair them with specific elements or themes.

For **shared CSS**, you’ll define an `outputPath` - the file where your shared styles will be stored. For **critical CSS**, you’ll define `fileToInlinePaths` - a list of HTML files (or glob patterns) where these essential styles should be inlined.

Let’s take a look at some examples:

## Defining Custom Properties

In the `cssCustomProperties` section, you can define custom properties and their key-value pairs for any DOM elements in your app. Here are the key parts of this configuration:

1. **`element`**: The optional DOM element associated with the CSS variable (e.g., `tag`, `class`, `id`, or even `:root`).
2. **`dataParam`**: The parameter name used in your CSS configuration.
3. **`dataValue`**: The corresponding value for that parameter.
4. **`cssVariables`**: A set of CSS variables and their values that will be applied to the element.

Here’s how this might look in JSON:

```json
"cssCustomProperties": [
  {
    "element": "body",
    "dataParam": "theme",
    "dataValue": "light",
    "cssVariables": {
      "base-bg-clr": "white"
    }
  },
  {
    "element": "body",
    "dataParam": "theme",
    "dataValue": "dark",
    "cssVariables": {
      "base-bg-clr": "black"
    }
  }
]
```

This structure allows you to define theme-specific variables, making it easier to maintain consistency across your application.

## Real-World Example

Here’s a complete example of how you might configure `shared` and `critical` CSS in Grimoire:

```json
"shared": [
  {
    "cssCustomProperties": [],
    "outputPath": "shared.css",
    "styles": [
      "font-size=20px"
    ]
  }
],
"critical": [
  {
    "fileToInlinePaths": [
      "about/*.html",
      "blog/*.html"
    ],
    "styles": [
      "reset.css",
      "padding=10%_24px",
      "color=darkblue",
      "animation-name=swing"
    ],
    "cssCustomProperties": [
      {
        "element": "body",
        "dataParam": "theme",
        "dataValue": "light",
        "cssVariables": {
          "base-bg-clr": "white"
        }
      },
      {
        "element": "body",
        "dataParam": "theme",
        "dataValue": "dark",
        "cssVariables": {
          "base-bg-clr": "black"
        }
      }
    ]
  }
]
```

In this example:

- **Shared CSS** includes a simple style (`font-size=20px`) and outputs to `shared.css`.
- **Critical CSS** will be inlined into all HTML files under the `about` and `blog` directories, ensuring essential styles like `reset.css`, padding, colors, and animations load immediately.

# Performance By Design: Built for Speed and Efficiency

Grimoire CSS achieves exceptional performance through architectural decisions, algorithmic optimizations, and efficient implementation in Rust. The system is built from the ground up with performance as a core principle:

- **Single-Pass Processing**: Processing styles in a single efficient pass
- **Smart Memory Management**: Careful memory handling and efficient data structures minimize resource usage
- **Optimized File I/O**: Reduced system calls and efficient file handling
- **Rust Implementation**: Taking advantage of zero-cost abstractions and predictable performance

Grimoire CSS isn't just fast—it's blazingly efficient:

- **Class Processing Speed**: Processes an incredible **~200,000 classes per second**
- **Memory Efficiency**: Handles **~4,000 classes per MB** of memory
- **Output Optimization**: Generates optimized CSS with minimum overhead

Remember that Grimoire CSS is a complete CSS engine that goes far beyond simple class collection and CSS generation. It handles parsing, optimization, vendor prefixing, project management, and provides powerful features like variables, functions, animations, and component composition—all while maintaining this exceptional performance profile.

## Benchmark

Grimoire CSS is lightning-fast and highly efficient. While its absolute performance is unquestionable, side-by-side comparisons often offer better perspective. This benchmark is designed to compare Grimoire CSS and Tailwind CSS by accurately measuring build time, memory usage, CPU load, and output file size.

### Overview

The benchmark creates a series of standardized test projects, each containing a large number of HTML files with utility classes for both Grimoire CSS and Tailwind CSS. Then each framework is run to process these projects, and various performance metrics are recorded and analyzed.

#### Measured Metrics

- **Build Time** — total time required to process all projects
- **Class Processing Speed** — number of processed classes per second
- **Memory Usage** — peak and average memory consumption
- **Memory Efficiency** — number of processed classes per MB of used memory
- **I/O Operations** — volume of data read and written
- **Output File Size** — total size of generated CSS files

#### Results

When compared to the latest version of Tailwind CSS (v4.x) processing the same workload of 400,000+ classes across 100,000 files, Grimoire CSS demonstrates significant advantages:

| Metric                     | Grimoire CSS  | Tailwind CSS | Difference              |
| -------------------------- | ------------- | ------------ | ----------------------- |
| **Build Time**             | 2.10s         | 10.58s       | **5.0x faster**         |
| **Peak Memory Usage**      | 111.2 MB      | 344.97 MB    | **3.1x less memory**    |
| **Average Memory Usage**   | 45.76 MB      | 182.31 MB    | **4.0x less memory**    |
| **CPU User Time**          | 755.11ms      | 7.77s        | **10.3x less**          |
| **CPU System Time**        | 1.33s         | 60.89s       | **45.7x less**          |
| **Class Processing Speed** | 190,684 cls/s | 37,824 cls/s | **5.0x faster**         |
| **Memory Efficiency**      | 3,597 cls/MB  | 1,160 cls/MB | **3.1x more efficient** |
| **Output Size**            | 5.05 MB       | 5.66 MB      | **1.1x smaller**        |

These performance advantages translate into:

- Dramatically improved development experience, even on resource-limited machines.
- Faster CI/CD pipelines and reduced cloud infrastructure costs.
- Efficient scaling for projects of any size.
- Reduced energy consumption for more sustainable development.

[View benchmark README](https://github.com/persevie/grimoire-css/tree/main/benchmark/README.md)

# A Streamlined CLI with a Strict and Straightforward API

Grimoire CSS comes with a minimal but powerful **CLI** (Command Line Interface) that’s designed for simplicity and efficiency. Whether you’re integrating it into your build process or running it manually, the CLI gets the job done without unnecessary complexity.

There are only 3 commands you need to know:

- **`init`**: Initializes your Grimoire CSS configuration, either by loading an existing config or generating a new one if none is found. This is your starting point.
- **`build`**: Kicks off the build process, parsing all your input files and generating the compiled CSS. If you haven’t already run `init`, the `build` command will handle that for you automatically.
- **`shorten`**: Automatically converts all full-length component names in your spells (as defined in your config) to their corresponding shorthand forms. This helps keep your code concise and consistent. Run this command to refactor your files, making your spell syntax as brief as possible without losing clarity or functionality.

Grimoire CSS’s CLI is built for developers who want power without bloat. It’s direct, no-nonsense, and integrates smoothly into any project or bundler.

Here’s a refined version of the remaining parts, keeping the technical depth and making them more engaging and polished:

# Easy Migration with Transmutator

Migrating to Grimoire CSS is simple thanks to the Grimoire CSS Transmutator. You can use it as a CLI tool or as a Web UI

- With the CLI, provide paths to your compiled CSS files (or pass raw CSS via a command-line flag).
- In the Web UI, either write CSS in the editor and view the JSON output in a separate tab or upload your CSS files and download the transmuted JSON.

In both modes, the Transmutator returns JSON that conforms to the external Scrolls convention by default, so you can immediately leverage your existing CSS classes as Grimoire CSS Scrolls.

You can also run the compiled CSS from Tailwind or any other framework through the Transmutator, include the produced JSON as external scrolls alongside your config, and keep using your existing class names powered by Grimoire CSS.

```json
{
  "classes": [
    {
      "name": "old-class-name",
      "spells": ["spell-1", "spell-2"]
    }
  ]
}
```

> [Grimoire CSS Transmutator Repo](https://github.com/persevie/grimoire-css-transmutator)

# Usage and Distribution

Grimoire CSS is built to integrate seamlessly into a wide range of ecosystems. It supports both filesystem-based and in-memory operations, making it perfect for traditional web development and dynamic runtime environments. It's distributed in three ways to give you maximum flexibility:

- **Single Executable Application**: A standalone binary for those who prefer a direct, no-nonsense approach.
- **NPM Library**: A Node.js-compatible interface, perfect for JavaScript and web developers.
  - `grimoire-css-js` - bin/cli versions [repo](https://github.com/persevie/grimoire-css-js)
  - webpack/rollup/vite plugins [repo](https://github.com/persevie/grimoire-css-js/tree/main/plugins)
- **Rust Crate**: For developers building in Rust or those who want to integrate Grimoire CSS at the system level.

## Working Modes

Grimoire CSS offers two primary modes of operation:

1. **Filesystem Mode** (Traditional):

   - Works with files on disk
   - Reads input files and writes CSS output to specified locations
   - Perfect for build-time CSS generation
   - Uses the standard configuration file approach

2. **In-Memory Mode**:
   - Processes CSS entirely in memory
   - No filesystem operations required
   - Ideal for runtime CSS generation or serverless environments
   - Accepts configuration and content directly through API
   - Returns compiled CSS without writing to disk

Example of using In-Memory mode in Rust:

```rust
use grimoire_css_lib::{core::ConfigInMemory, start_in_memory};

let config = ConfigInMemory {
    content: Some("your HTML/JS/any content here".to_string()),
    // Optional: provide custom browserslist configuration
    browserslist_content: Some("last 2 versions".to_string()),
    // ... other configuration options
};

let result = start_in_memory(&config)?;
// result contains Vec<CompiledCssInMemory> with your generated CSS
```

The core of Grimoire CSS is architected entirely in Rust, ensuring top-notch performance and scalability. The main repository compiles both into a standalone executable (SEA) and a Rust crate, meaning you can use it in different environments with ease.

The `grimoire-css-js` takes the core crate and wraps it into a Node.js-compatible interface, which is then compiled into an npm package. Whether you’re working with Rust, Node.js, or need a direct CLI, Grimoire CSS is ready to integrate into your workflow and bring powerful CSS management wherever you need it.

<!-- START DESK -->

<!-- END DESK -->

## Installation

**Rust crate:**

If you’re using Rust, simply add Grimoire CSS to your Cargo.toml, and follow the link for documentation about crate: [docs.rs](https://docs.rs/grimoire_css).

```bash
cargo install grimoire_css
```

or

```bash
cargo add grimoire_css
```

**Single Executable Application (SEA):**

1. Download the binary for your operating system from the [releases page](https://github.com/persevie/grimoire-css/releases).
2. Add the binary to your system’s $PATH (optional for easier usage).

**NPM Library:**

```bash
npm i @persevie/grimoire-css-js
```

**Once installed, you can run the following commands:**

Initialize a Grimoire CSS config in your project:

```bash
grimoire_css init
```

or if you are using NPM library:

```bash
grimoire-css-js init
```

Build your CSS using the Grimoire CSS config:

```bash
grimoire_css build
```

or if you are using NPM library:

```bash
grimoire-css-js build
```

<!-- START CIRCLE -->

<!-- END CIRCLE -->

<!-- START RELEASE INFO -->

<!-- END RELEASE INFO -->

<!-- START SLOGAN -->

<!-- END SLOGAN -->
