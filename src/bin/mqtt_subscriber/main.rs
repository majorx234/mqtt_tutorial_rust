use std::time::Duration;
use std::{process, thread};
use uuid::Uuid;

extern crate paho_mqtt as mqtt;

fn try_reconnect(mqtt_client: &mqtt::Client) -> bool {
    println!("Connection lost. Reconnecting...");
    for _ in 0..60 {
        thread::sleep(Duration::from_secs(1));
        if mqtt_client.reconnect().is_ok() {
            println!("  Successfully reconnected");
            return true;
        }
    }
    println!("Unable to reconnect after several attempts.");
    false
}

fn main() {
    let id = Uuid::new_v4();

    println!("WIP mqtt subscriberer with ID:{}!", id);
    let client_id = format!("mqtt_subscriber_rust_{}", id);
    let host = "mqtt://localhost:1883".to_string();

    let create_opts = mqtt::CreateOptionsBuilder::new()
        .server_uri(host)
        .persistence("persist")
        .client_id(client_id)
        .finalize();

    let mqtt_client = mqtt::Client::new(create_opts).unwrap_or_else(|e| {
        println!("Error creating the client: {:?}", e);
        process::exit(1);
    });

    let rx = mqtt_client.start_consuming();

    // Define the set of options for the connection
    let lwt = mqtt::MessageBuilder::new()
        .topic("status")
        .payload("Sync consumer lost connection")
        .finalize();

    let conn_opts = mqtt::ConnectOptionsBuilder::new()
        .keep_alive_interval(Duration::from_secs(20))
        .clean_session(false)
        .will_message(lwt)
        .finalize();

    match mqtt_client.connect(conn_opts) {
        Ok(rsp) => {
            if let Some(conn_rsp) = rsp.connect_response() {
                println!(
                    "Connected to: '{}' with MQTT version {}",
                    conn_rsp.server_uri, conn_rsp.mqtt_version
                );
                if conn_rsp.session_present {
                    println!("  w/ client session already present on broker.");
                } else {
                    mqtt_client
                        .subscribe("testing", mqtt::QOS_1)
                        .and_then(|rsp| {
                            rsp.subscribe_response()
                                .ok_or(mqtt::Error::General("Bad response"))
                        })
                        .map(|vqos| {
                            println!("QoS granted: {:?}", vqos);
                        })
                        .unwrap_or_else(|err| {
                            println!("Error subscribing to topics: {:?}", err);
                            mqtt_client.disconnect(None).unwrap();
                            process::exit(1);
                        });
                }
            }
        }
        Err(e) => {
            println!("Error connecting to the broker: {:?}", e);
            process::exit(1);
        }
    }

    println!("\nWaiting for messages on topic 'testing'");
    for msg in rx.iter() {
        if let Some(msg) = msg {
            println!("{}", msg);
        } else if mqtt_client.is_connected() || !try_reconnect(&mqtt_client) {
            break;
        }
    }

    // If we're still connected, then disconnect now,
    // otherwise we're already disconnected.
    if mqtt_client.is_connected() {
        println!("\nDisconnecting...");
        mqtt_client.disconnect(None).unwrap();
    }
}
