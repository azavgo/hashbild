use std::fs;
use std::io::Write; 

use crate::custom_error::HashBildError;

pub fn image_create() -> Vec<String> {
    const IMAGE_WIDTH: i32 = 255; 
    const IMAGE_HEIGHT: i32 = 255;

    let mut bild_vector: Vec<String> = Vec::new(); 
    
    bild_vector.push(format!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT));

    for j in (0..IMAGE_HEIGHT).rev() {
        for i in 0..IMAGE_WIDTH {
             let r = i as f64 / (IMAGE_WIDTH - 1) as f64; 
             let g = j as f64 / (IMAGE_HEIGHT - 1) as f64; 
             let b = 0.25;

             let ir = (255.999 * r) as i32;
             let ig = (255.999 * g) as i32;
             let ib = (255.999 * b) as i32;

             bild_vector.push(format!("{} {} {}\n", ir, ig, ib));
        }
    }

    bild_vector
}

pub fn bild(bild_vector: Vec<String>) -> Result<(), HashBildError> {
    
    let mut file = fs::File::create("image.ppm").unwrap();
    
    for e in bild_vector {
        write!(file, "{}" , e)?;
    }

    Ok(())
    
}