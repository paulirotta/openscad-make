extern crate glob;
use self::glob::glob;
use rayon::prelude::*;
use std::ops::Add;
use std::process::Command;

fn main() {
    println!("All .SCAD files will be built");

    let paths: Vec<String> = glob("**/*.scad")
        .expect("Failed to read *.SCAD pattern")
        .map(|e| {
            String::from(
                e.expect("Can't get it")
                    .to_str()
                    .expect("Still can't get it"),
            )
        })
        .collect();

    let _result: Vec<String> = paths.par_iter().map(|path| make(path)).collect();

    println!();
    println!("End .SCAD file build");
}

// Make an .STL file from .SCAD
fn make(scad_path: &str) -> String {
    println!("Render {:?}", scad_path);

    let path = String::from(scad_path);
    let part = path.split('.').next().expect("Can not split");
    let mute = String::with_capacity(scad_path.len()).add(part).add(".stl");

    let output = Command::new("openscad")
        .arg("-o")
        .arg(&mute)
        .arg(scad_path)
        .output()
        .expect("Failed to execute OpenSCAD STL render command");

    if output.status.success() {
        render_image(&path, &part);

        String::from(scad_path)
    } else {
        eprintln!(
            "{:?}: {:?}",
            scad_path,
            String::from_utf8(output.stderr).expect("Can not convert stdout")
        );

        String::from("Error")
    }
}

// Render the SCAD file as a PNG image
fn render_image(scad_path: &String, root: &str) {
    println!("Render image {:?}", scad_path);
    let png = String::with_capacity(scad_path.len()).add(root).add(".png");

    let output = Command::new("openscad")
        .arg("--render")
        .arg("-o")
        .arg(&png)
        .arg(scad_path)
        .output()
        .expect("Failed to execute OpenSCAD PNG render command");

    if output.status.success() {
        println!("PNG render complete: {:?}", &scad_path);
    } else {
        eprintln!(
            "{:?}: {:?}",
            png,
            String::from_utf8(output.stderr).expect("Can not convert stdout")
        );
    }
}
