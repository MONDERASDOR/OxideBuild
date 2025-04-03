---

# OxideBuild

**A Multi-Platform Rust Build Tool That Actually Works**

## What's This About?

OxideBuild is what happens when you get tired of manually typing `cargo build --release --target=whatever` for the hundredth time. It's a build tool that handles the boring stuff so you can get back to writing code that matters.

## Installation

First, make sure you have Rust installed (if you don't, we'll wait while you [fix that](https://www.rust-lang.org/tools/install)).

Then:

```bash
cargo install --git https://github.com/yourusername/oxidebuild.git
```

For full cross-compilation support, run the setup script:

```bash
curl -sSL https://github.com/yourusername/oxidebuild/scripts/post_install.sh | bash
```

## Quick Start

1. Create an `oxidebuild.toml` in your project root:

```toml
[project]
name = "your_shiny_crate"
version = "0.1.0"
authors = ["You <you@example.com>"]

[build]
out_dir = "build_output"
targets = ["linux", "windows"] # Because we're inclusive like that
```

2. Build your project like a pro:

```bash
oxidebuild build --profile release
```

3. Deploy it somewhere useful:

```bash
oxidebuild deploy --destination /path/to/your/server
```

## Features That Won't Make You Sigh

- **Multi-platform builds** - Linux, Windows, macOS. No favoritism here.
- **Sensible defaults** - Because guessing build parameters is nobody's idea of fun.
- **Cross-compilation** - Build for platforms you don't even own.
- **One-command deployment** - Move your builds where they need to go.
- **Clean builds** - Remove artifacts like a responsible adult.

## When Should You Use This?

- You're tired of remembering cargo flags
- You need consistent builds across different platforms
- You want deployment to be someone else's problem (where "someone else" is OxideBuild)
- You like tools that do what you tell them

## When Should You Not Use This?

- You enjoy typing long cargo commands
- You prefer manually copying build artifacts
- You think build tools should be more complicated
- You're writing a "Hello World" program

## Contributing

Found a bug? Have an idea? We accept pull requests and constructive criticism (emphasis on constructive). 

1. Fork it
2. Fix it
3. Send it

## License

MIT. Do what you want with it, just don't blame us if it doesn't make coffee.
