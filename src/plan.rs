//! Module for planning recipes for the week
use crate::errors::RecipeParsingError;
use crate::utils::{
    fuzzy_select, get_recipe_out_path, print_table, recipe_name_from_summary, summarize_recipes,
    title,
};
use crate::{GroceryRow, Ingredient, PlanRow, Recipe, WEEK};
use colored::*;
use dialoguer::Confirm;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fmt::{Display, Formatter, Result as FmtResult};
use std::fs;
use std::io;
use std::path::Path;
use std::path::PathBuf;
use tabled::{object::Columns, Format, Modify, Style, Table};

/// Logic for week planning
pub fn plan_week(
    recipe_dir: &String,
    plan_dir: &String,
    date: &String,
) -> Result<(), RecipeParsingError> {
    title("\t\u{21F8} Plan\n\n");
    Plan::new(date, recipe_dir, plan_dir)
        .add_recipes()?
        .compile_groceries()
        .print_grocery_list()
        .write()?;
    Ok(())
}

/// Associates recipes with days of the week
#[derive(Debug, Default, Clone, Serialize, Deserialize)]
pub struct Plan {
    /// Date in the form of YYYY-MM-DD is used as file name
    pub name: String,
    /// Day of week -> List of recipe names
    pub recipes: HashMap<String, Vec<String>>,
    /// Contains the distilled set of groceries
    #[serde(skip)]
    groceries: Vec<Ingredient>,
    /// Path to recipe directory
    #[serde(skip)]
    recipe_dir: String,
    /// Path to plan directory
    #[serde(skip)]
    plan_dir: String,
}

impl Plan {
    /// Creates a new Plan given a name, recipe directory, and plan directory
    fn new(name: &String, recipe_dir: &String, plan_dir: &String) -> Self {
        Plan {
            name: name.into(),
            recipe_dir: recipe_dir.into(),
            plan_dir: plan_dir.into(),
            ..Default::default()
        }
    }

    /// Associates recipes with days of the week
    fn add_recipes(&mut self) -> io::Result<&mut Self> {
        let summaries =
            summarize_recipes(&self.recipe_dir).expect("Failed to fetch recipe summaries");
        loop {
            title("\t\u{21F8} Plan\n\nSelect Day");
            print_table(&vec![PlanRow::from(self.clone())]);
            let day_idx = fuzzy_select(&WEEK)?;
            let recipe_idx = fuzzy_select(&summaries)?;
            self.recipes
                .entry(WEEK[day_idx].into())
                .or_insert(vec![])
                .push(recipe_name_from_summary(&summaries[recipe_idx]).unwrap());
            if !Confirm::new()
                .with_prompt("Add another recipe?")
                .interact()?
            {
                break;
            }
        }
        Ok(self)
    }

    /// Convert Plan to vector of Recipes
    fn to_recipes(&self) -> Vec<Recipe> {
        self.recipes
            .iter()
            .flat_map(|(_, v)| {
                v.iter()
                    .map(|x| Recipe::try_from(&get_recipe_out_path(&self.recipe_dir, x)).unwrap())
            })
            .collect()
    }

    /// Compiles groceries from a list of recipes
    fn compile_groceries(&mut self) -> &Self {
        let mut ingr_map: HashMap<String, Ingredient> = HashMap::new();
        self.to_recipes().iter().for_each(|recipe: &Recipe| {
            recipe.ingredients.iter().for_each(|ingr| {
                let key = format!("{}_{}", ingr.name, ingr.unit);
                ingr_map.entry(key).or_insert(ingr.clone());
            })
        });
        self.groceries = ingr_map
            .into_iter()
            .map(|(_, v)| v)
            .collect::<Vec<Ingredient>>();
        self
    }

    /// Prints grocery list
    fn print_grocery_list(&self) -> &Self {
        let table = Table::new(self.get_grocery_table())
            .with(Style::psql())
            .with(Modify::new(Columns::single(1)).with(Format::new(|s| s.red().to_string())));
        println!("{table}");
        self
    }

    /// Generates table from a set of groceries
    fn get_grocery_table(&self) -> Vec<GroceryRow> {
        self.groceries
            .iter()
            .enumerate()
            .map(|(i, ingr)| GroceryRow {
                Id: i,
                Amount: ingr.amount,
                Unit: ingr.unit.clone(),
                Ingredient: ingr.name.clone(),
            })
            .collect()
    }

    /// Write plan to disk
    fn write(&self) -> Result<String, RecipeParsingError> {
        let outpath = Path::new(&self.plan_dir)
            .join(&self.name)
            .with_extension("yaml");
        let serialized = serde_yaml::to_string(&self)?;
        fs::write(&outpath, &serialized)?;
        println!("Recipe saved to {}", outpath.to_str().unwrap());
        Ok(outpath.to_string_lossy().to_string())
    }
}

impl Display for Plan {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        let table = Table::new(vec![PlanRow::from(self.clone())])
            .with(Style::psql())
            .to_string();
        write!(f, "{table}")
    }
}

impl TryFrom<&PathBuf> for Plan {
    type Error = RecipeParsingError;
    fn try_from(path: &PathBuf) -> Result<Self, Self::Error> {
        if !path.exists() {
            panic!("The file does not exist: {path:?}")
        }
        Ok(serde_yaml::from_str(&fs::read_to_string(&path)?)?)
    }
}

impl From<Plan> for PlanRow {
    fn from(p: Plan) -> Self {
        let default = vec![String::from("")];
        PlanRow {
            Date: p.name,
            Sunday: p.recipes.get("Sunday").unwrap_or(&default).join("\n "),
            Monday: p.recipes.get("Monday").unwrap_or(&default).join("\n "),
            Tuesday: p.recipes.get("Tuesday").unwrap_or(&default).join("\n "),
            Wednesday: p.recipes.get("Wednesday").unwrap_or(&default).join("\n "),
            Thursday: p.recipes.get("Thursday").unwrap_or(&default).join("\n "),
            Friday: p.recipes.get("Friday").unwrap_or(&default).join("\n "),
            Saturday: p.recipes.get("Saturday").unwrap_or(&default).join("\n "),
        }
    }
}
