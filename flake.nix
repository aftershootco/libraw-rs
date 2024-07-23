{
  description = "A simple rust flake using rust-overlay and craneLib";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs";
    flake-utils.url = "github:numtide/flake-utils";
    crane = {
      url = "github:ipetkov/crane";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    self,
    crane,
    flake-utils,
    nixpkgs,
    rust-overlay,
    ...
  }:
    flake-utils.lib.eachDefaultSystem (
      system: let
        pkgs = import nixpkgs {
          inherit system;
          overlays = [rust-overlay.overlays.default];
        };
        inherit (pkgs) lib;

        stableToolchain = pkgs.rust-bin.stable.latest.default;
        stableToolchainWithRustAnalyzer = pkgs.rust-bin.stable.latest.default.override {
          extensions = ["rust-src" "rust-analyzer"];
          # Extra targets if required
          # targets = [
          #   "x86_64-unknown-linux-gnu"
          #   "x86_64-unknown-linux-musl"
          #   "x86_64-apple-darwin"
          #   "aarch64-apple-darwin"
          # ];
        };
        craneLib = crane.lib.${system}.overrideToolchain stableToolchain;
        src = craneLib.cleanCargoSource (craneLib.path ./.);
        commonArgs = {
          inherit src;
          # buildInputs = with pkgs; [libz]; # Inputs required for the TARGET system
          buildInputs =
            [
              pkgs.libz
            ]
            ++ pkgs.lib.optionals pkgs.stdenv.isDarwin [
              pkgs.darwin.apple_sdk.frameworks.Security
              pkgs.darwin.apple_sdk.frameworks.CoreServices
            ];
          nativeBuildInputs = with pkgs; [cmake rustPlatform.bindgenHook] ++ lib.optionals pkgs.stdenv.isDarwin [libiconv]; # Intputs required for the HOST system
          # This is often requird for any ffi based packages that use bindgen
          # LIBCLANG_PATH = "${pkgs.llvmPackages.libclang.lib}/lib";
          # For using pkg-config that many libraries require
          PKG_CONFIG_PATH = lib.makeSearchPath "lib/pkgconfig" (with pkgs; [openssl.dev zlib.dev]);
        };
        cargoArtefacts = craneLib.buildDepsOnly commonArgs;

        libraw = craneLib.buildPackage (commonArgs
          // {
            inherit cargoArtefacts;
          });
      in {
        checks = {
          libraw-clippy = craneLib.cargoClippy (commonArgs
            // {
              inherit cargoArtefacts;
              cargoClippyExtraArgs = "--all-targets -- --deny warnings";
            });
          libraw-fmt = craneLib.cargoFmt {
            inherit src;
          };
          libraw-nextest = craneLib.cargoNextest (commonArgs
            // {
              inherit cargoArtefacts;
              partitions = 1;
              partitionType = "count";
            });
        };
        packages = {
          default = libraw;
        };

        apps.default = flake-utils.lib.mkApp {
          drv = libraw;
        };

        devShells.default =
          (craneLib.overrideToolchain stableToolchainWithRustAnalyzer).devShell commonArgs
          // {
            buildInputs = [];
            nativeBuildInputs = [];
            packages = with pkgs; [
              just
            ];
          };
      }
    );
}
