use crate::ui::print_header;
use dialoguer::{Input, Select};
use std::process::Command;

fn open_chrome(site: &str) {
    Command::new("google-chrome")
        .arg("--new-window")
        .arg("--app")
        .arg(site)
        .output()
        .expect("Failed to open Chrome");
}

fn open_project(project: &String) {
    Command::new("gnome-terminal")
        .args(&["--window", &format!("--working-directory={}", &project)])
        .output()
        .expect("Failed to open the project");
}

pub fn opener() {
    let title = "---- Workspace Starter ----";
    let phrase = "Welcome to your workspace!\nWhat task do you want me to perform?";

    print_header(&title, &phrase);

    let menu = Select::new()
        .item("Open Chrome + Project")
        .item("Open Project")
        .item("Open Chrome")
        .item("Exit")
        .default(0)
        .interact()
        .unwrap();

    match menu {
        0 => {
            let site: String = Input::new()
                .with_prompt("Enter the website to open in Chrome")
                .interact()
                .unwrap();

            let project: String = Input::new()
                .with_prompt("Enter the project path to open in the terminal")
                .interact()
                .unwrap();

            open_chrome(&site);
            open_project(&project);
        }

        1 => {
            let project: String = Input::new()
                .with_prompt("Enter the project path to open in the terminal")
                .interact()
                .unwrap();

            open_project(&project);
        }

        2 => {
            let site: String = Input::new()
                .with_prompt("Enter the website to open in Chrome")
                .interact()
                .unwrap();

            open_chrome(&site);
        }
        3 => {
            println!("Leaving...");

            std::process::exit(0);
        }
        _ => println!("Invalid choice"),
    }
}
