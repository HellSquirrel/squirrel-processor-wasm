mod utils;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

fn conv_step(arr1: &[u8], kernel: &Vec<Vec<f32>>) -> u8 {
    let mut acc = 0.0;
    for (i, v) in kernel.iter().flatten().enumerate() {
        acc += *v * arr1[i] as f32;
    }
    acc as u8
}

pub fn convolve(
    array: &[u8],
    kernel: &Vec<Vec<f32>>,
    w: u32,
    h: u32,
    stride: u32,
    ch_in_image: u32,
) -> Vec<u8> {
    let mut result = vec![255; (w * h * ch_in_image) as usize];
    let kh = kernel.len() as u32;
    let kw = kernel[0].len() as u32;

    for i in 0..w - kw {
        for j in 0..h - kh {
            for c in 0..ch_in_image {
                let mut arr_to_convolve: Vec<u8> = Vec::with_capacity((kh * kw) as usize);
                for k in 0..kw {
                    for l in 0..kh {
                        arr_to_convolve.push(
                            array[((ch_in_image * w * j
                                + ch_in_image * i
                                + c
                                + ch_in_image * k
                                + ch_in_image * l * kw)
                                as usize)],
                        );
                    }
                }

                let conv_step_result = conv_step(&arr_to_convolve, kernel);
                result[(ch_in_image * w * j + ch_in_image * i + c) as usize] = conv_step_result;
            }
        }
    }

    result
}

#[wasm_bindgen]
pub fn blur_image(array: &[u8], image_width: u32, image_height: u32) -> Vec<u8> {
    let blur_kernel: Vec<Vec<f32>> = vec![
        vec![1.0 / 16.0, 1.0 / 8.0, 1.0 / 16.0],
        vec![1.0 / 8.0, 1.0 / 4.0, 1.0 / 8.0],
        vec![1.0 / 16.0, 1.0 / 8.0, 1.0 / 16.0],
    ];

    convolve(array, &blur_kernel, image_width, image_height, 2, 4)
}
