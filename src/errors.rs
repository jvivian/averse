//! Minimal error definitions using `thiserror`
use crate::UNITS;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RecipeParsingError {
    #[error("Failed to read/write recipe file")]
    IOError(#[from] std::io::Error),
    #[error("Failed to serialize/deserialize recipe file")]
    DeserializeError(#[from] serde_yaml::Error),
}

#[derive(Debug, Error)]
pub enum IngredientParsingError {
    #[error("AMOUNT must be a valid number")]
    InvalidAmount(#[from] std::num::ParseFloatError),
    #[error("{0} invalid UNIT - must be one of: {UNITS:?}")]
    InvalidUnit(String),
    #[error("No ingredient provided")]
    NoIngredient,
}
