use std::collections::BTreeMap;

use std::io::Write;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

pub type MenuFunction = fn();
pub type Menu = BTreeMap<&'static str, MenuItem>;

pub struct MenuItem {
    pub label: &'static str,
    pub action: MenuAction,
}

pub enum MenuAction {
    Execute(MenuFunction),
    Submenu(Menu),
    Exit,
}

pub fn print_header(title: &str, phrase: &str) {
    // Initialize the Terminal with Crossterm
    let mut stdout = std::io::stdout();
    execute!(
        stdout,
        SetForegroundColor(Color::Yellow),
        Print(title),
        Print("\n"),
        ResetColor
    )
    .unwrap();

    stdout.flush().unwrap(); // Clean the buffer and displays the text in the terminal

    println!("{}", phrase);
}

pub fn print_menu(menu: &Menu) {
    for (key, value) in menu.iter() {
        println!("{}: {}", key, value.label);
    }

    let mut input = String::new();
    std::io::stdin()
        .read_line(&mut input)
        .expect("Failed to read the input");

    if let Some(item) = menu.get(input.trim()) {
        match &item.action {
            MenuAction::Execute(func) => func(),
            MenuAction::Submenu(submenu) => print_menu(&submenu),
            MenuAction::Exit => println!("Leaving"),
        }
    } else {
        println!("Invalid Choice!");
    }
}
