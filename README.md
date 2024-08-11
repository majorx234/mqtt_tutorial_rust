# Info
- WIP
- MQTT examples in Rust

# Tryouts with paho-mqtt
- https://github.com/eclipse/paho.mqtt.rust
## Example mqtt_publisher
- simple publisher
- uses async lient
- uses Topic-struct
  - `topic.publish(msg)` sends messages
    - return a token
    - this ca be used for blocking (`token.wait()`)
  - even `client.disconnect(None)` returns token
## Example mqtt_subscriber
- simple subscriber
  - uses oldschool synced API
  - `client.connect(conn_opt)` connects the clien to the broker
    - `conn_opt` - contains struct connection options (lastwill, keepalive etc)
  - `let rx = client.start_consuming()`
    - creates some kind of message queue to receive messages
  - `client.subscribe(topciname, qos_level)`
    - subscribes topic with `topic_name`
  - afterwars you can iterate over the genereated queue
    - `for msg in rx.iter() { ...`
