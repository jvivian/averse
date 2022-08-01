//! Module to BEHOLD your meal plan creations
use crate::plan::Plan;
use crate::utils::{get_jsons, get_recipe_out_path, print_table, select, title};
use crate::{PlanRow, Recipe, RecipeParsingError};
use std::path::Path;

/// Logic to display plans
pub fn display_plan(
    recipe_dir: &String,
    plan_dir: &String,
    n_plans: &usize,
) -> Result<(), RecipeParsingError> {
    title("\t\u{21F8} Behold\n\n");
    let plans = get_latest_plans(plan_dir, n_plans)?;
    let rows: Vec<PlanRow> = plans.iter().map(|x| PlanRow::from(x.clone())).collect();
    print_table(&rows);

    // Select Plan
    let plan_names = plans
        .iter()
        .map(|x| x.name.clone())
        .collect::<Vec<String>>();
    let plan = &plans[select(&plan_names)?];

    // Select Day
    let keys = plan.recipes.keys().cloned().collect::<Vec<String>>();
    let day = &keys[select(&keys)?];
    let recipe_names = plan.recipes.get(day).unwrap();

    // Pick Recipe
    let name = &recipe_names[select(recipe_names)?];
    let recipe = Recipe::try_from(&get_recipe_out_path(&recipe_dir, &name))?;
    println!("{recipe}");

    Ok(())
}

/// Fetches latest N plans
fn get_latest_plans(plan_dir: &str, n_plans: &usize) -> Result<Vec<Plan>, RecipeParsingError> {
    let mut plan_paths = get_jsons(Path::new(&plan_dir))?;
    if plan_paths.len() > *n_plans {
        let idx = n_plans.min(&plan_paths.len()).clone();
        plan_paths = plan_paths[..idx].to_vec();
    }
    plan_paths.iter().map(|x| Plan::try_from(x)).collect()
}
