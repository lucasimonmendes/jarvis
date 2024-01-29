use dialoguer::Select;
use crate::ui::print_header;

use std::process::Command;

fn download_repo(repo: &str) {
    let output = Command::new("git")
        .arg("clone")
        .arg(repo)
        .output()
        .expect("Failed to run 'git clone' command");

    if output.status.success() {
        println!("Repository downloaded successfully: {}", repo);
    } else {
        println!("Error downloading repository {}: {}", repo, String::from_utf8_lossy(&output.stderr));
    }
}



pub fn repos() {

    let title = "----- Repos Downloader -----";
    let phrase = "Welcome to Repos Downloader, what do you want to download?";

    print_header(&title, &phrase);


    let menu = Select::new()
        .item("Livros")
        .item("melhorqnada")
        .item("Separates")
        .item("Exit")
        .default(0)
        .interact()
        .unwrap();

    match menu {
        0 => {
            let books_menu = Select::new()
                .item("FS-41")
                .item("Cicatrizes Poéticas")
                .item("Exit")
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

                    println!("Downloading Cicatrizes Poéticas");

                    download_repo(repo);

                },
                2 => {
                    println!("Leaving...");

                    std::process::exit(0);

                },
                _ => {
                    println!("Invalid Choice");
                }
            }
        },
        1 => {
            println!("Downloading mqn-cli");

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

                    println!("Downloading Blog LSM...");

                    download_repo(repo);

                },
                1 => {
                    let repo = "git@github.com:lucasimonmendes/alfred.git";

                    println!("Downloading Alfred CLI...");

                    download_repo(repo);


                },
                2 => {
                    let repo = "git@github.com:lucasimonmendes/emomnichannel.git";

                    println!("Downloading EMOmnichannel...");

                    download_repo(repo);
                },
                3 => {
                    let repo = "git@github.com:lucasimonmendes/installer-bros.git";

                    println!("Downloading Installer Bros...");

                    download_repo(repo);
                },
                4 => {
                    let repo = "git@github.com:lucasimonmendes/teamtrekker.git";

                    println!("Downloading Teamtrekker...");

                    download_repo(repo);
                },
                5 => {
                    println!("Leaving...");

                    std::process::exit(0);
                },
                _ => {
                    println!("Invalid Choice");
                },
            }
        },
        3 => {
            println!("Leaving...");

            std::process::exit(0);

        },
        _ => {
            println!("Invalid Choice");
        },
    }
}
