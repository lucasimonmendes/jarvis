use crate::ui::{print_header, print_menu};
use crate::ui::{Menu, MenuAction, MenuItem};
use std::collections::BTreeMap;

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
        println!(
            "Error downloading repository {}: {}",
            repo,
            String::from_utf8_lossy(&output.stderr)
        );
    }
}

pub fn repos() {
    let title = "Repos Downloader";
    let phrase = "Welcome to Repos Downloader, what do you want to download?";

    print_header(title, phrase);

    let mut main_menu: Menu = BTreeMap::new();
    main_menu.insert(
        "1",
        MenuItem {
            label: "Books",
            action: MenuAction::Submenu(books()),
        },
    );
    main_menu.insert(
        "2",
        MenuItem {
            label: "melhorqnada",
            action: MenuAction::Execute(melhorqnada),
        },
    );
    main_menu.insert(
        "3",
        MenuItem {
            label: "Separates",
            action: MenuAction::Submenu(separates()),
        },
    );
    main_menu.insert(
        "4",
        MenuItem {
            label: "Exit",
            action: MenuAction::Exit,
        },
    );
    // Exibição do menu principal
    print_menu(&main_menu);
}

fn books() -> Menu {
    fn download_fs() {
        println!("Você não fez nada ainda pra esse livro");

        std::process::exit(0);
    }

    fn download_cicatrizes() {
        let repo = "git@github.com:lucasimonmendes/cicatrizes-poeticas.git";

        println!("Downloading Cicatrizes Poéticas");

        download_repo(repo);
    }

    let mut submenu: Menu = BTreeMap::new();
    submenu.insert(
        "1",
        MenuItem {
            label: "FS-41",
            action: MenuAction::Execute(download_fs),
        },
    );
    submenu.insert(
        "2",
        MenuItem {
            label: "Cicatrizes Poéticas",
            action: MenuAction::Execute(download_cicatrizes),
        },
    );
    submenu.insert(
        "3",
        MenuItem {
            label: "Exit",
            action: MenuAction::Exit,
        },
    );
    submenu
}

fn melhorqnada() {
    println!("Downloading mqn-cli");

    let repo = "git@github.com:lucasimonmendes/mqn-cli.git";

    download_repo(repo);
}

fn separates() -> Menu {
    fn download_blog_osm() {
        let repo = "git@github.com:lucasimonmendes/blog.git";

        println!("Downloading Blog LSM...");

        download_repo(repo);
    }

    fn download_alfred_cli() {
        let repo = "git@github.com:lucasimonmendes/alfred.git";

        println!("Downloading Alfred CLI...");

        download_repo(repo);
    }

    fn download_emomnichannel() {
        let repo = "git@github.com:lucasimonmendes/emomnichannel.git";

        println!("Downloading EMOmnichannel...");

        download_repo(repo);
    }

    fn download_installer_bros() {
        let repo = "git@github.com:lucasimonmendes/installer-bros.git";

        println!("Downloading Installer Bros...");

        download_repo(repo);
    }

    fn download_teamtrekker() {
        let repo = "git@github.com:lucasimonmendes/teamtrekker.git";

        println!("Downloading Teamtrekker...");

        download_repo(repo);
    }

    let mut submenu: Menu = BTreeMap::new();
    submenu.insert(
        "1",
        MenuItem {
            label: "Blog OSM",
            action: MenuAction::Execute(download_blog_osm),
        },
    );
    submenu.insert(
        "2",
        MenuItem {
            label: "EMOmnichannel",
            action: MenuAction::Execute(download_emomnichannel),
        },
    );
    submenu.insert(
        "3",
        MenuItem {
            label: "Alfred CLI",
            action: MenuAction::Execute(download_alfred_cli),
        },
    );
    submenu.insert(
        "4",
        MenuItem {
            label: "Installer Bros",
            action: MenuAction::Execute(download_installer_bros),
        },
    );
    submenu.insert(
        "5",
        MenuItem {
            label: "Teamtrekker",
            action: MenuAction::Execute(download_teamtrekker),
        },
    );
    submenu.insert(
        "6",
        MenuItem {
            label: "Exit",
            action: MenuAction::Exit,
        },
    );
    submenu
}
