# openscad-make
Cross-platform parallel .stl and .png batch generation from .scad files

![openscad-make screen shot](img/openscad-make-screen-shot.png)

The fastest way to get support is on [Discord PELA Blocks](https://discord.gg/Yy2srz)

Pre-built binaries for Linux (x86 and ARM .tar.gz), OSX (.zip) and Windows (.zip) are provided.

To run:
1. Install [OpenSCAD](https://www.openscad.org/) to your executable path ('c:\Program Files\OpenSCAD' or on Linux perhaps '~/bin/openscadversionblah.AppImage')
1. Decompress the appropriate binary for your platform (.zip for Windows, .tar.gz for Linux, for OSX or if you prefer source, see 'build' below)
1. Invoke the binary from the directory (or root of a directory tree) containing '.scad' models. The '.stl' and '.png' files for each which is not just a library will be built.  
1. Pass '-h' or '--help' for the list of options.

To build:

1. Install [Rust](https://rustup.rs/)
1. Install C/C++ for your platform (build-essentials on Linux, XCode with C/C++ command line tools, Visual Studio with C/C++ on Windows)
1. Type 'cargo build --release'. The binary is in '/target/release/'

Next steps: command line additions for
1. overriding the directory to build (vs current directory)
1. passing OpenSCAD parameters to all files built
1. informative error message if openscad is not in the path

Issue reports and pull requests welcome.
