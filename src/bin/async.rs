use triagebot::agenda;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match &args[1][..] {
            "reading" => {
                let agenda = agenda::async_reading();
                print!("{}", agenda.call().await?);
                return Ok(());
            }
            "triage" => {
                let agenda = agenda::async_triage();
                print!("{}", agenda.call().await?);
                return Ok(());
            }
            _ => {}
        }
    }

    eprintln!("Usage: async triage");

    Ok(())
}
