use triagebot::agenda;

#[tokio::main(flavor = "current_thread")]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    tracing_subscriber::fmt::init();

    let args: Vec<String> = std::env::args().collect();
    if args.len() == 2 {
        match &args[1][..] {
            "agenda" | "triage" => {
                let agenda = agenda::lang();
                print!("{}", agenda.call().await?);
                return Ok(());
            }
            "design" => {
                let agenda = agenda::lang_design();
                print!("{}", agenda.call().await?);
                return Ok(());
            }
            "planning" => {
                let agenda = agenda::lang_planning();
                print!("{}", agenda.call().await?);
                return Ok(());
            }
            _ => {}
        }
    }

    eprintln!("Usage: lang (triage|design|planning)");

    Ok(())
}
