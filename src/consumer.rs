use amiquip::{Connection, ConsumerOptions, QueueDeclareOptions, Result};
use chrono::offset;

fn parse_data(raw: &[u8]) -> [u8; 8] {
    raw.try_into().expect("parse error")
}

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let queue = channel.queue_declare("test", QueueDeclareOptions::default())?;
    let consumer = queue.consume(ConsumerOptions::default())?;

    let mut durations: Vec<i64> = Vec::new();

    for (i, message) in consumer.receiver().iter().enumerate() {
        match message {
            amiquip::ConsumerMessage::Delivery(delivery) => {
                let data = parse_data(&delivery.body[..8]);
                let send_time = i64::from_be_bytes(data);
                let now = offset::Utc::now().timestamp_millis();
                let time_diff = now - send_time;

                durations.push(time_diff);

                consumer.ack(delivery)?;

                if i + 1 == 40 * 10_000 {
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

