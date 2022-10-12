# openscad-make
Cross-platform parallel .stl and .png batch generation from .scad files

![openscad-make screen shot](img/openscad-make-screen-shot.png)


To build this project for your machine:

1. Install [Rust](https://rustup.rs/)
1. Install C/C++ for your platform (On Linux `sudo apt-get install build-essentials`, On MacOS XCode with C/C++ command line tools, On Windows install Visual Studio with C/C++)
1. Type `cargo build --release`. The binary is in `/target/release/`

To run once the binary is in your path:
1. Install [OpenSCAD](https://www.openscad.org/) to your executable path (`c:\Program Files\OpenSCAD` or on Linux perhaps `~/bin/openscadversionblah.AppImage`)
1. Type 'openscad' on the command line to validate that is it in your execution path. Instructions for fixing this in MacOS are below.
1. Decompress the appropriate binary for your platform and place in the path or the root of the directory tree in which you want to build multiple OpenSCAD files
1. Type './openscad-make --help' for the list of build options.

To run on MacOS:

Add the following to `~/.zshrc` to add a `openscad` for command line launch, then restart your shell for this command to take effect.

```
function openscad {
    if [[ $# = 0 ]]
    then
        open -a "OpenSCAD"
    else
        local argPath="$1"
        [[ $1 = /* ]] && argPath="$1" || argPath="$PWD/${1#./}"
        open -a "OpenSCAD" --args "$argPath"
    fi
}
```

Issue reports and pull requests welcome.
