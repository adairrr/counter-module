use abstract_add_on::AddOnError;
use cosmwasm_std::{CheckedFromRatioError, Decimal, DecimalRangeExceeded, OverflowError, StdError};
use cw_controllers::AdminError;
use thiserror::Error;

#[derive(Error, Debug, PartialEq)]
pub enum TemplateError {
    #[error("{0}")]
    Std(#[from] StdError),

    #[error("{0}")]
    Admin(#[from] AdminError),

    #[error("{0}")]
    DappError(#[from] AddOnError),

    #[error("{0}")]
    OverflowError(#[from] OverflowError),

    #[error("The configured max count has an error, {}", msg)]
    MaxCountError { msg: String },

    #[error("The update would exceed the configured max count")]
    ExceededMaxCount {},
}
