use std::io::Write;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};



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
