extern crate glob;
use self::glob::glob;
use rayon::prelude::*;
use std::ops::Add;
use std::process::Command;

#[macro_use]
extern crate clap;

fn main() {
    let matches = clap_app!(openscadmake =>
        (version: "0.2")
        (author: "Paul Houghton <paulirotta@gmail.com>")
        (about: "Build multiple OpenSCAD models in parallel")
        (@arg image: -i --image "Also generate preview images of each model")
        (@arg recurse: -r --recurse "Walk all subdirectories")
    )
    .get_matches();

    let image = matches.is_present("image");
    let recurse = matches.is_present("recurse");

    let pattern = if recurse { "**/*.scad" } else { "*.scad" };

    println!("--- openscad-make ---");
    println!("All OpenSCAD models will be built");
    if image {
        println!("Preview images will be generated");
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

    let _result: Vec<String> = paths.par_iter().map(|path| make(path, image)).collect();

    println!("End .SCAD file build");
    println!();
}

// Make an .STL file from .SCAD
fn make(scad_path: &str, image: bool) -> String {
    println!("Render {}", scad_path);

    let path = String::from(scad_path);
    let part = path
        .split('.')
        .next()
        .expect("Can not split filename at '.'");
    let mute = String::with_capacity(scad_path.len()).add(part).add(".stl");

    let output = Command::new("openscad")
        .arg("-o")
        .arg(&mute)
        .arg(scad_path)
        .output()
        .expect("Failed to execute OpenSCAD .stl render command");

    if output.status.success() {
        println!("STL render complete: {}", &scad_path);
        if image {
            render_image(&path, &part);
        }

        String::from(scad_path)
    } else {
        eprintln!(
            "{} model build error: {}",
            scad_path,
            String::from_utf8(output.stderr)
                .expect("Error during model render, but I can not show it")
        );

        String::from("Error")
    }
}

// Render the SCAD file as a PNG image
fn render_image(scad_path: &String, root: &str) {
    println!("Render image {}", scad_path);
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
    } else {
        eprintln!(
            "{} preview image error: {}",
            png,
            String::from_utf8(output.stderr)
                .expect("Error during image render, but I can not show it")
        );
    }
}
