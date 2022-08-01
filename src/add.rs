//! Module for interactively adding recipes
use crate::utils::{get_recipe_out_path, input_msg, print_table, title};
use crate::{Ingredient, IngredientRow, Recipe, StepRow};
use colored::*;
use dialoguer::Input;
use std::fs;
use std::io;
use std::str::FromStr;

/// Adds recipe to `recipe_dir` interactively.
/// Displays a table of current ingredients / steps
pub fn add_recipe(recipe_dir: &String) -> io::Result<()> {
    title("\t\u{21F8} Recipe Name\n\n");
    let name: String = Input::new()
        .with_prompt("Enter recipe name")
        .interact_text()?;
    let recipe_path = get_recipe_out_path(&recipe_dir, &name);
    let tags = add_tags().expect("Failed to parse tags");
    let ingredients = add_ingredients().expect("Failed adding ingredients");
    let steps = add_steps().expect("Failed adding steps");
    let recipe = Recipe {
        name,
        tags,
        ingredients,
        steps,
    };
    let serialized = serde_yaml::to_string(&recipe).expect("Failed to serialize recipe");
    fs::write(&recipe_path, &serialized).expect("Failed to save recipe");
    println!("Recipe saved to {}", recipe_path.to_str().unwrap());
    Ok(())
}

/// Ask user to input tags for recipe
fn add_tags() -> io::Result<Vec<String>> {
    title("\t\u{21F8} Tags\n\n");
    Ok(input_msg("Enter associated tags (e.g. soup, mealprep)")?
        .split(", ")
        .map(|s| s.to_owned())
        .collect())
}

/// Ask user to add ingredient with loop for bad input
fn add_ingredients() -> io::Result<Vec<Ingredient>> {
    let base = "\t\u{21F8} Ingredients\n\n";
    let mut rows: Vec<IngredientRow> = vec![];
    let mut ingredients: Vec<Ingredient> = vec![];
    title(&format!(
        "{}<AMOUNT> <UNIT> <INGREDIENT> (Ex: 1 lb beef)",
        base
    ));
    loop {
        if !ingredients.is_empty() {
            print_table(&rows);
        }
        let ingredient_string: String = input_msg("Enter ingredient (or ENTER to continue)")?;
        if ingredient_string.is_empty() {
            break;
        }
        let ingredient = match Ingredient::from_str(&ingredient_string) {
            Ok(ingr) => ingr,
            Err(e) => {
                title(&format!("{} (or ENTER to continue)", base));
                println!("{e}\n{}\n", "...Please try again.".red());
                continue;
            }
        };
        ingredients.push(ingredient.clone());
        rows.push(ingredient.try_into().expect("IngredientRow failed"));
        title(&format!("{} (or ENTER to continue)", base));
    }
    Ok(ingredients)
}

/// Ask user for recipe steps
fn add_steps() -> io::Result<Vec<String>> {
    let msg = "\t\u{21F8} Steps\n\n";
    title(msg);
    let mut steps: Vec<String> = vec![];
    let mut rows: Vec<StepRow> = vec![];
    let mut i: u16 = 1;
    loop {
        if !steps.is_empty() {
            print_table(&rows);
        }
        let step: String = input_msg("Enter step (or ENTER to quit)")?;
        if step.is_empty() {
            break;
        };
        steps.push(step.clone());
        rows.push(StepRow {
            Step: i,
            Details: step,
        });
        i += 1;
        title(msg);
    }
    Ok(steps)
}
