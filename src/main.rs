use std::env::{self, args};

mod custom_error;
use custom_error::HashBildError;

mod bild;
use bild::{bild_file, Bild};


fn main() -> Result<(), HashBildError> {
    
    let args: Vec<String> = env::args().collect(); 

    match args.len() {
        2 => {
                let bild = Bild::new(args[1].clone(), 255, 255); 
                let bild_vector: Vec<String> = Bild::image_create(&bild);
                bild_file(bild_vector)?;
            },
        _ => {
                println!("Hash was not supplied!")
            },
    }

    Ok(())
}
