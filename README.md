# user.js-to-nix 
A simple rust program (≈ 50 lines) that takes a input user.js 
and outputs the userprefs formatted to a nix attribute set 
into a output file

## Usage
ff-format [input_file] [output_file]
no --help or complex cli arguments right now

## Installation

### Precompiled Binaries
Just go to release and download precompiled binary

### Cargo Compilation
Or build it locally with Cargo: 

Clone repo 
```bash
git clone https://github.com/Pogwat/user.js-to-nix 
```

Move into Directory 
```bash
cd user.js-to-nix
```

Build Contents of src using cargo 
```bash
cargo build --release
```

### Nix Packaging
Or use Nix packging and toolchain:

contents of ./ff-format-packge.nix: 
```nix
{ rustPlatform, lib, fetchFromGitHub }: 
rustPlatform.buildRustPackage rec { 
  pname = "ff-format"; 
 version = "0.1.0"; 
  src =  fetchFromGitHub { 
    owner = "Pogwat"; 
    repo = "user.js-to-nix"; 
    rev = "v${version}"; 
    hash = "sha256-0yX233J53vWp4/grtDesVxwa0kamOM8MUI6WOidyzGE="; 
   }; 
   cargoLock = { 
     lockFile = "${src}/Cargo.lock"; 
   }; 
  
   meta = { 
     description = "firefox user.js -> nix attributes "; 
     homepage = "https://github.com/Pogwat/user.js-to-nix/"; 
     license = lib.licenses.gpl3Only; 
     platforms = lib.platforms.linux; 
     maintainers = with lib.maintainers;  [];  
     mainProgram = "ff-format"; 
   }; 
 } 
```


contents of ./overlay.nix in same directoy:  
```nix      
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
```

under configuration.nix put: 
```nix 
 imports = [path/to/overlays.nix]; 
 ```



