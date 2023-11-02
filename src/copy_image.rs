use clippers::Clipboard;
use image::{open, GenericImageView, RgbaImage};

fn get_image_dimensions(path: &str) -> Result<(u32, u32), image::ImageError> {
    let img = image::open(path)?;
    Ok(img.dimensions())
}

pub fn clipboard_image_copy(path: &str) {
    let img = match open(path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Image Error: {}", e);
            return;
        }
    };

    let mut image_width:u32 = 0;
    let mut image_height:u32 = 0;
    match get_image_dimensions(path) {
        Ok((width, height)) =>  {
            image_width = width;
            image_height = height;
        }
        Err(e) => eprintln!("Image Error: {}", e)
    }

    if image_height == 0 || image_height == 0 {
        eprintln!("Error: Image is of nonsensical size");
        return;
    }
    let mut clipboard = Clipboard::get();
    let image: RgbaImage = img.to_rgba8();
    clipboard.write_image(image_width, image_height, &image).expect("Error writing to clipboard");

}
