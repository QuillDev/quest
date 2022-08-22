use clap::{Parser, Subcommand};
use crate::data::quest::Quest;
use crate::data::profile::Profile;

mod data;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(subcommand)]
    command: Option<Command>,
}

#[derive(Subcommand)]
enum Command {
    /// Shows Quests that are currently available
    Show,
    /// Completed the quest with the given name
    Complete {
        name: String,
    },
    /// Add a daily quest with the given name and experience value
    Add {
        name: String,
        exp: u32,
        daily: Option<bool>,
    },
    /// Remove the daily quest with the given name
    Remove {
        name: String,
    },
}

fn main() {
    // load the user profile
    let mut profile = Profile::load().expect("Failed to load profile.");
    let cli = Cli::parse();

    // You can check for the existence of subcommands, and if found use their
    // matches just as you would the top level cmd
    match &cli.command {
        Some(Command::Show) => {
            profile.print_status();
        }
        Some(Command::Add { name, exp, daily }) => {
            let quest = Quest::new(name, exp.clone(), daily.is_some());
            profile.add_quest(quest);
        }
        Some(Command::Remove {name}) => {
            profile.remove_quest(name);
        }
        Some(Command::Complete { name }) => {
            profile.complete(name);
        }
        None => {
            profile.print_status();
        }
    }
}
