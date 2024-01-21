use dialoguer::Select;
use std::io::Write;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};


const WELCOME_TEXT: &str = include_str!("../assets/welcome");

mod opener;
mod todolist;
mod repos;

fn main() {

    println!("{}", WELCOME_TEXT);

    // Inicializa o Terminal com Crossterm
        let mut stdout = std::io::stdout();
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),  // Define a cor do texto
            Print("----- Home -----"),       // Imprime Jarvis
            Print("\n"),
            ResetColor                // Restaura a cor padrão do Terminal
        )
        .unwrap();

        stdout.flush().unwrap(); // Limpa o buffer e exibe o texto no terminal
    

    println!("Olá, sou Jarvis seu assistente pessoal, o que deseja?");

    let menu = Select::new()
        .item("Iniciar espaço de trabalho")
        .item("Gerenciar Tarefas")
        .item("Download Projetos")
        .item("Sair")
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
            println!("Saindo...");
            std::process::exit(0);
        },   
        _ => println!("Escolha inválida"),
    }
}



