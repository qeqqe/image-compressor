#![allow(dead_code)]

use std::{error::Error, path::PathBuf};

use image::{DynamicImage, ImageBuffer, ImageReader, RgbImage};
use ndarray::{Array, Array2, Dimension};

pub enum CompressionTechnique {
    Gaussian,
    MaxPolling,
}

pub struct Compressor;

impl Compressor {
    pub fn compress(
        path: PathBuf,
        tech: CompressionTechnique,
        destination: PathBuf,
        compression_level: u8,
    ) -> Result<(), Box<dyn Error>> {
        let img = ImageReader::open(path).unwrap().decode().unwrap();

        match tech {
            CompressionTechnique::Gaussian => {
                Compressor::gaussian_weighted_compression(img, destination, compression_level)
            }
            CompressionTechnique::MaxPolling => todo!(),
        }
    }

    pub fn gaussian_weighted_compression(
        img: DynamicImage,
        destination: PathBuf,
        compression_level: u8,
    ) -> Result<(), Box<dyn Error>> {
        let img = img.to_rgb8();

        let compression_factor = 1.0 - (compression_level as f32 / 10.0);
        let original_dimensions = img.dimensions();

        let n_width = (original_dimensions.0 as f32 * compression_factor) as u32;
        let n_height = (original_dimensions.1 as f32 * compression_factor) as u32;

        let out = RgbImage::new(n_width, n_height);

        todo!()
    }

    fn build_gaussian_kenel(k: usize, sigma: f32) -> Array2<u8> {
        let mut kernel: Array2<u8> = Array2::zeros((k, k));

        let cx = k / 2;
        let cy = k / 2;

        let mut sum = 0.0;

        for y in 0..k {
            for x in 0..k {
                let dx = (x - cx) as f32;
                let dy = (y - cy) as f32;

                kernel[[y, x]] = (-(dx.powi(2) + dy.powi(2)) / (2.0 * sigma.powf(2.0))).exp() as u8;
                sum += kernel[[y, x]] as f32;
            }
        }

        for elem in kernel.iter_mut() {
            *elem = (*elem as f32 / sum) as u8;
        }

        kernel
    }

    pub fn max_polling(img: DynamicImage) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
