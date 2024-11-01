use std::collections::BTreeMap;

const WELCOME_TEXT: &str = include_str!("../assets/welcome");

mod opener;
mod repos;
mod todolist;
mod ui;

fn main() {
    println!("{}", WELCOME_TEXT);

    let title = "---- Home -----";
    let phrase = "Hi, I'm Jarvis, your personal assistant, what do you want?";

    ui::print_header(title, phrase);

    let mut main_menu: ui::Menu = BTreeMap::new();
    main_menu.insert(
        "1",
        ui::MenuItem {
            label: "Start Workspace",
            action: ui::MenuAction::Execute(execute_opener),
        },
    );
    main_menu.insert(
        "2",
        ui::MenuItem {
            label: "Manage Tasks",
            action: ui::MenuAction::Execute(execute_todolist),
        },
    );
    main_menu.insert(
        "3",
        ui::MenuItem {
            label: "Download Projects",
            action: ui::MenuAction::Execute(execute_repos),
        },
    );
    main_menu.insert(
        "4",
        ui::MenuItem {
            label: "Exit",
            action: ui::MenuAction::Exit,
        },
    );

    ui::print_menu(&main_menu);
}

fn execute_opener() {
    opener::opener();
}

fn execute_todolist() {
    todolist::todolist();
}

fn execute_repos() {
    repos::repos();
}
