# openscad-make
Cross-platform parallel .stl and .png batch generation from .scad files

Pre-built binaries for Linux (.tar.gz) and Windows (.zip) are provided.

To run:
1. Install (OpenSCAD)[https://www.openscad.org/] to your executable path ('c:\Program Files\OpenSCAD' or on Linux perhaps '~/bin/openscadversionblah.AppImage')
1. Decompress the appropriate binary for your platform
1. Invoke the binary in the root of a directory tree containing '.scad' models. The '.stl' and '.png' files for each which is not just a library will be built.  


To build:

1. Install (Rust)[https://rustup.rs/]
1. Install C/C++ for your platform (build-essentials on Linux, Visual Studio on Windows)
1. Type 'cargo build --release' and see '/target/release/'

Next steps: command line options for
1. optional generation of images
1. optional walking of the tree vs current directory
1. accepting one or more file paths to build a subset
1. error message if openscad is not in the path
