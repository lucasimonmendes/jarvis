use dialoguer::Select;

use std::io::Write;
use std::process::Command;

use crossterm::{
    execute,
    style::{Color, Print, ResetColor, SetForegroundColor},
};


fn download_repo(repo: &str) {
    let output = Command::new("git")
        .arg("clone")
        .arg(repo)
        .output()
        .expect("Falha ao executar o comando git clone");

    if output.status.success() {
        println!("Repositório baixado com sucesso: {}", repo);
    } else {
        println!("Erro ao baixar o repositório {}: {}", repo, String::from_utf8_lossy(&output.stderr));
    }
}



pub fn repos() {

    // Inicializa o Terminal com Crossterm
    let mut stdout = std::io::stdout();
    execute!(
        stdout, 
        SetForegroundColor(Color::Blue), 
        Print("----- Jarvis -----"), 
        Print("\n"), 
        ResetColor
    ).unwrap();

    stdout.flush().unwrap();

    println!("Bem vindo ao downloader de Projetos, o que deseja baixar?");


    let menu = Select::new()
        .item("Livros")
        .item("melhorqnada")
        .item("Avulsos")
        .item("Sair")
        .default(0)
        .interact()
        .unwrap();

    match menu {
        0 => {
            let books_menu = Select::new()
                .item("FS-41")
                .item("Cicatrizes Poéticas")
                .item("Sair")
                .default(0)
                .interact()
                .unwrap();

            match books_menu {
                0 => {

                    println!("Você não fez nada ainda pra esse livro");

                    std::process::exit(0);

                },
                1 => {
                    let repo = "git@github.com:lucasimonmendes/cicatrizes-poeticas.git";

                    println!("Baixando Cicatrizes Poéticas");

                    download_repo(repo);

                },
                2 => {
                    println!("Saindo...");

                    std::process::exit(0);

                },
                _ => {
                    println!("Opção inválida");
                }
            }
        },
        1 => {
            println!("Baixando mqn-cli");

            let repo = "git@github.com:lucasimonmendes/mqn-cli.git";

            download_repo(repo);

        },
        2 => {
            let randoms_menu = Select::new()
                .item("Blog LSM")
                .item("Alfred CLI")
                .item("EMOmnichannel")
                .item("Installer Bros")
                .item("Teamtrekker")
                .default(0)
                .interact()
                .unwrap();


            match randoms_menu {
                0 => {
                    let repo = "git@github.com:lucasimonmendes/blog.git";

                    println!("Baixando Blog LSM...");

                    download_repo(repo);

                },
                1 => {
                    let repo = "git@github.com:lucasimonmendes/alfred.git";

                    println!("Baixando Alfred CLI...");

                    download_repo(repo);


                },
                2 => {
                    let repo = "git@github.com:lucasimonmendes/emomnichannel.git";

                    println!("Baixando EMOmnichannel...");

                    download_repo(repo);
                },
                3 => {
                    let repo = "git@github.com:lucasimonmendes/installer-bros.git";

                    println!("Baixando Installer Bros...");

                    download_repo(repo);
                },
                4 => {
                    let repo = "git@github.com:lucasimonmendes/teamtrekker.git";

                    println!("Baixando Teamtrekker...");

                    download_repo(repo);
                },
                5 => {
                    println!("Saindo...");

                    std::process::exit(0);
                },
                _ => {
                    println!("Opção inválida");
                },
            }
        },
        3 => {
            println!("Saindo...");

            std::process::exit(0);

        },
        _ => {
            println!("Opção Inválida");
        },
    }
}
