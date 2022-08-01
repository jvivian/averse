//! Module for viewing recipes using `fuzzy` search
use crate::errors::RecipeParsingError;
use crate::utils::{get_jsons, input_msg, summarize_recipes, title};
use crate::Recipe;
use dialoguer::{theme, FuzzySelect};
use std::path::Path;

/// Logic for displaying recipes
pub fn display_recipes(recipe_dir: &String) -> Result<(), RecipeParsingError> {
    let base = "\t\u{21F8} View Recipes\n\n";
    let mainscr = format!("{base}Type to search recipes then hit ENTER\n\n");
    let recipe_paths = get_jsons(Path::new(&recipe_dir))?;
    let recipe_summaries = summarize_recipes(&recipe_dir)?;
    let mut recipe: Option<Recipe> = None;
    loop {
        title(&mainscr);
        if let Some(ref rec) = recipe {
            println!("{rec}");
            input_msg("Hit ENTER to search for another recipe")?;
        }
        title(&mainscr);
        let select_idx = FuzzySelect::with_theme(&theme::ColorfulTheme::default())
            .items(&recipe_summaries)
            .default(0)
            .interact()
            .unwrap();
        recipe = Recipe::try_from(&recipe_paths[select_idx]).ok();
    }
}
