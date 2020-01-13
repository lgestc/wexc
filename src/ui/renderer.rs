use crate::backend::provider::Provider;

pub trait Renderer {
    fn render(&self, provider: impl Provider);
}
