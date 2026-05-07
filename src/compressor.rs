#![allow(dead_code)]

use std::{error::Error, path::PathBuf};

use image::{DynamicImage, ImageReader, Rgb, RgbImage};
use ndarray::Array2;

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
        if compression_level > 10 {
            return Err("Compression level must be between 0 and 10".into());
        }
        let img = img.to_rgb8();

        let compression_factor = 1.0 - (compression_level as f32 / 10.0);

        let original_dimensions = img.dimensions();

        let n_width = (original_dimensions.0 as f32 * compression_factor) as u32;
        let n_height = (original_dimensions.1 as f32 * compression_factor) as u32;

        let mut out = RgbImage::new(n_width, n_height);

        let downscale_ratio = 1.0 / compression_factor;
        let sigma = downscale_ratio / 2.0;

        let stride = downscale_ratio.round() as u32;
        let k = if stride.is_multiple_of(2) {
            stride + 1
        } else {
            stride
        } as usize;

        let kernel = Self::build_gaussian_kernel(k, sigma);

        for oy in 0..n_height {
            for ox in 0..n_width {
                let cx = (ox as f32 * downscale_ratio) as i32;
                let cy = (oy as f32 * downscale_ratio) as i32;

                let half_k = (k / 2) as i64;

                let mut acc = [0.0f32; 3];

                for ky in 0..k {
                    for kx in 0..k {
                        let ix = (cx + kx as i32 - half_k as i32)
                            .clamp(0, original_dimensions.0 as i32 - 1)
                            as u32;
                        let iy = (cy + ky as i32 - half_k as i32)
                            .clamp(0, original_dimensions.1 as i32 - 1)
                            as u32;
                        let px = img.get_pixel(ix, iy);
                        let w = kernel[[ky, kx]];
                        acc[0] += px[0] as f32 * w;
                        acc[1] += px[1] as f32 * w;
                        acc[2] += px[2] as f32 * w;
                    }
                }

                out.put_pixel(
                    ox,
                    oy,
                    Rgb([
                        acc[0].round().clamp(0.0, 255.0) as u8,
                        acc[1].round().clamp(0.0, 255.0) as u8,
                        acc[2].round().clamp(0.0, 255.0) as u8,
                    ]),
                );
            }
        }
        out.save(destination)?;

        Ok(())
    }

    fn build_gaussian_kernel(k: usize, sigma: f32) -> Array2<f32> {
        let mut kernel: Array2<f32> = Array2::zeros((k, k));
        let cx = (k / 2) as isize;
        let cy = (k / 2) as isize;
        let mut sum = 0.0f32;

        for y in 0..k {
            for x in 0..k {
                let dx = (x as isize - cx) as f32;
                let dy = (y as isize - cy) as f32;
                let val = (-(dx.powi(2) + dy.powi(2)) / (2.0 * sigma.powi(2))).exp();
                kernel[[y, x]] = val;
                sum += val;
            }
        }

        kernel.mapv_inplace(|v| v / sum);
        kernel
    }

    pub fn max_polling(img: DynamicImage) -> Result<(), Box<dyn Error>> {
        todo!()
    }
}
