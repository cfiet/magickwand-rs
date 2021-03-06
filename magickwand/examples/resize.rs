extern crate magickwand;

use magickwand::{ColorspaceType, FilterType, MagickWand};

fn main() {
    let wand = MagickWand::new()
        .unwrap_or_else(|e| panic!("Error while creating MagickWand: {:?}", e))
        .read_image("logo:")
        .unwrap_or_else(|e| panic!("Error while reading image: {:?}", e))
        .resize_image(320, 240, FilterType::LanczosFilter)
        .unwrap_or_else(|e| panic!("Error while resizing image: {:?}", e))
        .set_image_compression_quality(100)
        .unwrap_or_else(|e| panic!("Error while setting image compression quality: {:?}", e))
        .set_colorspace(ColorspaceType::SRGB)
        .unwrap_or_else(|e| panic!("Error while setting colorspace: {:?}", e))
        .write_image(r"resized.png")
        .unwrap_or_else(|e| panic!("Error while writing image: {:?}", e));

    println!("Dimensions: {:?}", wand.get_colorspace());
}
