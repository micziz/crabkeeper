// requirements
use std::env;
use std::fs;
use std::path::Path;
use std::process::Command;

fn clear(){    
    // Check if the platform is windows or not
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .output()
            .expect("Failed to clear the screen");
    } else {
        Command::new("clear")
            .output()
            .expect("Failed to clear the screen");
    }
}

fn startup(){
    // Check if passwords.txt exists
    let path = Path::new("passwords.txt");
    if path.exists() {
        println!("passwords.txt exists");
    } else {
        println!("passwords.txt does not exist");
        // Create passwords.txt
        let _file = fs::File::create("passwords.txt").expect("Unable to create passwords.txt");
        println!("Created passwords.txt");
    }       
    clear();
}

// Create a new password
fn create(){
    // Select the mode of creation
    println!("Select the mode of creatoin of the password");
    println!("1. Input the password");
    // Get the input from the user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    // Remove the newline character
    input.pop();
    // Check if the input is 1
    if input == "1" {
        // Get the password
        println!("Enter the password");
        let mut password = String::new();
        std::io::stdin().read_line(&mut password).expect("Failed to read line");
        println!("Password created");
        fs::write("passwords.txt", password).expect("Unable to write to passwords.txt");
        fs::write("passwords.txt", "\n").expect("Unable to write to passwords.txt");
    } else {
        println!("Invalid input");
    }
}

// List all the passwords function
fn list(){
    // Read the passwords.txt file
    let contents = fs::read_to_string("passwords.txt").expect("Unable to read passwords.txt");
    println!("{}", contents);
}

fn help(){
    println!("Usage: crabkeeper [command]");
    println!("Commands:");
    println!("  create - Create a new password");
    println!("  list - List all the passwords");
    println!("  delateall - Delete all the passwords");
    println!("  -H --help - Show this help");
    println!("  -V --version - Show the version");
}

fn deleate_all(){
    println!("ARE YOU SURE THAT YOU WANT THAT YOU WANT TO DELEATE ALL OF YOUR PASSWORDS!!!? (TYPE YES OR NO)");
    let mut confirm = String::new();
    std::io::stdin().read_line(&mut confirm).expect("Failed to read line");
    if confirm == "YES"{
        println!("ARE YOU SURE THAT YOU WANT THAT YOU WANT TO DELEATE ALL OF YOUR PASSWORDS!!!? (TYPE YES OR NO)");
        let mut confirm_again = String::new();
        std::io::stdin().read_line(&mut confirm_again).expect("Failed to read line");
        if confirm_again == "YES"{
            // Delete the passwords.txt file
            fs::remove_file("passwords.txt").expect("Unable to delete passwords.txt");
            println!("Deleted passwords.txt");
        } else {
            println!("Deletion aborted");
        }
    } else {
        println!("Deletion aborted");
    }
}

fn main() {
    // Get version from Cargo.toml
    let version = env!("CARGO_PKG_VERSION");
    startup();
    let args: Vec<String> = env::args().collect();
    let command = &args[1];
    if command == "create" {
        create();
    } else if command == "list" {
        list();
    } else if command == "deleteAll" {
        deleate_all();
    } else if command == "help" {
        help();
    } else if command == "-H" || command == "--help" {
        help();
    } else if command == "-V" || command == "--version" {
        println!("Crab version {}", version);
    } else {
        println!("Invalid command");
    }
}