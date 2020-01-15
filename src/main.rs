mod backend;
mod model;
mod ui;

use backend::git::GitProvider;
use ui::cli::Cli;
use ui::renderer::Renderer;

fn main() {
    let provider = GitProvider::new();
    let cli = Cli::new();

    cli.render(provider);
}
