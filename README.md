# firefox-js-format
A simple rust program (≈ 50 lines) that takes a input user.js 
and outputs the userprefs formatted to a nix attribute set 
into a output file

# Usage
ff-format [input_file] [output_file]
no --help or complex cli arguments right now

## Installation
Downlaod the binary from releases 

Or build it locally with Cargo:
git clone https://github.com/Pogwat/firefox-js-format
cd firefox-js-format 
cargo build

Or use Nix packging and toolchain:

contents of ./ff-format-packge.nix:

{ rustPlatform, lib, fetchFromGitHub }:
rustPlatform.buildRustPackage rec {
  pname = "ff-format";
 version = "0.1.0";
  src =  fetchFromGitHub {
    owner = "Pogwat";
    repo = "firefox-js-format";
    rev = "v${version}";
    hash = "sha256-0yX233J53vWp4/grtDesVxwa0kamOM8MUI6WOidyzGE=";
   };
   cargoLock = {
     lockFile = "${src}/Cargo.lock";
   };
 
   meta = {
     description = "firefox user.js -> nix attributes ";
     homepage = "https://github.com/Pogwat/firefox-js-format/";
     license = lib.licenses.gpl3Only;
     platforms = lib.platforms.linux;
     maintainers = with lib.maintainers; [];
     mainProgram = "ff-format";
   };
 }

contents of ./overlay.nix in same directoy:       
 { config, pkgs, ... }: {
 nixpkgs.overlays = [
 
(final: prev: {
  ff-format = final.callPackage ./ff-format-packge.nix {};
 }) 
 
 
 ];
 
 
 


 environment.systemPackages = with pkgs; [
   ff-format 
   ];
 }

under configuration.nix put: 
 imports = [path/to/overlays.nix];



