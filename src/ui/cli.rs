use super::renderer::Renderer;

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        Cli {}
    }
}

impl Renderer for Cli {}
