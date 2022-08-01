//! Collection of utility functions
use crate::{Recipe, RecipeParsingError};
use colored::*;
use console::{Emoji, Term};
use dialoguer::{theme::ColorfulTheme, FuzzySelect, Input, Select};
use std::fs;
use std::io::{self};
use std::path::{Path, PathBuf};
use tabled::{Style, Table, Tabled};

/// Displays tool title after clearing terminal
pub fn title(msg: &str) {
    let term = Term::stdout();
    term.clear_screen().unwrap();
    println!(
        "{} {}v{}{}{}{} {} {} {} {}{}\n{}",
        "\u{222E}".purple(),
        "\u{0104}",
        "\u{0119}",
        "\u{0155}",
        "\u{015B}",
        "\u{0113}",
        "\u{2563}".purple(),
        "A Meal Planner".cyan(),
        "\u{2560}".purple(),
        "made with Rust".italic().red(),
        Emoji("🦀", ""),
        msg.green()
    );
    term.flush().unwrap();
}

/// Simple horizontal divider
pub fn divider() {
    println!("{}\n", "\u{2322}\u{2323}\u{2322}".repeat(23).cyan());
}

/// Prompts user for input
pub fn get_user_input() -> io::Result<String> {
    let mut line = String::new();
    io::stdin().read_line(&mut line)?;
    Ok(line.trim().to_string())
}

/// Prints a table from a collection of objects which implement the `Tabled` trait
pub fn print_table<T: Tabled>(rows: &Vec<T>) {
    let table = Table::new(rows).with(Style::psql()).to_string();
    println!("{table}");
}

/// Fetches output path for recipe
pub fn get_recipe_out_path(recipe_dir: &str, name: &str) -> PathBuf {
    assert!(!recipe_dir.trim().is_empty());
    let mut out_path = Path::new(&recipe_dir).join(name.replace(" ", "-"));
    out_path.set_extension("yaml");
    out_path
}

/// Fetches all recipes
pub fn get_jsons(dir: &Path) -> io::Result<Vec<PathBuf>> {
    fs::read_dir(dir)?
        .into_iter()
        .map(|x| x.map(|entry| entry.path()))
        .collect()
}

/// Generates a set of summaries for all recipes in a directory
pub fn summarize_recipes(recipe_dir: &String) -> Result<Vec<String>, RecipeParsingError> {
    get_jsons(Path::new(&recipe_dir))?
        .iter()
        .map(|x| Ok(Recipe::try_from(x)?.summary()))
        .collect()
}

/// Converts summary back to recipe name
pub fn recipe_name_from_summary(s: &str) -> Option<String> {
    s.split("--").next()?.trim().replace(" ", "-").into()
}

/// Wrapper for dialoguer::FuzzySelect
pub fn fuzzy_select<T: std::fmt::Display>(items: &[T]) -> io::Result<usize> {
    FuzzySelect::with_theme(&ColorfulTheme::default())
        .items(items)
        .default(0)
        .interact()
}

/// Wrapper for dialoguer::Input
pub fn input_msg(msg: &str) -> io::Result<String> {
    Input::<String>::new()
        .with_prompt(msg)
        .allow_empty(true)
        .interact_text()
}

/// Wrapper for dialoguer::Select
pub fn select<T: std::fmt::Display>(items: &[T]) -> io::Result<usize> {
    Select::with_theme(&ColorfulTheme::default())
        .items(items)
        .default(0)
        .interact()
}
