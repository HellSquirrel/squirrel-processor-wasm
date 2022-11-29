use squirrel_processor::*;

use image::{open, save_buffer, ColorType, GenericImage, GenericImageView, ImageBuffer, Rgba};

pub fn process_image() {
    let blur_kernel: Vec<Vec<f32>> = vec![
        vec![1.0 / 16.0, 1.0 / 8.0, 1.0 / 16.0],
        vec![1.0 / 8.0, 1.0 / 4.0, 1.0 / 8.0],
        vec![1.0 / 16.0, 1.0 / 8.0, 1.0 / 16.0],
    ];

    let mut img = open("dog.png").unwrap();
    let buf = img.as_bytes();
    let blurred_squirrel = blur_image(buf, img.width(), img.height());

    save_buffer(
        "dog-processed.png",
        blurred_squirrel.as_slice(),
        img.width(),
        img.height(),
        ColorType::Rgba8,
    );
}

#[cfg(test)]
#[test]
fn test_process_image() {
    process_image();
    assert_eq!(1, 2);
}
