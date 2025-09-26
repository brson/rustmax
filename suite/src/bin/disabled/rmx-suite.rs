// CLI binary entry point.

#[rmx::tokio::main]
async fn main() -> rustmax_suite::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    rustmax_suite::cli::run(args).await
}