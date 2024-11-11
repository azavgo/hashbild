//Custom error
#[derive(Debug)]
pub enum HashBildError { 
    IOError(std::io::Error),  
}

impl From<std::io::Error> for HashBildError {
    fn from(error: std::io::Error) -> Self {
        HashBildError::IOError(error)
    }
}