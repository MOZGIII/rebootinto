# rebootinto

Instruct your system to reboot into a particular OS.

Currently only supports UEFI-powered systems.

## Apps

This project provides the following apps:

- `rebootinto-cli` - a command-line interface;
- `rebootinto-tui` - a terminal user interface;
- `rebootinto-iui` - a graphical user interface (based on `libiui`);
- `rebootinto-iced` - a graphical user interface (based on `iced` crate).

## Installation

### Github Releases

Grab packages from the [Github releases](https://github.com/MOZGIII/rebootinto/releases).

Not all installation artifacts are currently uploaded to the Github releases.
Use other installation methods if you didn't find what you were looking for.

### CI Artifacts

You can download prebuilt binaries from CI system artifacts storage.
Find a commit that has a CI build associated and grab stuff from there.
As with Github Releases, not everything is currently built there, so if you
didn't find what you were looking for - try other installation methods.

- [Github Actions](https://github.com/MOZGIII/rebootinto/actions)

### Source installation

> When building on Windows, use Visual Studio shell, or somehow otherwise point
> the build system to the Windows Resource Compiler.

You'll need a [`Rust` installation](https://www.rust-lang.org/tools/install) (stable).

1. Clone the repo and `cd` into it.

2. `cargo build --release`

3. Install built executables

   - Raw binaries

     Copy `rebootinto-*` executables from `target/release/` to your desired location.

   - Windows installation via `.msi` file.

     1. Install cargo WIX plugin

        ```shell
        cargo install cargo-wix
        ```

     2. Build MSI installer

        ```shell
        cargo wix
        ```

   - Debian/Ubuntu installation via locally built `.deb` packages.

     1. Install cargo deb plugin

        ```shell
        cargo install cargo-deb
        ```

     2. `cd` into each application directory, build and install `.deb` packages

        ```shell
        cd cli
        cargo deb
        cd -
        cd target/debian
        sudo dpkg -i rebootinto-*.deb
        ```
