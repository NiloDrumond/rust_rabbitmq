use std::{env, process};

use chrono::offset;

use amiquip::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();

    let size: usize = match args.next() {
        Some(arg) => match arg.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("Bytes inválido: {}", e);
                process::exit(1);
            }
        },
        None => {
            eprintln!("Bytes não recebido");
            process::exit(1);
        }
    };

    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    for _ in 0..10_000 {
        let now = offset::Utc::now();
        let now = now.timestamp_millis();
        let mut now = Vec::from(now.to_be_bytes());
        now.resize(size, 0);

        exchange.publish(Publish::new(&now, "test"))?;
    }

    connection.close()
}
