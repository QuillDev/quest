use chrono::Local;
use std::cmp::Ordering;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::BufReader;
use std::path::{Path, PathBuf};

use crate::Quest;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Profile {
    pub exp: u32,
    pub completed: u32,
    pub quests: Vec<Quest>,
}

impl Profile {
    /// Create a new profile
    pub fn new() -> Profile {
        Profile {
            exp: 0,
            completed: 0,
            quests: Vec::new(),
        }
    }

    /// Load the profile from a given file
    pub fn load() -> Result<Profile, Box<dyn Error>> {
        if !Profile::config_path().exists() {
            Profile::new().write();
        }
        // read from file
        let file = File::open(Profile::config_path()).expect("File does not exist!");
        let reader = BufReader::new(file);
        let profile = serde_json::from_reader(reader)?;
        Ok(profile)
    }

    /// print a basic status for this profile
    pub fn print_status(&self) {
        println!(
            "Hey {}👋!, Here's your quests for today!",
            whoami::username()
        );

        println!("Quests Completed {} | Exp: {}", self.completed, self.exp);

        let mut open = self.get_open();
        if open.is_empty() {
            println!("All quests completed!");
            return;
        }
        open.sort_by(|a, b| {
            let daily_comp = a.daily.cmp(&b.daily);
            if daily_comp != Ordering::Equal {
                return daily_comp.reverse();
            }

            return a.exp.cmp(&b.exp).reverse();
        });

        for quest in open.iter() {
            let mut out = "".to_string();
            if quest.daily {
                out += "[Daily]";
            }
            out += format!("[{}xp]", quest.exp).as_str();
            out += " ";
            out += quest.name.as_str();

            println!("{}", out);
        }
    }

    /// Get the quest with the given name
    pub fn get_quest(&mut self, name: &str) -> Option<&mut Quest> {
        for quest in self.quests.iter_mut() {
            if quest.name == name {
                return Some(quest);
            }
        }

        return None;
    }

    /// Try to add the given quest, fails
    /// if a quest already exists with the same name
    pub fn add_quest(&mut self, quest: Quest) -> bool {
        // ensure a quest doesn't already exist.
        if self.get_quest(&quest.name).is_some() {
            return false;
        }

        self.quests.push(quest);
        self.write();

        return true;
    }

    pub fn get_open(&self) -> Vec<Quest> {
        let mut copy = self.quests.clone();
        copy.retain(|quest| !quest.completed());

        return copy;
    }

    /// Complete the quest with the given name
    pub fn complete(&mut self, name: &str) -> bool {
        let res = match self.get_quest(name) {
            None => false,
            Some(quest) => {
                // if the quest is already completed, we can't complete it
                if quest.completed() {
                    return false;
                }

                // complete the quest
                quest.date_last_completed = Local::now().date_naive().to_string();
                self.exp += quest.exp;
                self.completed += 1;
                self.write();
                true
            }
        };

        res
    }

    /// Try to remove the quest with the given name from the list
    pub fn remove_quest(&mut self, name: &str) -> bool {
        let old_len = self.quests.len();
        self.quests.retain(|q| q.name != name);

        let removed = old_len != self.quests.len();

        if removed {
            self.write();
        }

        return removed;
    }

    /// Write this to the profile config
    pub fn write(&self) {
        Profile::write_profile(&self);
    }

    /// Write the given profile
    pub fn write_profile(profile: &Profile) {
        // create parent directories
        Profile::config_path()
            .parent()
            .map(|parent| fs::create_dir_all(parent));

        let path = Profile::config_path();
        let file = File::create(path).expect("Cannot create file!");
        serde_json::to_writer(&file, profile).expect("Failed to write config");
    }

    /// Get the config path for the profile
    pub fn config_path() -> PathBuf {
        dirs::config_dir()
            .expect("No config directory")
            .as_path()
            .join(Path::new("quest"))
            .join(Path::new("profile.json"))
    }
}
