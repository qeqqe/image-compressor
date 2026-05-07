use crate::compressor::Compressor;

mod compressor;

fn main() {
    let _ = Compressor::compress(
        std::path::PathBuf::from("./assets/vro.ppm"),
        compressor::CompressionTechnique::Gaussian,
        std::path::PathBuf::from("./new.ppm"),
        1,
    );
}
