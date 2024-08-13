use clap::Parser;
use paho_mqtt as mqtt;
use std::process;
use std::{thread, time};
use uuid::Uuid;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// name of the client
    #[arg(short, long, value_name = "client_id")]
    pub client_id: Option<String>,
    /// host url
    #[arg(short, long, value_name = "host")]
    pub host: Option<String>,
    /// host port
    #[arg(short, long, value_name = "port")]
    pub port: Option<u16>,
}

fn main() {
    let uuid = Uuid::new_v4();

    let client_id = Args::parse().client_id.map_or_else(
        || format!("mqtt_publisher_rust_{}", uuid),
        |client_id| format!("{}_{}", client_id, uuid),
    );

    let port = Args::parse().port.map_or_else(|| 1883, |port| port);

    println!("WIP mqtt publisher with ID:{}!", client_id);
    let host_url = Args::parse().host.map_or_else(
        || format!("mqtt://localhost:{}", port).to_string(),
        |host| format!("mqtt://{}:{}", host, port).to_string(),
    );

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host_url)
        .persistence("persist")
        .client_id(client_id)
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
