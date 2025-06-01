{
  inputs = {
    utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, utils }:
    utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs { inherit system; };
        # overrides = (builtins.fromTOML (builtins.readFile (self + "./rust-toolchain.toml")));
      in
      {
        devShells.default = pkgs.mkShell rec {
          nativeBuildInputs = with pkgs; [
            clang
            llvm
            llvmPackages.bintools
            mold # faster compilations
            pkg-config
          ];
          buildInputs = with pkgs; [
            rustup
            pre-commit
            openssl
            zstd
          ] ++ lib.optionals stdenv.isLinux [
            alsa-lib
            libxkbcommon
            udev
            vulkan-loader
            wayland
            xorg.libX11
            xorg.libXcursor
            xorg.libXi
            xorg.libXrandr
          ] ++ lib.optionals stdenv.isDarwin [
            darwin.apple_sdk_11_0.frameworks.Cocoa
            rustPlatform.bindgenHook
          ];
          packages = with pkgs; [
            taplo-lsp
          ];

          libInputs = buildInputs ++ nativeBuildInputs;

          LIBCLANG_PATH = pkgs.lib.makeLibraryPath [ pkgs.llvmPackages_latest.libclang.lib ];
          RUSTC_VERSION = "nightly-2025-04-03";
          LD_LIBRARY_PATH = "${pkgs.lib.makeLibraryPath libInputs}";
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";

          shellHook = ''
            export CARGO_HOME=~/.cargo
            export RUSTUP_HOME=~/.rustup
            export PATH=$CARGO_HOME/bin:$PATH
            export PATH=$RUSTUP_HOME/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin:$PATH
          '';
        };
      }
    );
}
