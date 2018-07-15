use image::{GrayImage};
use rayon::prelude::*;

pub fn check_error(err: u32, img: &GrayImage) -> bool{
    let (w,h) = img.dimensions();
    let threshold: u32 = w*h*5;
    if threshold > err {
        true
    }else{
        false
    }
    
}

pub fn solve(img1: &GrayImage, img2: &GrayImage) -> Option<(u32, u32, u32)>{
    let (_w1, h1) = img1.dimensions();
    let (_w2, h2) = img2.dimensions();
    let result = (0..h1-h2).into_par_iter()
        .filter_map( |j| {
            check_line(j, &img1, &img2)  
        }).min_by(|(_,_,sq1), (_,_,sq2)| sq1.cmp(sq2));

    result
}

fn check_line(j: u32, img1: &GrayImage, img2: &GrayImage) -> Option<(u32, u32, u32)> {
    let (w1, _h1) = img1.dimensions();
    let (w2, _h2) = img2.dimensions();
    let max_w = w1-w2;
    (0..max_w).into_par_iter().map( |i|{
        let sqt_error = check_img(i,j,&img1,&img2);
        (i,j,sqt_error)
    }).min_by(|(_,_,sq1), (_,_,sq2)| sq1.cmp(sq2))
    
}

fn check_img(i: u32, j: u32, img1: &GrayImage, img2: &GrayImage) -> u32{
    let (w, h) = img2.dimensions();
    let mut sum_err :u32 = 0;
    for i2 in 0..(w){
        for j2 in 0..(h){
//            println!("{:?} {:?}", (i+i2,j+j2), (i2,j2)); 
            let a = img2.get_pixel(i2,j2)[0] as u32;
            let b = img1.get_pixel(i+i2, j+j2)[0] as u32;
            let diff = (a-b).pow(2);
            sum_err = sum_err + diff;
         }
    }
    sum_err
}
