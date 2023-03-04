use std::fs;
use std::path::PathBuf;

use image::imageops::resize;
use image::imageops::FilterType;
use image::io::Reader as ImageReader;
use image::DynamicImage;
use image::RgbImage;

use indicatif::ParallelProgressIterator;

use rayon::prelude::*;

const SRC_FOLDER: &str = "Source";
const DST_FOLDER: &str = "Destination";

fn main() {
    //Get a list of files in the source folder and turn it into a vector of paths
    let mut src_img_paths: Vec<PathBuf> = fs::read_dir(SRC_FOLDER)
        .unwrap()
        .map(|p| p.unwrap().path())
        .collect();
    src_img_paths.sort();

    //Start loading images
    let src_img_path_len = src_img_paths.len() as u64;
    let src_imgs: Vec<DynamicImage> = src_img_paths
        .into_par_iter()
        .progress_count(src_img_path_len)
        .map(|path| ImageReader::open(path).unwrap().decode().unwrap())
        .collect();

    //Store all of the old values for later
    let old_width = src_imgs[0].width();
    let old_height = src_imgs[0].height();
    let time_dim = src_imgs.len() as u32;

    //Resize all of the images so they're as many pixels wide as there are frames
    let resized_imgs: Vec<RgbImage> = src_imgs
        .into_par_iter()
        .progress_count(time_dim as u64)
        .map(|i| {
            i.resize_exact(time_dim, old_height, FilterType::CatmullRom)
                .into_rgb8()
        })
        .collect();

    let p: u32 = (1..=time_dim)
        .into_par_iter()
        .progress_count(time_dim as u64)
        .map(|f| resize_worker(&resized_imgs, time_dim, old_width, old_height, f))
        .sum();

    println!("{p} frames processed successfully.");
}

fn resize_worker(src_imgs: &[RgbImage], time: u32, width: u32, height: u32, frame: u32) -> u32 {
    //Create the new image buffer with the right size
    let mut new_img = RgbImage::new(time, height);
    for x in 0..time {
        for y in 0..height {
            new_img.put_pixel(x, y, *src_imgs[x as usize].get_pixel(frame - 1, y));
        }
    }

    //Downsize to the proper dimension
    new_img = resize(&new_img, width, height, FilterType::CatmullRom);

    //Figure out the proper path
    let mut dst_path: PathBuf = PathBuf::new();
    dst_path.push(DST_FOLDER);
    dst_path.push(format!("{:06}.png", frame));
    new_img.save(dst_path).unwrap();
    1
}
