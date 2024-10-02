use phone_numbers::build_database;
use phone_numbers::is_consistent;

fn main() {
    let mut databases: Vec<Vec<String>> = vec![];

    databases.push(build_database("o) Bob 91 12 54 26\nEmergency 911"));
    databases.push(build_database("o) Bob 91 12 54 26\nnon-Emergency 912"));
    databases.push(build_database("o) Bob 91 12 54 26\nnon-Emergency 912\n8016280543"));

    for database in databases {
        println!(
            "The phone number array {:?} is{} consistent.",
            database,
            if is_consistent(&database) { "" } else { " not" }
        );
    }
}
