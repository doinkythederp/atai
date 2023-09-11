use async_trait::async_trait;

pub mod chatgpt;

#[async_trait]
pub trait Adapter {
    async fn generate(
        &mut self,
        prompt: &str,
        progress_hook: &mut (impl FnMut(String) + Send),
    ) -> String;
}
