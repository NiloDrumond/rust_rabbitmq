use std::{env, process};

use amiquip::{Connection, ConsumerOptions, QueueDeclareOptions, Result};
use chrono::offset;

fn parse_data(raw: &[u8]) -> [u8; 8] {
    raw.try_into().expect("parse error")
}

fn main() -> Result<()> {
    let mut args = env::args();
    args.next();

    let prefetch_count: u16 = match args.next() {
        Some(arg) => match arg.parse() {
            Ok(num) => num,
            Err(e) => {
                eprintln!("PC inválido: {}", e);
                process::exit(1);
            }
        },
        None => {
            eprintln!("PC não recebido");
            process::exit(1);
        }
    };

    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    channel.qos(0, prefetch_count, false)?;
    let queue = channel.queue_declare("test", QueueDeclareOptions::default())?;
    let consumer = queue.consume(ConsumerOptions::default())?;

    let mut durations: Vec<i64> = Vec::new();

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            amiquip::ConsumerMessage::Delivery(delivery) => {
                let data = parse_data(&delivery.body[..8]);
                consumer.ack(delivery)?;
                let send_time = i64::from_be_bytes(data);
                let now = offset::Utc::now().timestamp_millis();
                let time_diff = now - send_time;

                durations.push(time_diff);

                if i + 1 == 10_000 {
                    let avg: i64 = durations.iter().sum::<i64>() / durations.len() as i64;
                    println!("Average Time Diff: {avg:?}");
                    break;
                };
            }
            other => {
                println!("Consumer ended: {:?}", other);
                break;
            }
        }
    }

    connection.close()
}
