use averse::{add, behold, plan, view};
use clap::{Parser, Subcommand};

/// CLI
#[derive(Parser)]
#[clap(author, version, about)]
struct Cli {
    /// Path to JSON-encoded recipe directory
    #[clap(short, long, default_value_t=String::from("./recipes"))]
    recipe_dir: String,

    /// Path to plans directory
    #[clap(short, long, default_value_t=String::from("./plans"))]
    plan_dir: String,

    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Add recipe interactively
    Add,
    /// View & filter recipes
    View,
    /// Plan meals + grocery list for the week
    Plan {
        /// Date in the form (YEAR-MONTH-DAY) e.g. 2022-05-15
        #[clap(short, long)]
        date: String,
    },
    /// Display weekly plan, select day to show recipe details
    Behold {
        /// Number of plans to display
        #[clap(short, long, default_value_t = 5)]
        n_plans: usize,
    },
}

fn main() {
    let cli = Cli::parse();
    if let Commands::Add {} = &cli.command {
        add::add_recipe(&cli.recipe_dir).expect("Failed to add recipe");
    } else if let Commands::View {} = &cli.command {
        view::display_recipes(&cli.recipe_dir).expect("Failed to view recipes");
    } else if let Commands::Plan { date } = &cli.command {
        plan::plan_week(&cli.recipe_dir, &cli.plan_dir, date).expect("Planning failed");
    } else if let Commands::Behold { n_plans } = &cli.command {
        behold::display_plan(&cli.recipe_dir, &cli.plan_dir, n_plans)
            .expect("Failed to Behold meal plan")
    } else {
        panic!("At the Disco")
    }
}
