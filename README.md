# Jarvis
```
   _                  _     
  (_) __ _ _ ____   _(_)___ 
  | |/ _` | '__\ \ / / / __|
  | | (_| | |   \ V /| \__ \
 _/ |\__,_|_|    \_/ |_|___/
|__/
```
## Descrição

Jarvis é uma CLI (Interface de Linha de Comando) implementada em Rust, inspirada no assistente virtual fictício do filme Homem de Ferro. Esta aplicação oferece funcionalidades para interação por linha de comando, incluindo recursos de automação, utilidades e informações relevantes.

## Funcionalidades Principais
- **Automatização de Tarefas**: Execute tarefas repetitivas ou específicas do sistema de forma automatizada (abrir chrome e projetos em terminais novos).
- **Informações Relevantes(A IMPLEMENTAR)**: Oferece informações úteis sobre o sistema, dados meteorológicos, notícias, etc.
- **Todolist**: Lista e adiciona novas tarefas.
- **Download de projetos**: Download de projetos e inicializadores de ambientes de projetos.

## Instalação

Para instalar Jarvis, é necessário ter o Rust instalado. Clone o repositório e compile o código usando Cargo, o gerenciador de pacotes do Rust.

```bash
git clone https://github.com/lucasimonmendes/jarvis.git
cd jarvis
cargo build --release
```
## Uso

Após a instalação, execute a CLI utilizando:

```bash
./target/release/jarvis
```

Para visualizar os comandos disponíveis e a ajuda(ainda a IMPLEMENTAR):

```bash
./target/release/jarvis --help
```



## Autor
Lucas Simon


## Contribuição

Contribuições são bem-vindas! Se deseja colaborar com o Jarvis, por favor, abra uma issue para discutir as mudanças que gostaria de fazer.

## Licença

Este projeto está licenciado sob a MIT License - veja o arquivo [LICENSE](LICENSE) para mais detalhes.
