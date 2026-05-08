use crate::compressor::Compressor;

mod compressor;

fn main() {
    let _ = Compressor::compress(
        std::path::PathBuf::from("./assets/windark.png"),
        compressor::CompressionTechnique::Gaussian,
        std::path::PathBuf::from("./windark-comp.ppm"),
        9, // from 1 to 10
    );
}
