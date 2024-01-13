use dialoguer::Select;
use std::io::Write;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};


mod opener;
mod todolist;


fn main() {

    // Inicializa o Terminal com Crossterm
        let mut stdout = std::io::stdout();
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),  // Define a cor do texto
            Print("----- Jarvis -----"), // Imprime Jarvis
            Print("\n"),
            ResetColor                // Restaura a cor padrão do Terminal
        )
        .unwrap();

        stdout.flush().unwrap(); // Limpa o buffer e exibe o texto no terminal
    

    println!("Olá, sou Jarvis seu assistente pessoal, o que deseja?");

    let menu = Select::new()
        .item("Iniciar espaço de trabalho")
        .item("Gerenciar Tarefas")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();


    match menu {
        
        0 => {
            opener::opener();
        },

        1 => {
            // println!("Abrindo gerenciador de projetos");

            todolist::todolist();

             

        },

        2 => {
            println!("Saindo...");
            std::process::exit(0);
        },   
        _ => println!("Escolha inválida"),
    
    }
}



