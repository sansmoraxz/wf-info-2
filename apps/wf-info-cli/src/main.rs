#[tokio::main]
async fn main() -> anyhow::Result<()> {
    wf_info_cli::run().await
}
