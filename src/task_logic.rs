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

pub fn modify_task(list: &mut Vec<Task>, name: String, new_status: Option<String>, new_imp: Option<u8>) {
    let tasks_exist = list.iter().any(|t| t.name == name);
    
    if !tasks_exist {
        println!("No task with name '{}' found!", name);
        return;
    }
    
    for task in list.iter_mut().filter(|t| t.name == name) {
        if let Some(status) = &new_status {
            let valid_status = vec!["waiting", "done", "in_progress"];
            
            if !valid_status.contains(&status.as_str()) {
                println!(
                    "Invalid status '{}'. Supported statuses: {:?}", 
                    status, valid_status
                );
            } else {
                task.status = status.clone();
                println!("Status for task '{}' updated to '{}'", name, status);
            }
        }
        
        if let Some(imp) = new_imp {
            task.importance = imp;
            println!("Importance for task '{}' updated to '{}'", name, imp);
        }
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

    #[test]
    fn test_add_task() {
        let mut vec = Vec::new();
        add_task(&mut vec, String::from("Foo"), 5, None, String::from("waiting"));
        let res = Task {
            name: String::from("Foo"),
            importance: 5,
            completion_date: String::from("-"),
            status: String::from("waiting"),
        };
        assert_eq!(1, vec.len());
        assert_eq!(vec[0], res);
    }

    #[test]
    fn test_delete_task() {
        let mut vec = Vec::new();
        add_task(&mut vec, String::from("Foo"), 5, None, String::from("waiting"));
        delete_task(&mut vec, String::from("Foo"));
        assert_eq!(0, vec.len());
    }

    #[test]
    fn test_delete_task_2() {
        let mut vec = Vec::new();
        add_task(&mut vec, String::from("Foo"), 5, None, String::from("waiting"));
        add_task(&mut vec, String::from("Bar"), 5, None, String::from("waiting"));
        add_task(&mut vec, String::from("FooBar"), 5, None, String::from("waiting"));
        delete_task(&mut vec, String::from("Foo"));
        assert_eq!(2, vec.len());
    }

    #[test]
    fn test_same_name_delete() {
        let mut vec = Vec::new();
        add_task(&mut vec, String::from("Foo"), 5, None, String::from("waiting"));
        add_task(&mut vec, String::from("Foo"), 7, None, String::from("in_progress"));
        add_task(&mut vec, String::from("Foo"), 2, None, String::from("done"));
        add_task(&mut vec, String::from("Foo"), 5, Some(String::from("2025-03-01")), String::from("waiting"));
        delete_task(&mut vec, String::from("Foo"));
        assert_eq!(0, vec.len());
    }

    #[test]
    fn test_same_name_delete_2() {
        let mut vec = Vec::new();
        add_task(&mut vec, String::from("Foo"), 5, None, String::from("waiting"));
        add_task(&mut vec, String::from("FooBar"), 7, None, String::from("in_progress"));
        add_task(&mut vec, String::from("Bar"), 2, None, String::from("done"));
        add_task(&mut vec, String::from("Foo"), 5, Some(String::from("2025-03-01")), String::from("waiting"));
        delete_task(&mut vec, String::from("Foo"));
        assert_eq!(2, vec.len());
    }

    #[test]
    fn test_modify_task_status_valid() {
        let mut vec = Vec::new();
        add_task(&mut vec, String::from("Foo"), 5, None, String::from("waiting"));
        modify_task(&mut vec, String::from("Foo"), Some(String::from("done")), None);
        assert_eq!(vec[0].importance, 5);
        assert_eq!(vec[0].status, String::from("done"))
    }

    #[test]
    fn test_modify_task_importance() {
        let mut vec = Vec::new();
        add_task(&mut vec, String::from("Foo"), 5, None, String::from("waiting"));
        modify_task(&mut vec, String::from("Foo"), None, Some(8));
        assert_eq!(vec[0].status, String::from("waiting"));
        assert_eq!(vec[0].importance, 8);
    }
    
}
