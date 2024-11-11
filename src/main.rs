use std::env;

mod custom_error;
use custom_error::HashBildError;

mod bild;
use bild::{bild, image_create}; 


fn main() -> Result<(), HashBildError> {
    
    let args: Vec<String> = env::args().collect(); 
    
    match args.len() {
        1 => println!("Hash was not supplied"), 
        _ => println!("Hash is {}", args[1]),
    }
     
    bild(image_create())?;

    Ok(())
}
