use std::fs;
use std::io::Write; 

use crate::custom_error::HashBildError;

pub struct Bild {
    pub hash_string: String, 
    pub image_width: u32, 
    pub image_height: u32,
}

impl Bild {
    pub fn new(hash_string: String, image_width: u32, image_height: u32) -> Bild {
        Bild { hash_string, 
               image_width, 
               image_height, 
            }
    }

    pub fn image_create(bild: &Bild) -> Vec<String> {
    
        let mut bild_vector: Vec<String> = Vec::new(); 
        
        bild_vector.push(format!("P3\n{} {}\n255\n", bild.image_width, bild.image_height));
    
        for j in (0..bild.image_height).rev() {
            for i in 0..bild.image_width {
                 let r = i as f64 / (bild.image_width - 1) as f64; 
                 let g = j as f64 / (bild.image_height - 1) as f64; 
                 let b = 0.25;
    
                 let ir = (255.999 * r) as i32;
                 let ig = (255.999 * g) as i32;
                 let ib = (255.999 * b) as i32;
    
                 bild_vector.push(format!("{} {} {}\n", ir, ig, ib));
            }
        }
    
        bild_vector
    }
}


pub fn bild_file(bild_vector: Vec<String>) -> Result<(), HashBildError> {
    
    let mut file = fs::File::create("image.ppm").unwrap();
    
    for e in bild_vector {
        write!(file, "{}" , e)?;
    }

    Ok(())
    
}