use wexc::{
    backend::git::GitProvider,
    ui::{cli::Cli, renderer::Renderer},
};

fn main() {
    let provider = GitProvider::new();
    let cli = Cli::new();

    cli.render(provider);
}
