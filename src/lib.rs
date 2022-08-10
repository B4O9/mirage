pub fn blur(infile: String, outfile: String, blur: f32) {
    // opening an existing image file
    let img = image::open(infile).expect("Failed to open INFILE.");
    let img2 = img.blur(blur);
    // saving an image to a file.
    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn brighten(infile: String, outfile: String, bright: i32) {

    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = img.brighten(bright);

    img2.save(outfile).expect("Failed writing.");
}

pub fn crop(infile: String, outfile: String, x: u32, y: u32, width: u32, height: u32) {

    let mut img = image::open(infile).expect("Failed to open INFILE.");

    // .crop() takes four arguments: x: u32, y: u32, width: u32, height: u32
    let img2 = img.crop(x, y, width, height);

    
    img2.save(outfile).expect("Failed writing.");
}

pub fn rotate(infile: String, outfile: String, rotation: u32) {
    
    let img = image::open(infile).expect("Failed to open INFILE.");

    // There are 3 rotate functions to choose from (all clockwise):
    //   .rotate90()
    //   .rotate180()
    //   .rotate270()
    let temp_img;

    match rotation {
        90 => {
            temp_img = img.rotate90();
        }
        180 => {
            temp_img = img.rotate180();
        }
        270 => {
            temp_img = img.rotate270();
            
        }
        _ => panic!("Invalid rotation value"),
    };
    

    let img2 = temp_img;
    
    img2.save(outfile).expect("Failed writing.");
}

pub fn invert(infile: String, outfile: String) {

    let mut img = image::open(infile).expect("Failed to open INFILE.");

    img.invert();

    img.save(outfile).expect("Failed writing OUTFILE.");

}

pub fn grayscale(infile: String, outfile: String) {

    let img = image::open(infile).expect("Failed to open INFILE.");

    let img2 = img.grayscale();

    img2.save(outfile).expect("Failed writing OUTFILE.");
}

pub fn generate(outfile: String, width: u32, height: u32) {

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        let green = (0.3 * 0.0) as u8;

        

        *pixel = image::Rgb([red, green, blue]);
    }

    
    imgbuf.save(outfile).unwrap();
}

pub fn fractal(outfile: String, width: u32, height: u32) {

    let mut imgbuf = image::ImageBuffer::new(width, height);


    let scale_x = 5.0 / width as f32;
    let scale_y = 1.5 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // the fractal math 
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;


        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(outfile).unwrap();
}
