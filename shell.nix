let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import (builtins.fetchTarball {
    name = "nixos-23.11-release";
    url = "https://github.com/NixOS/nixpkgs/archive/057f9aecfb71c4437d2b27d3323df7f93c010b7e.tar.gz";
    sha256 = "1ndiv385w1qyb3b18vw13991fzb9wg4cl21wglk89grsfsnra41k";
  }) {
    overlays = [ (import rust-overlay) ];
  };
  rust = pkgs.rust-bin.stable.latest.default.override {
    targets = [
      "wasm32-unknown-unknown"
    ];
    extensions = [
      "rust-src"
    ];
  };
in
  pkgs.mkShell.override {
    stdenv = pkgs.clang16Stdenv;
  } {
    buildInputs = [
      rust
      pkgs.gdb
      pkgs.llvm
      pkgs.wasm-pack
      pkgs.wasm-bindgen-cli
      pkgs.just
      pkgs.darkhttpd
      pkgs.firefox
    ];

    # Constants for compiler
    CC_wasm32_unknown_unknown = "${pkgs.llvmPackages_16.clang-unwrapped}/bin/clang-16";
    AR_wasm32_unknown_unknown = "${pkgs.llvmPackages_16.libllvm}/bin/llvm-ar";
    CFLAGS_wasm32_unknown_unknown = "-I ${pkgs.llvmPackages_16.libclang.lib}/lib/clang/16/include/";

    # Constants for IDE setup
    RUST_TOOLCHAIN = "${rust}/bin";
    RUST_STDLIB = "${rust}/lib/rustlib/src/rust";
    DEBUGGER = "${pkgs.gdb}";
  }
