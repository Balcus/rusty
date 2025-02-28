use colored::ColoredString;
use super::*;

#[derive(Serialize, Deserialize ,Debug, PartialEq)]
pub struct Task {
    pub name: String,
    pub importance: u8,
    pub completion_date: String,
    pub status: String,
}

impl Task {
    pub fn new(name: String, importance: u8, completion_date: Option<String>, status: String) -> Task {
        let valid_status = vec!["waiting", "done", "in_progress"];

        if !valid_status.contains(&status.as_str()) {
            panic!("The provided status is not supported! Given status: {}, supported statuses: {:?}", status, valid_status);
        }

        match completion_date {
            Some(date_as_str) => {
                match NaiveDate::parse_from_str(&date_as_str, "%Y-%m-%d") {
                    Ok(_date) => Task {
                        name,
                        importance,
                        completion_date: date_as_str,
                        status
                    },
                    Err(_e) => {
                        panic!("Failed at parsing date! Please make sure the date format is: YYYY-MM-DD");
                    }
                }
            },
            None => Task {
                name,
                importance,
                completion_date: String::from("-"),
                status
            }
        }
    }
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let icon: char;
        let name_colored: ColoredString;
        
        if self.status == "waiting".to_string() {
            icon = '❌';
            name_colored = self.name.bold().red();
        }else if self.status == "done".to_string() {
            icon = '✅';
            name_colored = self.name.bold().green();
        }else {
            icon = '🔧';
            name_colored = self.name.bold().blue();
        }

        write!(
            f,
            "[ {} ] {:<30} {:<5} {:<10}",
            icon, name_colored, self.importance, self.completion_date
        )
    }
}

pub fn add_task(list: &mut Vec<Task>, name: String, imp: u8, c_date: Option<String>, status: String) {
    let entry = Task::new(name, imp, c_date, status);
    list.push(entry);
}

// The delete method will delete ALL tasks with the given name
pub fn delete_task(list: &mut Vec<Task>, name: String) {
    list.retain(|t| t.name != name);
    println!("Task {} deleted!", name);
}

pub fn list_tasks(list: &Vec<Task>) {
    for t in list {
        println!("{}", t);
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_constructor_correct_params() {
        let name = String::from("Foo");
        let imp = 5u8;
        let cd = None;
        let status = String::from("waiting");
        let task = Task::new(name.clone(), imp, cd, status.clone());

        let res = Task {
            name,
            importance: imp,
            completion_date: String::from("-"),
            status
        };

        assert_eq!(task, res);
    }

    #[test]
    #[should_panic(expected = "status is not supported")]
    fn test_constructor_invalid_status() {
        let name = String::from("Foo");
        let imp = 5u8;
        let cd = None;
        let status = String::from("bar");
        let _task = Task::new(name, imp, cd, status);
    }

    #[test]
    #[should_panic(expected = "Failed at parsing date")]
    fn test_constructor_invalid_date() {
        let name = String::from("Foo");
        let imp = 5u8;
        let cd = Some(String::from("22-11-2021"));
        let status = String::from("waiting");
        let _task = Task::new(name, imp, cd, status);
    }
}
