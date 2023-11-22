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

`presciidoc` reads a single AsciiDoc file, either from the specified file path or from the standard input (`stdin`) if you provide no file.

* By default, it performs no actions on the document and prints it back to the terminal (`stdout`). For example:

    ```
    $ presciidoc proc_configuring-thing.adoc
    
    // The document starts here.
    [id="configuring-thing"]
    = Configuring thing
    
    ////
    Enter a module introduction here.
    ////
    An introduction...
    ```

* You can use the `--no-comments` (`-c`) option to remove all line comments and comment blocks. `presciidoc` replaces them with blank lines so that all line numbers still match the original file:

    ```
    $ presciidoc --no-comments proc_configuring-thing.adoc
    
    
    [id="configuring-thing"]
    = Configuring thing
    
    
    
    
    An introduction...
    ```

* To remove the comments completely, rather than replacing them, add the `--remove-lines` (`-r`) option:

    ```
    $ presciidoc --no-comments --remove-lines proc_configuring-thing.adoc
    
    [id="configuring-thing"]
    = Configuring thing
    
    An introduction...
    ```

* To calculate how many lines in the file are comments or literals, use the `--fraction` (`-f`) option. This calculates the percentage of the lines that would otherwise get removed:

    ```
    $ presciidoc --no-comments --fraction proc_configuring-thing.adoc

    27.8%
    ```

* To save the changes, you can overwrite the original file or create a new file by redirecting the output:

    ```
    $ presciidoc -c -r proc_configuring-thing.adoc > modified.adoc
    
    [id="configuring-thing"]
    = Configuring thing
    
    An introduction...
    ```
