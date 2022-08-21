use chrono::{Local, NaiveDate};

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Quest {
    pub name: String,
    pub exp: u32,
    pub date_last_completed: String,
    pub daily: bool,
}

impl Quest {
    pub fn new(name: &str, exp: u32, daily: bool) -> Quest {
        let name = String::from(name);
        let date_last_completed = String::new();
        Quest {
            name,
            exp,
            date_last_completed,
            daily,
        }
    }

    pub fn completed(&self) -> bool {
        let today = Local::now().date_naive();
        let completed_date = self.date_last_completed();

        // if the quest is a daily, it is not completed
        // if completed date is none, or the date is today.
        if self.daily && (completed_date.is_none() || completed_date.unwrap() != today) {
            return false;
        } else if !self.daily && completed_date.is_none() {
            return false;
        }

        return true;
    }

    pub fn date_last_completed(&self) -> Option<NaiveDate> {
        match NaiveDate::parse_from_str(self.date_last_completed.as_str(), "%Y-%m-%d") {
            Ok(date) => Some(date),
            Err(_) => None,
        }
    }
}
