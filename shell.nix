{ pkgs ? import <nixpkgs> {}}:

pkgs.mkShell (with pkgs; {

    name = "The Rust Programming Language Nix Shell";

    env = {
        RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
    };

    buildInputs = with pkgs; [
        rustc
        rustup
        cargo
        gcc
    ];

    shellHook = ''
        rustup default stable
    '';

})
