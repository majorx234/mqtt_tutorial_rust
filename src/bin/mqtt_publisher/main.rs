use std::process;
use std::{thread, time};

extern crate paho_mqtt as mqtt;

fn main() {
    println!("WIP mqtt publisher!");
    let host = "mqtt://localhost:1883".to_string();

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .persistence("persist")
        .client_id("mqtt_publisher_rust")
        .finalize();

    let mqtt_client = mqtt::AsyncClient::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    let conn_opts = mqtt::ConnectOptions::new();

    // Connect and wait for it to complete or fail
    if let Err(err) = mqtt_client.connect(conn_opts).wait() {
        println!("Unable to connect: {}", err);
        process::exit(1);
    }
    let topic = mqtt::Topic::new(&mqtt_client, "testing", mqtt::QOS_1);

    let mut value = 0;
    for _ in 0..10 {
        let msg = format!("{{\"value\": \"{}\"}}", value);
        let msg_token = topic.publish(msg);
        if let Err(e) = msg_token.wait() {
            println!("Error sending message: {:?}", e);
            break;
        }
        thread::sleep(time::Duration::from_millis(1000));
        value += 1;
    }

    let tok = mqtt_client.disconnect(None);
    tok.wait().unwrap();
}
