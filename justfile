# Commands in here can be run using `just`, see https://just.systems/man/en/ for syntax etc.

# Run all commands using bash by default
set shell := ["bash", "-c"]

# allow positional arguments to commands
set positional-arguments := true

# List available recipes in the order in which they appear in this file
_default:
    @just --list --unsorted

# Serve site locally (for development)
dev-serve:
    wrangler dev --live-reload --env dev

# Build the Rust function as wasm and trigger a pages dev reload
# dev-build-rust:
#     cargo watch --shell "wasm-pack build && touch functions/index.js" --watch Cargo.toml --watch Cargo.lock --watch src/

# dev-build-tailwind:
#     tailwindcss -i ./input.css -o ./static/out.css --watch

# Build the project once, so it's ready to deploy
# build:
#     tailwindcss -i ./input.css -o ./static/out.css
#     wasm-pack build

# Deploy to Cloudflare Pages
# deploy: build
#     wrangler pages deploy --project-name=felix-rath-dot-dev static/

dev-zellij:
    # zellij --session "www-felix-rath-dot-dev-dev" --layout "support/zellij-dev-layout.kdl"
    # # wrangler somehow survives the zellij shutdown, so kill it manually.
    # killall wrangler
