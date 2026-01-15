use std::env;
use std::error::Error;
use std::io;

use transaction_engine::Engine;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Use: cargo run -- <csv_file_path>");
        std::process::exit(1);
    }

    let mut engine = Engine::new();
    let filename = &args[1];
    let mut reader = csv::ReaderBuilder::new()
        .trim(csv::Trim::All)
        .from_path(filename)?;

    for result in reader.deserialize() {
        match result {
            Ok(record) => engine.process(record),
            Err(e) => {
                eprintln!("Parser problem into CSV line: {}", e);
            }
        }
    }

    let mut writer = csv::Writer::from_writer(io::stdout());

    for (&client_id, account) in engine.get_accounts() {
        writer.serialize(account.to_output(client_id))?;
    }

    writer.flush()?;

    Ok(())
}
