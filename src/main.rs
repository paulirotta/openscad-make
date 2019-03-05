extern crate glob;
use self::glob::glob;
use rayon::prelude::*;
use rayon::join;
use std::ops::Add;
use std::process::Command;

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(openscadmake =>
        (version: "0.2")
        (author: "Paul Houghton <paulirotta@gmail.com>")
        (about: "Build multiple OpenSCAD models in parallel")
        (@arg stl: -e --stl "Generate each model")
        (@arg image: -i --image "Generate preview images of each model")
        (@arg recurse: -r --recurse "Walk all subdirectories")
    )
    .get_matches();

    let stl = matches.is_present("stl");
    let image = matches.is_present("image");
    let recurse = matches.is_present("recurse");
    let pattern = if recurse { "**/*.scad" } else { "*.scad" };

    println!("--- openscad-render_stl ---");
    if !(stl || image) {
        println!("'--help' for options");
    }
    if recurse {
        println!("All subdirectories will be walked recursively");
    }
    println!();

    let paths: Vec<String> = glob(pattern)
        .expect("Failed to read *.scad pattern")
        .map(|e| {
            String::from(
                e.expect("Can't get the name of a file")
                    .to_str()
                    .expect("Still convert the name of a file to readable form"),
            )
        })
        .collect();


    join(|| {
        if stl {
            println!("stl models will be generated");
            let _stl_result: Vec<String> = paths.par_iter().map(|path| render_stl(path)).collect();
        }
    }, || {
        if image {
            println!("png images will be generated");
            let _image_result: Vec<String> = paths.par_iter().map(|path| render_image(path)).collect();
        }
    });


    println!("End openscad-build");
    println!();
}

// Make an .STL file from .SCAD
fn render_stl(scad_path: &str) -> String {
    println!("Render stl {}", scad_path);

    let path = String::from(scad_path);
    let root = path
        .split('.')
        .next()
        .expect("Can not split filename at '.'");
    let mute = String::with_capacity(scad_path.len()).add(root).add(".stl");

    let output = Command::new("openscad")
        .arg("-o")
        .arg(&mute)
        .arg(scad_path)
        .output()
        .expect("Failed to execute OpenSCAD .stl render command");

    if output.status.success() {
        println!("STL render complete: {}", &scad_path);

        String::from(scad_path)
    } else {
        let e = String::from_utf8(output.stderr)
                .expect("Error during model render, but I can not show it");
        eprintln!(
            "{} model build error: {}",
            scad_path, e);

        String::from(e)
    }
}

// Render the SCAD file as a PNG image
fn render_image(scad_path: &str) -> String {
    println!("Render image {}", scad_path);
    let path = String::from(scad_path);
    let root = path
        .split('.')
        .next()
        .expect("Can not split filename at '.'");
    let png = String::with_capacity(scad_path.len()).add(root).add(".png");

    let output = Command::new("openscad")
        .arg("--render")
        .arg("-o")
        .arg(&png)
        .arg(scad_path)
        .output()
        .expect("Failed to execute OpenSCAD .png image render command");

    if output.status.success() {
        println!("PNG render complete: {}", &scad_path);

        String::from(scad_path)
    } else {
        let e = String::from_utf8(output.stderr)
                .expect("Error during image render, but I can not show it");
        eprintln!(
            "{} preview image error: {}",
            png, e);

        String::from(e)
    }
}
