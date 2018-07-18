use image::GrayImage;
use ndarray::prelude::*;
use rayon::prelude::*;

pub fn check_error(err: u32, img: &GrayImage) -> bool {
    let (w, h) = img.dimensions();
    let threshold: u32 = w * h * 6; // This needs a better threshold...
     if threshold > err {
        true
    } else {
        false
    }
}

fn get_isize(img: &GrayImage) -> (isize, isize) {
    let (w, h) = img.dimensions();
    (w as isize, h as isize)
}

pub fn solve(img1: &GrayImage, img2: &GrayImage) -> Option<(isize, isize, u32)> {
    let (w1, h1) = get_isize(&img1);
    let (w2, h2) = get_isize(&img2);

    let (a1, a2) = get_arrays(&img1, &img2);
    
    (0..h1 - h2)
        .into_par_iter()
        .filter_map(|j| check_line(j, w1 - w2, w2, h2, &a1, &a2))
        .min_by(|(_, _, sq1), (_, _, sq2)| sq1.cmp(sq2))
}

fn check_line(
    j: isize,
    max: isize,
    w2: isize,
    h2: isize,
    a1: &Array2<u32>,
    a2: &Array2<u32>,
) -> Option<(isize, isize, u32)> {
    (0..max)
        .map(|i| {
            let sub_image = a1.slice(s![j..(j + h2), i..(i + w2)]);
            let err = compute_error(&sub_image, &a2);
            (j, i, err)
        })
        .min_by(|(_, _, sq1), (_, _, sq2)| sq1.cmp(sq2))
}

fn compute_error(img: &ArrayView2<u32>, template: &Array2<u32>) -> u32 {
     (template - img).mapv(|a| a.pow(2)).scalar_sum()
}

fn get_arrays(img1: &GrayImage, img2: &GrayImage) -> (Array2<u32>, Array2<u32>) {
    let (w1, h1) = img1.dimensions();
    let (w2, h2) = img2.dimensions();
    let a1 = Array::from_iter(img1.pixels().map(|p| p[0] as u32))
        .into_shape((h1 as usize, w1 as usize))
        .expect("Problem converting to array");
    let a2 = Array::from_iter(img2.pixels().map(|p| p[0] as u32))
        .into_shape((h2 as usize, w2 as usize))
        .expect("Problem converting to array");
    (a1, a2)
}
