{ pkgs ? import <nixpkgs> {} }:

with pkgs;

mkShell rec {
  nativeBuildInputs = [
    pkg-config
  ];

  buildInputs = with pkgs; [
    # Rustup
    clang
    llvmPackages.bintools
    rustup
  ];

  LD_LIBRARY_PATH = lib.makeLibraryPath buildInputs;

  RUSTC_VERSION = "stable";
  # https://github.com/rust-lang/rust-bindgen#environment-variables
  LIBCLANG_PATH = lib.makeLibraryPath [ llvmPackages_latest.libclang.lib ];
  shellHook = ''
    export PATH=$PATH:''${CARGO_HOME:-~/.cargo}/bin
    export PATH=$PATH:''${RUSTUP_HOME:-~/.rustup}/toolchains/$RUSTC_VERSION-x86_64-unknown-linux-gnu/bin/
    '';
  # Add precompiled library to rustc search path
  RUSTFLAGS = (builtins.map (a: ''-L ${a}/lib'') [
    # add libraries here (e.g. pkgs.libvmi)
  ]);
  # Add glibc, clang, glib and other headers to bindgen search path
  BINDGEN_EXTRA_CLANG_ARGS = 
  # Includes with normal include path
  (builtins.map (a: ''-I"${a}/include"'') [
    # add dev libraries here (e.g. pkgs.libvmi.dev)
    glibc.dev 
  ])
  # Includes with special directory paths
  ++ [
    ''-I"${llvmPackages_latest.libclang.lib}/lib/clang/${llvmPackages_latest.libclang.version}/include"''
    ''-I"${glib.dev}/include/glib-2.0"''
    ''-I${glib.out}/lib/glib-2.0/include/''
  ];
}