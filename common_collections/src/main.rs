use std::collections::HashMap;

fn main() {
    let mut vector: Vec<i32> = Vec::new();
    vector.push(1);
    vector.push(2);

    let val_at_idx: Option<&i32> = vector.get(1);

    dbg!("{}", &val_at_idx);

    match val_at_idx {
        Some(x) => println!("{} was returned", x),
        None => {
            println!("Your value does not exist");
        }
    }

    for i in vector {
        println!("{i}");
    }


    #[derive(Debug)]
    enum Person {
        Female {
            name: String,
            age: i32,
        },
        Male {
            name: String,
            age: i32,
        },
    }

    let person_vec: Vec<Person> = Vec::from([Person::Female { name: String::from("Helen"), age: 19 }, Person::Male { name: String::from("John"), age: 32 }]);

    dbg!("{:#?}", &person_vec.get(1));

    let mut team_scores_map: HashMap<&str, i8> = HashMap::new();

    team_scores_map.insert("Blue", 1);
    team_scores_map.insert("Red", 0);

    let blue_score = team_scores_map.get("Blue").unwrap_or(&0);

    println!("Blue Score: {}", blue_score);

    for (key, value) in &team_scores_map {
        println!("{key}: {value}");
    }

    let mut median_vector: Vec<i32> = Vec::new();

    median_vector.push(1);
    median_vector.push(16);
    median_vector.push(7);
    median_vector.push(23);
    median_vector.push(20);

    median_vector.sort();

    println!("{:#?}", &median_vector.get(2).copied().unwrap());

    #[derive(Debug)]
    struct Employee {
        first_name: String,
        last_name: String,
    }

    let employee_sally: Employee = Employee { first_name: String::from("Sally"), last_name: String::from("Doe") };
    let employee_amir: Employee = Employee { first_name: String::from("Amir"), last_name: String::from("Doe") };

    let mut employee_map: HashMap<&str, Vec<Employee>> = HashMap::new();

    employee_map.insert("Engineering", Vec::from([employee_sally]));
    employee_map.insert("Sales", Vec::from([employee_amir]));

    let employee_kanush: Employee = Employee { first_name: String::from("Kanush"), last_name: String::from("Doe") };

    employee_map.entry("Engineering").and_modify(|f| f.push(employee_kanush));

    dbg!("{:#?}", &employee_map);
}
