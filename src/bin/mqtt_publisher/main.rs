use std::process;

extern crate paho_mqtt as mqtt;

fn main() {
    println!("WIP mqtt publisher!");
    let host = "mqtt://localhost:1883".to_string();

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .persistence("persist")
        .client_id("mqtt_publisher_rust")
        .finalize();

    let cli = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptions::new();

    // Connect and wait for it to complete or fail
    if let Err(err) = cli.connect(conn_opts).wait() {
        println!("Unable to connect: {}", err);
        process::exit(1);
    }

    let msg = mqtt::Message::new("test", "Hello world!", QOS);
}
