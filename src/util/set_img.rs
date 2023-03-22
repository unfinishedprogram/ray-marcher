use std::io::Cursor;

use image::RgbImage;

pub fn show_image(image: RgbImage) {
    let mut image_data: Vec<u8> = Vec::new();
    image
        .write_to(
            &mut Cursor::new(&mut image_data),
            image::ImageOutputFormat::Png,
        )
        .expect("Failed to write image");

    let base64 = format!("data:image/png;base64,{}", base64::encode(image_data));
    log::info!("{base64}");

    web_sys::window()
        .unwrap()
        .document()
        .unwrap()
        .get_element_by_id("img")
        .unwrap()
        .set_attribute("src", &base64)
        .expect("Failed to show image")
}
