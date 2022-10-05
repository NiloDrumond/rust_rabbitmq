use chrono::offset;

use amiquip::{Connection, Exchange, Publish, Result};

fn main() -> Result<()> {
    let mut connection = Connection::insecure_open("amqp://guest:guest@localhost:5672")?;
    let channel = connection.open_channel(None)?;
    let exchange = Exchange::direct(&channel);

    for _ in 0..10_000 {
        let now = offset::Utc::now();
        let now = now.timestamp_millis();
        let mut now = Vec::from(now.to_be_bytes());
        now.resize(1024, 0);

        exchange.publish(Publish::new(&now, "test"))?;
    }

    connection.close()
}
