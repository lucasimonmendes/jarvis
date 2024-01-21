
use dialoguer::{Input, Select};
use std::process::Command;
use std::io::Write;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};


fn open_chrome(site: &str) {
    // Comando pra abrir o Chrome no site desejado
    Command::new("google-chrome")
    .arg("--new-window")
    .arg("--app")
    .arg(site)
    .output()
    .expect("Falha ao abrir o Chrome");
}

fn open_project(project: &String) {
    // Comando para abrir o projeto desejado em uma nova janela no terminal
    Command::new("gnome-terminal")
        .args(&["--window", &format!("--working-directory={}",  &project)])
        .output()
        .expect("Falha ao abrir o Projeto");
}


pub fn opener() {

        // Inicializa o Terminal com Crossterm
        let mut stdout = std::io::stdout();
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),  // Define a cor do texto
            Print("----- Inicializador -----"), // Imprime Jarvis
            Print("\n"),
            ResetColor                // Restaura a cor padrão do Terminal
        )
        .unwrap();

        stdout.flush().unwrap(); // Limpa o buffer e exibe o texto no terminal
    

    println!("Bem-vindo ao trabalho!\nQual tarefa você quer que eu execute?");

    let menu = Select::new()
        .item("Abrir Chrome + Projeto")
        .item("Abrir Projeto")
        .item("Abrir Chrome")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();


    match menu {
        
        0 => {
            // Solicita ao usuário o site para abrir o Chrome
            let site: String = Input::new()
                .with_prompt("Digite o site para abrir no Chrome")
                .interact()
                .unwrap();

            // Solicita ao usuário o projeto para abrir no LunarVim
            let project: String = Input::new()
                .with_prompt("Digite o caminho do projeto para abrir no terminal")
                .interact()
                .unwrap();

            open_chrome(&site);
            open_project(&project);
        },

        1 => {
            // Solicita ao usuário o projeto para abrir no LunarVim
            let project: String = Input::new()
                .with_prompt("Digite o caminho do projeto para abrir no terminal")
                .interact()
                .unwrap();

            open_project(&project);
        },

        2 => {
            // Solicita ao usuário o site para abrir o Chrome
            let site: String = Input::new()
                .with_prompt("Digite o site para abrir no Chrome")
                .interact()
                .unwrap();

            open_chrome(&site);
        },
        3 => {
            println!("Saindo...");

            std::process::exit(0);
        },
        _ => println!("Escolha inválida"),
    
    }
}

