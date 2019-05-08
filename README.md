# rebootinto

Instruct your system to reboot into a particular OS.

Currently only supports UEFI-powered systems.

## Apps

This project provides a Command Line (`rebootinto-cli`), Terminal UI (`rebootinto-tui`) and a Graphical UI (`rebootinto-iui`) application.

## Installation

### Github releases

Grab packages from the [Github releases](https://github.com/MOZGIII/rebootinto/releases).

Not all installation artifacts are currently uploaded to the Github repos, use other installation methods if you didn't find
what you was looking for.

### CI artifacts

You can download prebuilt binaries from CI system artifacts storage. Find a commit that has a CI build associated and grab
stuff from there.
As with github releases, not everything is currently built there, so is you didn't find what you was looking for - try other
installation methods.

### Source installation

You'll need a [`Rust` installation](https://www.rust-lang.org/tools/install) (stable).

1. Clone the repo and `cd` into it.

2. `cargo build --release`

3. Install built executables
  
  - Raw binaries
    
    Copy `rebootinto-*` executables from `target/release/` to your desired location.
    
  - Windows installation via `.msi` file.
  
    1. Install cargo WIX plugin
    
       `cargo install cargo-wix`
    
    2. Build MSI installer
    
       `cargo wix`

  - Debian/Ubuntu installation via locally built `.deb` packages.
  
    1. Install cargo deb plugin
    
       `cargo install cargo-deb`
       
    2. `cd` into each application directory, build and install `.deb` packages
    
       ```shell
       cd cli
       cargo deb
       cd -
       cd target/debian
       sudo dpkg -i rebootinto-*.deb
       ```
