use crate::ui::{print_header, print_menu};
use crate::ui::{Menu, MenuAction, MenuItem};
use std::collections::BTreeMap;

use dialoguer::Input;
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
        .args(["--window", &format!("--working-directory={}", &project)])
        .output()
        .expect("Failed to open the project");
}

pub fn opener() {
    let title = "Workspace Starter";
    let phrase = "Welcome to your workspace!\nWhat task do you want me to perform?";

    print_header(title, phrase);

    let mut main_menu: Menu = BTreeMap::new();
    main_menu.insert(
        "1",
        MenuItem {
            label: "Open Chrome + Project",
            action: MenuAction::Execute(execute_project_and_chrome),
        },
    );
    main_menu.insert(
        "2",
        MenuItem {
            label: "Open Project",
            action: MenuAction::Execute(execute_project),
        },
    );
    main_menu.insert(
        "3",
        MenuItem {
            label: "Open Chrome",
            action: MenuAction::Execute(execute_chrome),
        },
    );
    main_menu.insert(
        "4",
        MenuItem {
            label: "Exit",
            action: MenuAction::Exit,
        },
    );
    // Exibição do menu principal
    print_menu(&main_menu);
}

fn execute_project() {
    let project: String = Input::new()
        .with_prompt("Enter the project path to open in the terminal")
        .interact()
        .unwrap();

    open_project(&project);
}
fn execute_chrome() {
    let site: String = Input::new()
        .with_prompt("Enter the website to open in Chrome")
        .interact()
        .unwrap();

    open_chrome(&site);
}
fn execute_project_and_chrome() {
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
