use thiserror::Error;

#[derive(Error, Debug)]
pub enum IkigaiError {
    #[error("Invalid date")]
    InvalidDate,
}

pub type IkigaiResult<T> = Result<T, IkigaiError>;
