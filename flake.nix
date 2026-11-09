{
  description = "A Nix-devShell to build/develop this project";

  inputs = {
    # `nixpkgs-unstable` is fully ok for an application (i.e., not a NixOS-system)
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

    # `flake-utils` for easier nix-system handling
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    {
      self,
      nixpkgs,
      flake-utils,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        pkgs = import nixpkgs { inherit system; };
      in
      {
        # Rust itself comes from rustup, via rust-toolchain.toml
        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [
            zellij

            # for `cargo upgrade` (see justfile)
            cargo-edit

            # the cloudflare cli & deps
            wrangler
            worker-build
            wasm-pack

            # input.css/tailwind.config.js use v3 syntax
            tailwindcss_3

            # From https://github.com/dpc/htmx-sorta/blob/9e101583ec9391127b5bfcbe421e3ede2d627856/flake.nix#L83-L85
            # This is required to prevent a mangled bash shell in nix develop
            # see: https://discourse.nixos.org/t/interactive-bash-with-nix-develop-flake/15486
            (lib.hiPrio pkgs.bashInteractive)
          ];
        };
      }
    );
}
