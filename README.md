<p align="center">
<img height="60" alt="Grimoire CSS logo" src="./assets/grimoire-css-logo.png">
</p>

---

<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->

**Table of Contents**

- [Welcome to the Circle: Unlock the Magic of CSS with Grimoire](#welcome-to-the-circle-unlock-the-magic-of-css-with-grimoire) - [
  Craft Your Code, Cast Your Spells
  ](#craft-your-code-cast-your-spells)
  - [A Spell System That Unleashes the Full Power of CSS — With No Limits, No Extra Learning](#a-spell-system-that-unleashes-the-full-power-of-css--with-no-limits-no-extra-learning)
    - [Recap](#recap)
  - [Scroll: Crafting Reusable, Dynamic CSS with Infinite Flexibility](#scroll-crafting-reusable-dynamic-css-with-infinite-flexibility)
    - [Inheritance with `Scrolls`: The Power of Composition](#inheritance-with-scrolls-the-power-of-composition)
    - [Why Scrolls Matter: Unlimited Possibilities](#why-scrolls-matter-unlimited-possibilities)
  - [Projects: Organizing and Compiling Your CSS with Flexibility and Control](#projects-organizing-and-compiling-your-css-with-flexibility-and-control)
    - [Projects on Your Terms](#projects-on-your-terms)
  - [Shared and Critical CSS: Optimizing Your Styles for Maximum Efficiency](#shared-and-critical-css-optimizing-your-styles-for-maximum-efficiency)
    - [Shared CSS: One File, Multiple Uses](#shared-css-one-file-multiple-uses)
    - [Critical CSS: Inline for Faster Rendering](#critical-css-inline-for-faster-rendering)
    - [How It Works](#how-it-works)
    - [Defining Custom Properties](#defining-custom-properties)
    - [Real-World Example](#real-world-example)
    - [Efficiency at Its Core](#efficiency-at-its-core)
  - [Predefined Scrolls and Built-In Animations: Flexibility at Your Fingertips](#predefined-scrolls-and-built-in-animations-flexibility-at-your-fingertips)
    - [Built-In Animations: Ready When You Are](#built-in-animations-ready-when-you-are)
    - [Create Your Own Animations](#create-your-own-animations)
  - [Variables and Built-in Functions: Total Control Over Styles and Sizes](#variables-and-built-in-functions-total-control-over-styles-and-sizes)
    - [How to Use Variables](#how-to-use-variables)
      - [Defining a Variable](#defining-a-variable)
      - [Using the Variable](#using-the-variable)
    - [Built-in Areas: Responsive Design, Simplified](#built-in-areas-responsive-design-simplified)
    - [Adaptive Size Functions: `mrs` and `mfs`](#adaptive-size-functions-mrs-and-mfs)
      - [`mrs`: Make Responsive Size](#mrs-make-responsive-size)
      - [Example Usage](#example-usage)
    - [The Power of Grimoire’s Variables and Functions](#the-power-of-grimoires-variables-and-functions)
  - [CSS Optimization: Minification, Vendor Prefixes, and Deduplication — All with CSS Cascade in Mind](#css-optimization-minification-vendor-prefixes-and-deduplication--all-with-css-cascade-in-mind)
  - [Performance-Driven by Rust: Parallel Processing and Auto-Scaling Built In](#performance-driven-by-rust-parallel-processing-and-auto-scaling-built-in)
  - [Language-Agnostic Parser: Extracting Spells from Any File, Any Format](#language-agnostic-parser-extracting-spells-from-any-file-any-format)
    - [Spells in Plain Text with Template Syntax](#spells-in-plain-text-with-template-syntax)
  - [A Streamlined CLI with a Strict and Straightforward API](#a-streamlined-cli-with-a-strict-and-straightforward-api)
  - [Easy Migration with Grimoire CSS Transmute (gcsst) Utility](#easy-migration-with-grimoire-css-transmute-gcsst-utility)
  - [Usage and Distribution](#usage-and-distribution)
    - [Installation](#installation)
  - [The Arcane Circle](#the-arcane-circle)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->

> Demo projects, contribution guides, instructions, and recipes are on the way. Stay tuned for updates!

# Welcome to the Circle: Unlock the Magic of CSS with Grimoire

Grimoire CSS is more than just a framework—it’s your entry into a circle of developers who wield the true power of CSS. By mastering Spells and Scrolls, you’ll craft styles with precision, control, and a touch of magic. Whether you’re building responsive interfaces or optimizing for performance, Grimoire empowers you to write CSS in a way that feels both natural and powerful. Welcome to the circle, where the limits of traditional styling fade away, and the full potential of CSS is revealed.

## A Spell System That Unleashes the Full Power of CSS — With No Limits, No Extra Learning

At the heart of Grimoire CSS lies the `Spell`, the foundational entity of the system. `Spell` takes a different approach from traditional utility classes, like those you’d find in Tailwind. While utilities in Tailwind feel like slightly enhanced Bootstrap classes, Grimoire CSS takes things to a new level. In Tailwind, you’re expected to memorize arbitrary names like `rounded-md` for `border-radius: 0.375rem` — which doesn’t even make things look rounded. And then there’s `tracking-tight` for `letter-spacing: -0.025em`. How are you supposed to know that’s related to letter spacing?

Grimoire CSS cuts through that confusion by introducing `Spell`—an approach that is both simple and infinitely flexible. At its core, a `Spell` is just a CSS declaration, written in a format everyone understands: `property=value`. For example, `border-radius: 0.375rem` in Grimoire CSS becomes `border-radius=0.375rem`. If you prefer something shorter, `br=0.375rem` works too, or even `br=.375rem` (yes, Grimoire CSS respects CSS's own shorthand capabilities). Unlike pre-baked utility classes, `Spells` follow the natural structure of CSS: `property: value` becomes `component=target`.

This isn't just another syntax. It’s the whole system reimagined. You’re free to write any value in the target, whether it's custom units, functions, or even complex animations. Everything CSS supports is fair game, and all you need to do is escape spaces with underscores (`_`). That’s it. Of course, we didn't stop at the basics. Spells also introduce **optional enhancements**: `area`, `focus`, and `effects`, which give you deeper control over media queries, pseudo-classes, attributes, and more.

1. **`area`**: You know those media queries that clutter your CSS? In Grimoire CSS, they’re handled elegantly by `area`. The `area` defines conditions like screen size and sits at the start of your spell, separated from the rest by double underscores (`__`). For example, `(width>=768px)__br=0.375rem` will activate the rule only for screens wider than 768px. Prefer a shorthand? You can use built-in names like `md__br=0.375rem`. It’s still valid CSS, but with all the magic of `Spell`.

2. **`focus`**: Sometimes, you need more than a class or a media query. `focus` lets you wrap anything—attributes, pseudo-classes, or nested selectors—inside your spell. Placed as the second part of the spell (or first if there's no `area`), it’s enclosed in curly brackets. For example: `{[hidden]_>_p:hover:active}color=red` becomes this CSS:

   ```css
   ... [hidden] > p: hover:active {
     color: red;
   }
   ```

   It’s not just readable—it’s intuitive. What you see is exactly what you get.

3. **`effects`**: Sometimes, you need quick pseudo-classes without the full complexity of `focus`. That’s where `effects` come in. Just add pseudo-classes directly in the spell like this: `hover,active:color=blue`. With `effect`, you keep it compact without losing any power. Simply separate it from the `component` and `target` with a colon (`:`).

The entire `Spell` system is built on clarity and explicitness. There are no magical, arbitrary strings for targets like you find in other systems. And we don’t compromise on clarity for the sake of brevity. Targets are full, valid CSS values—because that’s how it should be. Components mirror actual CSS properties, but they can be shortened to your liking. In this way, Grimoire CSS is both a **CSS declaration** and a **methodology**. It’s so powerful because every `Spell` is valid CSS—there’s no abstraction that gets in the way of what you need to achieve.

So, why call it a `Spell`? Because, like magic, it’s composed of multiple elements: `area`, `focus`, `effect`, `component`, and `target`. And each of these pieces works together to create something far greater than the sum of its parts. With Grimoire CSS, you’re not just writing styles—you’re casting spells. The name ‘Grimoire’ comes from ancient magical texts. Just as those books hold the knowledge to perform spells, Grimoire CSS provides you the knowledge and tools to perform CSS magic—without relying on pre-baked solutions. You’re in full control.

### Recap

- The structure of a spell follows this format: `area__{focus}component=target` or `area__effect:component=target`.
- Use dashes (`-`) to separate words and underscores (`_`) to escape spaces.

## Scroll: Crafting Reusable, Dynamic CSS with Infinite Flexibility

A `Scroll` is like a `Spell`, but with one crucial difference—it’s something you build from scratch. Think of it as a customized collection of styles, bundled into one reusable class. Sometimes, you need to combine multiple styles into a single class for consistency, reusability, or just to make your life easier. With `Scroll`, you can do just that. Combine spells, give your new creation a name, and you’ve got a `Scroll` ready to use across your projects.

And here's the best part: everything you love about `Spells` works seamlessly with `Scrolls` too—`area`, `focus`, `effect`, and even `target`. But there’s even more: when you define a `Scroll`, you can introduce **variables** to make your styles dynamic. Just use the `$` symbol, and the `target` becomes a placeholder, waiting for the actual value to be filled in. Want to create a button class that accepts variable values? No problem. Here’s an example:

```json
"scrolls": [
  {
    "name": "btn",
    "spells": [
      "p=6px",
      "br=$",
      "ac=center",
      "bg=none",
      "bgc=$",
      "cur=pointer",
      "hover:bgc=$",
      "active:bgc=$",
      "xl__min-w=32px",
      "xl__p=10px",
      "xl__hover:fw=bold"
    ]
  }
]
```

This `btn` scroll expects four target values, and if you pass fewer or more, Grimoire CSS will kindly let you know. The targets are applied in order, giving you incredible flexibility. But we’re not done yet.

### Inheritance with `Scrolls`: The Power of Composition

One of the most exciting aspects of `Scrolls` is **inheritance**. Yes, you can extend a `Scroll` with another `Scroll`. Combine and compose them endlessly to create complex, reusable styles. Let’s take a look:

```json
"scrolls": [
  {
    "name": "btn",
    "spells": [
      "p=6px",
      "br=$",
      "ac=center",
      "bg=none",
      "bgc=$",
      "cur=pointer",
      "hover:bgc=$",
      "active:bgc=$",
      "xl__min-w=32px",
      "xl__p=10px",
      "xl__hover:fw=bold"
    ]
  },
  {
    "name": "danger-btn",
    "extends": [
      "btn"
    ],
    "spells": [
      "hover:g-anim=vibrate-3",
      "anim-ic=infinite",
      "c=white"
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
        "h=$",
        "w=$"
    ]
  }
]
```

In this example, `danger-btn` extends `btn`, meaning it inherits all of `btn`'s spells plus its own. So, `danger-btn.spells` will look like `btn.spells` + `danger-btn.spells`, with the parent scroll's styles taking priority at the top.

But the fun doesn’t stop there—`danger-btn-rnd` extends both `danger-btn` and `round`. This means that `danger-btn-rnd.spells` equals `btn.spells` + `danger-btn.spells` + `round.spells`, combined in the correct order. And yes, the order matters. This layered inheritance allows you to build complex style structures effortlessly.

### Why Scrolls Matter: Unlimited Possibilities

The real magic of `Scrolls` lies in their **unlimited possibilities**. You can chain styles together, extend them endlessly, and define variables as placeholders to create flexible, reusable patterns across your entire project. With `Scrolls`, Grimoire CSS goes far beyond being Yet Another CSS Framework. In fact, you could even recreate the entire structure of Tailwind or Bootstrap using nothing but the flexibility of Spells and Scrolls.

It’s pure, beautiful madness—without limits.

## Projects: Organizing and Compiling Your CSS with Flexibility and Control

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

### Projects on Your Terms

Grimoire CSS gives you full control over how you manage and compile your styles. You can configure projects for different output strategies depending on whether you're building large, single-page applications or static sites with multiple pages. The flexibility to switch between single or multiple output files means you’re never locked into one approach. Grimoire adapts to your needs, not the other way around.

## Shared and Critical CSS: Optimizing Your Styles for Maximum Efficiency

Grimoire CSS makes it easy to define **shared** and **critical** CSS alongside your project-specific styles, allowing you to optimize how styles are applied across your entire application.

### Shared CSS: One File, Multiple Uses

Shared CSS is exactly what it sounds like—a set of styles that you can build into a separate file and reuse across multiple projects or pages in your application. By defining shared styles, you ensure consistency and reduce repetition, improving performance and maintainability.

### Critical CSS: Inline for Faster Rendering

Critical CSS goes a step further. It automatically inlines essential styles directly into your HTML files, ensuring that key styles are loaded instantly. And here’s the clever part: if some spells are already used in your components or files, Grimoire won’t regenerate them—because they’re now part of your critical CSS. No duplicates, no unnecessary bloat—just efficient, fast-loading styles.

### How It Works

Both the `shared` and `critical` sections of the config are similar in structure. Each has:

- **`styles`**: An optional list of styles that are used in the shared or critical configuration. You can include any spells, scrolls, or even paths to existing CSS files. Grimoire will extract and optimize the content during compilation.
- **`cssCustomProperties`**: An optional list of custom CSS properties, which gives you the flexibility to define your own properties and pair them with specific elements or themes.

For **shared CSS**, you’ll define an `outputPath`—the file where your shared styles will be stored. For **critical CSS**, you’ll define `fileToInlinePaths`—a list of HTML files (or glob patterns) where these essential styles should be inlined.

Let’s take a look at some examples:

### Defining Custom Properties

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

### Real-World Example

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
      "c=darkblue",
      "anim-n=swing"
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

### Efficiency at Its Core

Grimoire CSS doesn’t just help you manage your styles—it ensures that only the CSS you actually need is generated. No duplicates, no wasted space. Whether it’s shared across multiple projects or inlined for critical loading, Grimoire makes sure your CSS is lean, efficient, and optimized for performance.

## Predefined Scrolls and Built-In Animations: Flexibility at Your Fingertips

Grimoire CSS doesn’t just give you the tools to build powerful styles from scratch—it also comes with a set of **predefined scrolls** to help you get started right away. All predefined scrolls follow the same convention: they begin with the prefix `g-`. This makes it easy to distinguish built-in scrolls from the ones you define yourself.

### Built-In Animations: Ready When You Are

Grimoire CSS comes loaded with **hundreds of built-in animations**. These animations are lightweight and efficient—they are only compiled if you actually use them. To trigger one, simply use its name in either the `animation-name` or `animation` CSS rule. But Grimoire CSS doesn’t stop at just applying animations; it also simplifies the process of adding associated rules.

For example, the predefined scroll `g-anim` allows you to apply an animation and its associated rules at the same time. Here, `g-` is the prefix, and `anim` is a short version of the spell `animation`. With this scroll, you can quickly inject an animation along with the necessary rules—saving time and keeping your styles clean and organized.

### Create Your Own Animations

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

## Variables and Built-in Functions: Total Control Over Styles and Sizes

Grimoire CSS allows you to define your own variables within its settings, making your styling even more dynamic and customizable. Unlike custom properties, these variables don’t compile into shared or critical CSS. Instead, they remain in your settings and are only compiled when used—keeping your CSS clean and efficient.

### How to Use Variables

You can define **any value** as a variable—font sizes, colors, dimensions, anything. To reference them in your styles, just add the `$` symbol before the variable name (you’ll remember this from the `Scroll` section). Here’s how you define and use a variable:

#### Defining a Variable

```json
{
  "variables": {
    "hero-fs": "42px"
  }
}
```

#### Using the Variable

```html
<h1 class="fs=$hero-fs">Hero text</h1>
```

In this example, the `hero-fs` variable holds the value `42px`, which is then applied to the `font-size` of the `<h1>` element. Variables in Grimoire CSS offer a simple and effective way to maintain consistency across your styles, while keeping your code flexible and DRY (Don’t Repeat Yourself).

### Built-in Areas: Responsive Design, Simplified

Grimoire CSS follows a mobile-first approach and comes with **built-in responsive areas**, including `sm`, `md`, `lg`, `xl`, and `2xl`. When you define a spell with one of these areas, like `md__w=100px`, the spell will apply only when the screen width is equal to or greater than the specified area.

For example, `md__w=100px` is equivalent to this media query:
`(width>=768px)__w=100px`.

Of course, you’re not limited to the built-in areas. You can define your own media queries just as easily, like this:

```css
(width>666px)__w=100px
```

With these areas, you have full control over your responsive design, but without the hassle of constantly writing and rewriting media queries.

### Adaptive Size Functions: `mrs` and `mfs`

Grimoire CSS takes responsive design even further with built-in functions like `mrs` (Make Responsive Size) and `mfs` (Make Fluid Size, coming soon). These functions allow you to adapt font sizes, widths, and more based on the viewport size.

#### `mrs`: Make Responsive Size

This function dynamically adjusts the size of an element between a minimum and maximum value, depending on the viewport width. Here are the arguments:

- `min_size`: The minimum size for the element.
- `max_size`: The maximum size for the element.
- `min_vw`: (Optional) The minimum viewport width.
- `max_vw`: (Optional) The maximum viewport width.

#### Example Usage

```html
<p class="fs=mrs(12px_36px_480px_1280px)">
  Font size of this text will dynamically change based on the screen size
</p>
```

In this example, the font size will automatically adjust between 12px and 36px, depending on the screen size, with fluid adjustments in between. This makes responsive design not only easier but more precise, without the need for complex calculations or multiple breakpoints.

### The Power of Grimoire’s Variables and Functions

With Grimoire CSS, you don’t just write styles—you take control of them. By leveraging variables, responsive areas, and adaptive size functions, you can make your CSS dynamic, scalable, and ready for any device or screen size. It’s flexibility without the fuss, and it’s all built right in.

## CSS Optimization: Minification, Vendor Prefixes, and Deduplication — All with CSS Cascade in Mind

Grimoire CSS takes optimization seriously. It generates only the CSS that’s actually used, and it monitors for duplicates right from the start, ensuring no unnecessary styles sneak through. This happens at the very **early stages** of generation, so by the time the process finishes, you’ve got a lean, clean stylesheet.

But it doesn’t stop there. Grimoire CSS integrates **LightningCSS** to take your code to the next level:

- **Minification**: It shrinks your CSS without sacrificing readability or maintainability.
- **Vendor Prefixes**: Automatically adds necessary prefixes for cross-browser compatibility, and even generates a `.browserlistrc` file using 'defaults' if you don’t already have one.
- **Deduplication**: Duplicate CSS? Not here. Grimoire keeps a close watch and ensures that only the needed CSS is generated.

All of this happens while preserving the **CSS cascade**—no unintentional overwrites, no broken styles. Just clean, optimized CSS that’s ready for any environment.

## Performance-Driven by Rust: Parallel Processing and Auto-Scaling Built In

Grimoire CSS is written entirely in **Rust**, a language designed for high performance. But we didn’t stop at Rust’s natural speed. Grimoire CSS is built with a commitment to efficiency, ensuring your CSS generation is fast, scalable, and precise.

One of the key features is **parallel processing**. Grimoire CSS knows when to scale—automatically. For projects with more than 10 files to process (whether parsing, writing, or injecting), Grimoire kicks into **parallel mode**, running tasks simultaneously to save time. For smaller projects with fewer than 10 files, it sticks to the classic execution method, ensuring order is maintained without sacrificing performance.

This **auto-scaler** ensures that whether you're working on a large-scale project or a small, focused application, Grimoire CSS adapts to meet your needs, processing your CSS efficiently and reliably.

Here’s the polished version of your next two sections, maintaining the technical depth while making it more engaging and professional:

## Language-Agnostic Parser: Extracting Spells from Any File, Any Format

Grimoire CSS isn’t just tied to traditional CSS, JavaScript, or HTML files. The beauty of its **language-agnostic parser** is that it can parse spells from virtually any file or extension. Whether you’re working with `.html`, `.tsx`, `.mdx`, or something else entirely, Grimoire CSS can handle it.

This means you’re not limited by file types or formats—you define the `inputPaths`, and Grimoire CSS takes care of the rest. Whether your project is built with React, Vue, or something entirely different, Grimoire CSS seamlessly integrates and extracts the styles you need.

### Spells in Plain Text with Template Syntax

If you want to use spells outside the traditional `class` or `className` attributes, Grimoire CSS provides a clever solution with its **template syntax**: `g!<spell>;`. This syntax lets you wrap your spell in a template, enabling the parser to collect spells from any text-based content.

Let’s say you have both a classic spell and a templated spell that are essentially the same. Don’t worry—Grimoire CSS is smart enough to combine them into one, as long as it doesn’t affect the CSS cascade. The result? Clean, efficient CSS output like this:

```css
.classic,
.templated {
  /* CSS declaration */
}
```

This flexibility means you can integrate Grimoire CSS in non-traditional environments, using it across various file types and even in plain text. It's not just tied to the web—it’s ready for any project, anywhere.

## A Streamlined CLI with a Strict and Straightforward API

Grimoire CSS comes with a minimal but powerful **CLI** (Command Line Interface) that’s designed for simplicity and efficiency. Whether you’re integrating it into your build process or running it manually, the CLI gets the job done without unnecessary complexity.

There are only two commands you need to know:

- **`init`**: Initializes your Grimoire CSS configuration, either by loading an existing config or generating a new one if none is found. This is your starting point.
- **`build`**: Kicks off the build process, parsing all your input files and generating the compiled CSS. If you haven’t already run `init`, the `build` command will handle that for you automatically.

Grimoire CSS’s CLI is built for developers who want power without bloat. It’s direct, no-nonsense, and integrates smoothly into any project or bundler.

Here’s a refined version of the remaining parts, keeping the technical depth and making them more engaging and polished:

## Easy Migration with Grimoire CSS Transmute (gcsst) Utility

Migrating to Grimoire CSS is simple, thanks to the Grimoire CSS Transmute utility, also known as gcsst. This CLI tool takes the paths of your built CSS files (or the content of built CSS if you’re working in a web environment) and returns a transmuted.json file in the following format:

```json
{
  "classes": [
    {
      "name": "old-class-name",
      "spells": ["spell-1", "spell-2"],
      "oneliner": "spell-1 spell-2"
    }
  ]
}
```

`gcsst` parses the existing CSS using cssparser and automatically generates corresponding spells for each class. One of the standout features of gcsst is the structure of the transmuted.json file, particularly the classes property. It’s designed to look like the structure of a scroll, except for the oneliner property. This makes it incredibly easy to create a scroll or copy-paste the single-line class into your component with minimal effort.

By simplifying the migration process, gcsst helps you move to Grimoire CSS without hassle, and you can instantly start leveraging the power of spells.

## Usage and Distribution

Grimoire CSS is built to integrate seamlessly into a wide range of ecosystems. It’s distributed in three ways to give you maximum flexibility:

- **Single Executable Application**: A standalone binary for those who prefer a direct, no-nonsense approach.
- **NPM Library**: A Node.js-compatible interface, perfect for JavaScript and web developers.
- **Rust Crate**: For developers building in Rust or those who want to integrate Grimoire CSS at the system level.

The core of Grimoire CSS is architected entirely in Rust, ensuring top-notch performance and scalability. The main repository compiles both into a standalone executable (SEA) and a Rust crate, meaning you can use it in different environments with ease.

The `grimoire-css-js` takes the core crate and wraps it into a Node.js-compatible interface, which is then compiled into an npm package. Whether you’re working with Rust, Node.js, or need a direct CLI, Grimoire CSS is ready to integrate into your workflow and bring powerful CSS management wherever you need it.

This version highlights the ease of migration with gcsst and the flexibility of Grimoire CSS’s distribution, making the text engaging while staying technical and informative.

Here’s the updated version with your requested changes, including installation and usage commands, followed by a React demo project using Grimoire CSS.

### Installation

**Single Executable Application (SEA):**

1. Download the binary for your operating system from the [releases page]().
2. Add the binary to your system’s $PATH (optional for easier usage).

**NPM Library:**

```bash
npm install grimoire-css-js
```

**Once installed, you can run the following commands:**

Initialize a Grimoire CSS config in your project:

```bash
grimoire-css init
```

or if you are using NPM library:

```bash
grimoire-css-js init
```

Build your CSS using the Grimoire CSS config:

```bash
grimoire-css build
```

or if you are using NPM library:

```bash
grimoire-css-js build
```

**Rust Crate:**

If you’re using Rust, simply add Grimoire CSS to your Cargo.toml, and follow the link for documentation about crate: []().

## The Arcane Circle

Grimoire CSS gives you the freedom to create styles that work exactly the way you want them to—no rigid rules or constraints. Whether you’re crafting dynamic interactions or fine-tuning layouts, Grimoire adapts to your needs, making each step straightforward and rewarding.

So, come join us. Share your work, exchange your thoughts, and help us keep pushing CSS to be more flexible and enjoyable. Together, we’re creating a space where writing styles is about mastery and craftsmanship, not about memorizing classes. Let’s see what we can build—one spell at a time.

<h3 align="center">
Craft Your Code, Cast Your Spells
</h3>
