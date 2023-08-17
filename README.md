# presciidoc

Preprocessing AsciiDoc for other tools (WIP).

The goal of this program is to partially parse an AsciiDoc file, remove certain non-paragraph elements, and present the document as a syntax tree. Other tools can later analyze or further process the representation.

## Installation

You can install `presciidoc` on various operating systems using several package managers.

### Fedora and CentOS Stream

1. Enable the Copr repository:

    ```
    # dnf copr enable mareksu/presciidoc
    ```

2. Install `presciidoc`:

    ```
    # dnf install presciidoc
    ```

    The Copr repository distributes packages only for *supported* releases of Fedora. If you have enabled the repository but the package fails to install, check if your Fedora is still supported.

### From source on any platform

1. Install the Rust toolchain: see <https://rustup.rs/>.

2. Install `presciidoc`:

    ```
    $ cargo install --git https://github.com/msuchane/presciidoc.git
    ```

## Usage

TODO
