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
    wrangler dev --live-reload --env dev --assets ./static

# Build and watch tailwind CSS
dev-build-tailwind:
    tailwindcss -i ./input.css -o ./static/out.css --watch

# Deploy to Cloudflare Workers
deploy:
    wrangler deploy --assets static/

# Run zellij as a dev-environment, with automated rebuilds etc.
dev-zellij:
    zellij --session "www-felix-rath-dot-dev" --layout "support/zellij-dev-layout.kdl" options --session-serialization=false
    # wrangler somehow survives the zellij shutdown, so kill it manually.
    killall wrangler || true
