use thiserror::Error;

#[derive(Error, Debug)]
pub enum SDRError{
    #[error("Unknown error: `{0}`")]
    Unknown(String),
    #[error("Device Not Found")]
    NotFound,

}

pub type SDRResult<T> = Result<T, SDRError>;