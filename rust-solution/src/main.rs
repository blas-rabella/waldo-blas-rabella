extern crate image;
extern crate rayon;

use image::{GrayImage};
use std::env;
mod solver;
use solver::{solve, check_error};

fn help() {
    println!("Waldo test solver.\nThe program needs the path to the two images to test. Ex: waldo-test img1 img2")
}

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        3 => {
            let img1_path = &args[1];
            let img2_path = &args[2];
            let (img1, img2) = read_images(&img1_path, &img2_path);
            let (i,j,err) = solve(&img1, &img2).expect("Template not found in image");
            if check_error(err, &img2){
                println!("Found template at pixel [{:?}]",(i,j))
            }else{
                println!("Template not found in image")
            }
           
        }
        _ => {
            help()
        }
    }
}

fn read_images(path1: &str, path2: &str) -> (GrayImage, GrayImage){
    let img1 = image::open(path1).expect("Couldn't read the first image").to_luma();
    let img2 = image::open(path2).expect("Couldn't read the second image").to_luma();
    let (w1, h1) = img1.dimensions();
    let (w2, h2) = img2.dimensions();
    if w1 < w2 || h1 < h2 {
        (img2, img1)
    } else {
        (img1, img2)
    }
}
