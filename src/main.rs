use dialoguer::Select;


const WELCOME_TEXT: &str = include_str!("../assets/welcome");

mod opener;
mod todolist;
mod repos;
mod ui;

fn main() {

    println!("{}", WELCOME_TEXT);

    let title = "---- Home -----";
    let phrase = "Hi, I'm Jarvis, your personal assistant, what do you want?";

    ui::print_header(&title, &phrase);


    let menu = Select::new()
        .item("Start Workspace")
        .item("Manage Tasks")
        .item("Download Projects")
        .item("Exit")
        .default(0)
        .interact()
        .unwrap();


    match menu {
        0 => {
            opener::opener();
        },
        1 => {
            todolist::todolist();
        },
        2 => {
            repos::repos();
        },
        3 => {
            println!("Leaving...");
            std::process::exit(0);
        },   
        _ => println!("Invalid Choice"),
    }
}



