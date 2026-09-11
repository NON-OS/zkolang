{
  description = "zKolang: a STARK-provable register machine and the NOX shield settlement path";

  # Pinned inputs, so the toolchain is the same on every machine and in CI. The
  # exact revisions live in flake.lock; bumping them is a reviewed change, not a
  # drift. A verifier whose byte digest must reproduce is built from a fixed
  # world or it is not reproducible at all.
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.05";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [ (import rust-overlay) ];
        };
        # Stable Rust, pinned through the rust-overlay revision in flake.lock,
        # with the components the gates use.
        rust = pkgs.rust-bin.stable.latest.default.override {
          extensions = [ "rustfmt" "clippy" "rust-src" ];
        };
        tools = [
          rust
          pkgs.cargo-deny   # the supply-chain gate reads deny.toml
          pkgs.cargo-audit  # the advisory gate
          pkgs.gitleaks     # the secret scan
          pkgs.elan         # the Lean toolchain manager reads lean/lean-toolchain
          pkgs.git
        ];
      in
      {
        # `nix develop` drops into the exact toolchain the gates run against.
        devShells.default = pkgs.mkShell {
          buildInputs = tools;
          shellHook = ''
            echo "zkolang dev shell"
            echo "  rust:  $(rustc --version 2>/dev/null)"
            echo "  lean:  managed by elan against lean/lean-toolchain"
          '';
        };

        # `nix flake check` builds this: a hermetic proof the toolchain resolves
        # and the fmt gate passes on the language crates, from a fixed world.
        checks.fmt = pkgs.stdenv.mkDerivation {
          name = "zkolang-fmt";
          src = self;
          nativeBuildInputs = [ rust ];
          buildPhase = ''
            export HOME=$TMPDIR
            cargo fmt --check -p nonos_zkolang -p nonos_zkolang_cli -p nonos_zkolang_proofs
          '';
          installPhase = "touch $out";
        };
      });
}
