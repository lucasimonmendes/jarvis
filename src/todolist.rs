use std::io::{self, Write, BufReader, BufRead};
use std::fs::{File, OpenOptions};
use std::fmt;

use dialoguer::Select;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};

use serde_json;
use serde::{Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Task {
    pub id: usize,
    pub description: String,
    pub completed: bool,
}

impl fmt::Display for Task {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}\t[{}] {}", self.id, if self.completed { "X" } else { " " }, self.description)
    }
}

impl Task {
    pub fn new(id: usize, description: String) -> Task {
        Task { 
            id,
            description, 
            completed: false, 
        }
    }
}

fn read_tasks_from_file() -> Vec<Task> {
    let file = match File::open("tasks.txt") {
        Ok(file) => file,
        Err(_) => return Vec::new(),
    };

    let reader = BufReader::new(file);
    let mut tasks = Vec::new();

    for line in reader.lines() {
        if let Ok(task) = serde_json::from_str::<Task>(&line.unwrap()) {
            tasks.push(task);
        }
    }


    tasks
}

fn write_tasks_to_file(tasks: &Vec<Task>) {
    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.txt")
        .expect("Failed to open file");

    for task in tasks {
        let serialized_task = serde_json::to_string(&task).expect("Failed to serialize a task");
        writeln!(file, "{}", serialized_task).expect("Failed to write file");
    }
}


fn add_task(tasks: &mut Vec<Task>) {
    println!("Digite a tarefa: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut description = String::new();
    io::stdin().read_line(&mut description).expect("Falha ao ler a linha");

    let id = tasks.len() + 1;
    let new_task = Task::new(id, description.trim().to_string());
    tasks.push(new_task);

    write_tasks_to_file(tasks);
}

fn list_tasks(tasks: &Vec<Task>) {
    println!("Lista de tarefas:");
    for task in tasks {
        let status = if task.completed { "Completada" } else { "Pendente" };
        println!("{}. [{}] {}", task.id, status, task.description);
    }
}

fn update_task_in_file(task: &Task) {
    let file = OpenOptions::new()
        .read(true)
        .write(true)
        .open("tasks.txt")
        .expect("Failed to open file");

    let reader = BufReader::new(&file);
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .create(true)
        .open("tasks.txt")
        .expect("Failed to open file");

    for line in lines {
        if let Ok(mut existing_task) = serde_json::from_str::<Task>(&line) {
            if existing_task.id == task.id {
                existing_task.completed = task.completed;
                let serialized_task = serde_json::to_string(&existing_task).expect("Failed to serialize task");
                writeln!(file, "{}", serialized_task).expect("Failed to write to file");
            } else {
                writeln!(file, "{}", line).expect("Failed to write to file");
            }
        }
    }
}


fn complete_task(tasks: &mut Vec<Task>) {
    list_tasks(&tasks);

    println!("Digite o ID da tarefa para marcar como completa: ");
    io::stdout().flush().expect("Failed to flush stdout");

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    if let Ok(id) = input.trim().parse::<usize>() {
        if let Some(task) = tasks.iter_mut().find(|t| t.id == id) {
            task.completed = true;
            update_task_in_file(&task);
            println!("Tarefa marcada como concluída");
        } else {
            println!("Tarefa não encontrada.");
        }
    } else {
        println!("Entrada inválida. Por favor, digite um ID válido.");
    }
}




pub fn todolist() {
   
    let mut tasks = read_tasks_from_file();

            // Inicializa o Terminal com Crossterm
        let mut stdout = std::io::stdout();
        execute!(
            stdout,
            SetForegroundColor(Color::Yellow),  // Define a cor do texto
            Print("----- Gerenciador de Tarefas -----"), // Imprime Jarvis
            Print("\n"),
            ResetColor                // Restaura a cor padrão do Terminal
        )
        .unwrap();

        stdout.flush().unwrap(); // Limpa o buffer e exibe o texto no terminal

    println!("Bem-vindo ao Gerenciador de tarefas!\nO que você deseja?");

    let menu = Select::new()
        .item("Abrir Lista")
        .item("Adicionar Tarefa")
        .item("Concluir Tarefa")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();


    match menu {
        
        0 => {
            println!("Abrindo lista...");
            list_tasks(&tasks);
        },

        1 => {
            add_task(&mut tasks);
        },

        2 => {
            complete_task(&mut tasks);
       },
        3 => {
            println!("Saindo...");

            std::process::exit(0);
        },
        _ => println!("Escolha inválida"),
    
    }

}
