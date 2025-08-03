{
  description = "RPG Helper development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        rustToolchain = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rust-src" "rust-analyzer" ];
          targets = [ "wasm32-unknown-unknown" ];
        };
      in
      {
        devShells.default = pkgs.mkShell {
          buildInputs = with pkgs; [
            # Rust toolchain
            rustToolchain
            
            # Build tools
            pkg-config
            openssl
            
            # Database
            sled
            
            # Web development tools
            trunk
            wasm-pack
            wasm-bindgen-cli
            
            # Additional development tools
            cargo-watch
            cargo-edit
            cargo-audit
            cargo-outdated
            bacon
            
            # System dependencies
            libiconv
            darwin.apple_sdk.frameworks.Security
            darwin.apple_sdk.frameworks.SystemConfiguration
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk.frameworks.CoreServices
            darwin.apple_sdk.frameworks.CoreFoundation
          ];

          shellHook = ''
            echo "RPG Helper Development Environment"
            echo "=================================="
            echo "Rust version: $(rustc --version)"
            echo "Cargo version: $(cargo --version)"
            echo ""
            echo "Available commands:"
            echo "  cargo run --bin server    - Run the server"
            echo "  cargo run --bin client    - Run the client"
            echo "  cargo run --bin cli       - Run the CLI"
            echo "  trunk serve               - Serve the web client with hot reload"
            echo "  cargo watch -x check      - Watch for changes and check"
            echo "  bacon                     - Better cargo output formatting"
            echo ""
            
            # Set up environment variables
            export RUST_BACKTRACE=1
            export RUST_LOG=debug
            export DATABASE_URL="sled://./data/rpg-helper.db"
            
            # Create data directory if it doesn't exist
            mkdir -p ./data
          '';

          # Environment variables
          RUST_SRC_PATH = "${rustToolchain}/lib/rustlib/src/rust/library";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
    });
}