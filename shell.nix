{ pkgs ? import <nixpkgs> {} }:
with pkgs;
mkShell {
  buildInputs = [ scdoc secp256k1 gdb autoreconfHook ];
}
