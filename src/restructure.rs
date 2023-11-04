fn select_image_for_copy(images: &Vec<Image>) {
    for image in images {
        println!("{}", image);
    };

    match images.len() {
        0 => println!("No images found under that name"),
        1 => {
            copy_image::clipboard_image_copy(&images[0].path);
            println!("Image copied to clipboard");
        },
        _ => {
            println!("Select image for copy");
            for (index, image) in images.iter().enumerate() {
                println!("{}. {}", index, image.name);
            }

        }
    }

}