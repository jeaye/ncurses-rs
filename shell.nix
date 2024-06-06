{ pkgs ? import <nixpkgs> {} }:

pkgs.stdenv.mkDerivation
{
  name = "ncurses-rs";
  buildInputs = with pkgs;
  [
    pkgs.pkg-config
    pkgs.cargo
    pkgs.rustup
    pkgs.rustfmt
    pkgs.ncurses
  ];
}
