use dialoguer::Select;
use std::io::Write;
use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};


pub fn todolist() {
   
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
    

    println!("Bem-vindo ao Gerenciador de tarefas!\nO que você deseja?");

    let menu = Select::new()
        .item("Abrir Listas")
        .item("Adicionar Lista")
        .item("Adicionar Tarefa")
        .item("Remover Lista")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();


    match menu {
        
        0 => {
            // show_lists();
            println!("Abrindo listas...");

            std::process::exit(0);
        },

        1 => {
            // add_list();
            println!("Adicionando lista...");

            std::process::exit(0);
        },

        2 => {
            // add_task();
            println!("Adicionando tarefa...");

            std::process::exit(0);
        },
        3 => {
            // remove_list();
            println!("Removendo lista...");

            std::process::exit(0);
        },
        4 => {
            println!("Saindo...");

            std::process::exit(0);
        },
        _ => println!("Escolha inválida"),
    
    }

}
