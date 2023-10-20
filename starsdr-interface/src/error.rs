use thiserror::Error;

#[derive(Error, Debug)]
pub enum SDRError {
    #[error("Unknown error: `{0}`")]
    Unknown(String),
    #[error("Device not found")]
    NotFound,
    #[error("Device not open")]
    NotOpen,
    #[error("Not support: `{0}`")]
    NotSupport(String),
    #[error("Param[{key:?}=\"{value:?}\"] error: {msg:?}")]
    Param {
        key: String,
        value: String,
        msg: String,
    },
}

pub type SDRResult<T> = Result<T, SDRError>;
