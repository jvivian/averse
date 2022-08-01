//!
//! Averse is a meal planner with designed to make storing, searching, viewing, and planning
//! meals for the week straight foward.  
//!
//! # Subcommands
//! The tool is split into 4 separate subcommands
//! - `add`     - Interactively define and save a recipe
//! - `view`    - Search through recipes/tags via `FuzzySearch`
//! - `plan`    - Plan a meal for the week and generate a grocery list
//! - `behold`  - Display weekly plan or view detailed breakdown by day
//!
//! # Installing Averse
//! `Averse` is compiled to an executable binary using `cargo`.
//! To build, clone the repository then type `cargo build --release`
//!
//! # Using Averse
//! Averse is designed to be run from the directory containing two subdirectories,
//! one that contain `recipes` and one that contains `plans`. See the
//! repository directories for examples.
//!
//! An example workflow for using Averse:
//!
//! 1. Recipes are interactively added via `averse add`
//! 2. Recipes are browsed via `averse view`
//! 3. Existing recipes are planned for the week using `averse plan`
//! 4. Meals for a week can be viewed using `averse behold`
//!
//!
//! # Example Commands
//! ```
//! averse add --recipe-dir /path/to/recipes --plan-dir /path/to/plans
//! averse view
//! averse plan --date 2022-07-31
//! averse behold
//! ```
//!

pub mod add;
pub mod behold;
pub mod errors;
pub mod plan;
pub mod utils;
pub mod view;

use crate::errors::{IngredientParsingError, RecipeParsingError};
use colored::*;
use serde::{Deserialize, Serialize};
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::path::PathBuf;
use std::str::FromStr;
use tabled::Tabled;

/// Valid units of measurement
const UNITS: [&str; 10] = [
    "can", "cup", "gallon", "gram", "item", "kg", "lb", "oz", "tsp", "tbsp",
];

/// Ordered days of the week
pub const WEEK: [&str; 7] = [
    "Sunday",
    "Monday",
    "Tuedsay",
    "Wednesday",
    "Thursday",
    "Friday",
    "Saturday",
];

/// Recipe contains all information for reproducing a recipe
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Recipe {
    /// Name of the recipe,
    name: String,
    ///
    tags: Vec<String>,
    ingredients: Vec<Ingredient>,
    steps: Vec<String>,
}

impl Recipe {
    /// Convert to RecipeRow for listing Recipes
    pub fn to_row(self, id: usize) -> RecipeRow {
        RecipeRow {
            ID: id,
            Name: self.name,
            Tags: self.tags.join(", "),
        }
    }
    /// Provide summary details of a recipe
    pub fn summary(&self) -> String {
        format!(
            "{:30} -- {}",
            self.name.replace("-", " "),
            self.tags.join(", ")
        )
    }
}

impl Display for Recipe {
    /// Print a human-readable version of a Recipe
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let name = self.name.purple();
        let tags = self.tags.join(", ").green();
        let ingredients = self
            .ingredients
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n⇒ ")
            .blue();
        let steps = self
            .steps
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join("\n🡢  ")
            .white();
        write!(
            f,
            "{name}\n\t{} {tags}\n\n{} {ingredients}\n\n🡢 {steps}\n",
            "→ Tags:".green(),
            "⇒".blue()
        )
    }
}

impl TryFrom<&PathBuf> for Recipe {
    type Error = RecipeParsingError;
    /// For deserializing a recipe from a path
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        if !path.exists() {
            panic!("The file does not exist dumbass: {path:?}")
        }
        Ok(serde_yaml::from_str(&fs::read_to_string(&path)?)?)
    }
}

/// Ingredient information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Ingredient {
    name: String,
    amount: f32,
    unit: Unit,
}

impl Display for Ingredient {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        write!(f, "{} {} {}", self.amount, self.unit, self.name)
    }
}

impl FromStr for Ingredient {
    type Err = IngredientParsingError;
    fn from_str(input: &str) -> Result<Ingredient, Self::Err> {
        let split = input.split(" ").collect::<Vec<_>>();
        let amount = split[0].parse::<f32>()?;
        let unit = split[1].parse::<Unit>()?;
        let name = split[2..].join(" ");
        Ok(Ingredient { name, amount, unit })
    }
}

/// Enum of all valid units used to describe ingredients
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Unit {
    Can,
    Cup,
    Gallon,
    Gram,
    Item,
    Kg,
    Lb,
    Oz,
    Tbsp,
    Tsp,
}

impl Display for Unit {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        match *self {
            Unit::Can => write!(f, "Can"),
            Unit::Cup => write!(f, "Cup"),
            Unit::Gallon => write!(f, "Gallon"),
            Unit::Gram => write!(f, "Gram"),
            Unit::Item => write!(f, "Item"),
            Unit::Kg => write!(f, "Kg"),
            Unit::Lb => write!(f, "Lb"),
            Unit::Oz => write!(f, "Oz"),
            Unit::Tsp => write!(f, "Tsp"),
            Unit::Tbsp => write!(f, "Tbsp"),
        }
    }
}

impl FromStr for Unit {
    type Err = IngredientParsingError;
    fn from_str(input: &str) -> Result<Unit, Self::Err> {
        match input.to_lowercase().as_str() {
            "can" => Ok(Unit::Can),
            "cup" => Ok(Unit::Cup),
            "gallon" => Ok(Unit::Gallon),
            "gram" => Ok(Unit::Gram),
            "item" => Ok(Unit::Item),
            "kg" => Ok(Unit::Kg),
            "lb" => Ok(Unit::Lb),
            "oz" => Ok(Unit::Oz),
            "tsp" => Ok(Unit::Tsp),
            "tbsp" => Ok(Unit::Tbsp),
            _ => Err(IngredientParsingError::InvalidUnit(input.into())),
        }
    }
}

/// Struct for printing Recipe summary information
#[allow(non_snake_case)]
#[derive(Tabled)]
pub struct RecipeRow {
    ID: usize,
    pub Name: String,
    Tags: String,
}

/// Struct for listing ingredients
#[allow(non_snake_case)]
#[derive(Tabled)]
pub struct IngredientRow {
    pub Name: String,
    pub Amount: String,
    pub Unit: String,
}

impl TryFrom<Ingredient> for IngredientRow {
    type Error = IngredientParsingError;
    fn try_from(ingr: Ingredient) -> Result<Self, Self::Error> {
        Ok(IngredientRow {
            Name: ingr.name.clone(),
            Amount: ingr.amount.to_string(),
            Unit: ingr.unit.to_string(),
        })
    }
}

/// Struct for listing steps in a recipe
#[allow(non_snake_case)]
#[derive(Tabled)]
pub struct StepRow {
    pub Step: u16,
    pub Details: String,
}

/// Struct for displaying meal plans for the week
#[allow(non_snake_case)]
#[derive(Tabled)]
pub struct PlanRow {
    Date: String,
    Sunday: String,
    Monday: String,
    Tuesday: String,
    Wednesday: String,
    Thursday: String,
    Friday: String,
    Saturday: String,
}

/// Struct for listing groceries for the week
#[allow(non_snake_case)]
#[derive(Debug, Tabled)]
pub struct GroceryRow {
    Id: usize,
    Amount: f32,
    Unit: Unit,
    Ingredient: String,
}
