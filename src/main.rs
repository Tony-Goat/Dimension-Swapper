use std::fs;
use std::path::PathBuf;

use image::imageops::FilterType;
use image::imageops::{resize, rotate270};
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

    //Resize, rotate, and flip all of the images so they're as many pixels wide as there are frames
    let resized_imgs: Vec<RgbImage> = src_imgs
        .into_par_iter()
        .progress_count(time_dim as u64)
        .map(|i| {
            i.resize_exact(time_dim, old_height, FilterType::CatmullRom)
                .rotate90()
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

const BPP: u32 = 3;

fn resize_worker(src_imgs: &[RgbImage], time: u32, width: u32, height: u32, frame: u32) -> u32 {
    //When we're working with the images, they're rotated 90 degrees clockwise
    //so X is from top to bottom, this takes advantage of image byte ordering to
    //more quickly copy rows
    let column_pitch = BPP * height;
    let source_column = ((frame - 1) * column_pitch) as usize..(frame * column_pitch) as usize;

    let mut new_img_buffer: Vec<u8> = vec![0; (column_pitch * time) as usize];
    for x in 0..time {
        let destination_column = (x * column_pitch) as usize..((x + 1) * column_pitch) as usize;
        new_img_buffer[destination_column]
            .copy_from_slice(&src_imgs[x as usize].as_raw()[source_column.clone()]);
    }

    let mut new_img = RgbImage::from_raw(height, time, new_img_buffer).unwrap();

    //Unrotate and unresize to the proper dimensions
    new_img = rotate270(&new_img);
    new_img = resize(&new_img, width, height, FilterType::CatmullRom);

    //Figure out the proper path
    let mut dst_path: PathBuf = PathBuf::new();
    dst_path.push(DST_FOLDER);
    dst_path.push(format!("{:06}.png", frame));
    new_img.save(dst_path).unwrap();
    1
}
