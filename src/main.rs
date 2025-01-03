use std::collections::BTreeMap;

use opener::opener;
use repos::repos;
use todolist::todolist;

mod opener;
mod repos;
mod todolist;
mod ui;

fn main() {
    let title = "Jarvis";
    let phrase = "Hi, I'm Jarvis, your personal assistant, what do you want?";

    ui::print_header(title, phrase);

    let mut main_menu: ui::Menu = BTreeMap::new();
    main_menu.insert(
        "1",
        ui::MenuItem {
            label: "Start Workspace",
            action: ui::MenuAction::Execute(opener),
        },
    );
    main_menu.insert(
        "2",
        ui::MenuItem {
            label: "Manage Tasks",
            action: ui::MenuAction::Execute(todolist),
        },
    );
    main_menu.insert(
        "3",
        ui::MenuItem {
            label: "Download Projects",
            action: ui::MenuAction::Execute(repos),
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
