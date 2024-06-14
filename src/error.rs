#[derive(thiserror::Error, Debug)]
pub enum JourneyValidationError {
    #[error("Missing startTime")]
    MissingStartTime,
    #[error("Missing endTime")]
    MissingEndTime,

    #[error("Missing driver")]
    MissingDriver,
    #[error("Missing passenger")]
    MissingPassenger,
    #[error("Driver is passenger")]
    InvalidPassenger,

    #[error("Missing {0} trace")]
    MissingTrace(String),

    #[error("Empty trace {0}")]
    EmptyTrace(String),

    #[error("invalid json")]
    Serde(#[from] serde_json::Error),

    #[error("error while reading json file")]
    Io(#[from] std::io::Error),
}
