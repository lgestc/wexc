use super::renderer::Renderer;
use crate::backend::provider::Provider;

pub struct Cli {}

impl Cli {
    pub fn new() -> Cli {
        Cli {}
    }
}

impl Renderer for Cli {
    fn render(&self, provider: impl Provider) {
        print!("{:?}", provider.provide_entries());
    }
}
