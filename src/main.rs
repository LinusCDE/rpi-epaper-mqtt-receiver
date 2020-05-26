mod eink;
mod eink_calls;
use crate::eink::Color::White;
use crate::eink_calls::Call;
use std::path;
use std::process::exit;
use std::fs::File;
use std::io::{Read, Write};
use std::thread;
use rand;
use rand::{Rng, thread_rng};
use rumqtt::{MqttOptions, MqttClient};
use rumqtt::QoS::AtMostOnce;
use rumqtt::client::Notification::Publish;
use serde::Deserialize;
use toml;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, Mutex};
use crate::eink::{DisplayMode, EInk};
use crate::eink_calls::Call::Display;


#[derive(Deserialize, Debug)]
struct Config {
    host: String,
    port: u16,
    id: String,
    username: Option<String>,
    password: Option<String>,
    channels: Channels,
}

#[derive(Deserialize, Debug)]
struct Channels {
    calls: String,
    error: String,
}

struct TimestampedCalls {
    calls: Vec<Call>,
    received_at: u128
}

fn read_string_or_create_empty(path: &str) -> Result<String, &str> {
    if path::Path::new(path).exists() {

        match File::open(path) {
            Ok(mut file) => {
                let mut content = String::new();
                file.read_to_string(&mut content).unwrap();
                return Ok(content);
            },
            Err(_) => Err("Failed to load file!")
        }

    }else {
        let default_config = format!(
            "host = \"localhost\"\n\
            port = 1883\n\
            id = \"rpi-epaper-mqtt-receiver-{}\"\n\
            #username = \"user\"\n\
            #password = \"pass\"\n\
            \n\
            [channels]\n\
            calls = \"cmnd/rpi-epaper-mqtt-receiver/calls\"\n\
            error = \"stat/rpi-epaper-mqtt-receiver/error\"\n",
            rand::thread_rng().gen_range(0, 10000)
        );
        match File::create(path) {
            Ok(mut file) => {
                write!(file, "{}", default_config);
                file.flush().unwrap();
            },
            Err(e) => panic!(e)
        }
        return Ok(String::from(default_config)); // Empty new file
    }
}


fn read_config_or_panic(path: &str) -> Config {
    match read_string_or_create_empty(path.clone()) {
        Ok(content) => {
            match toml::from_str::<Config>(content.as_str()) {
                Ok(config) => config,
                Err(err) => {
                    println!("Error: {}", err);
                    exit(1);
                }
            }
        },
        Err(reason) => {
            println!("Error: {}", reason);
            exit(1);
        }
    }
}


fn print_config(config: &mut Config) {
    println!("Address: {}:{}", config.host, config.port);
    println!("Id: {}", config.id);
    if let Some(user) = &config.username {
        println!("Username: {}", user);
    }else {
        println!("Username: not set");
    }
    if let Some(_) = config.password {
        println!("Password: set");
    }else {
        println!("Password: not set");
    }
    println!("Calls: {}", config.channels.calls);
    println!("Error: {}", config.channels.error);
}


fn currentTime() -> u128 {
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis()
}


fn main() {
    println!("Loading toml...");
    let mut config = read_config_or_panic("config.toml");
    print_config(&mut config);
    let mqtt_options = MqttOptions::new(&config.id, &config.host, config.port)
        .set_connection_timeout(120) // Seconds
        .set_keep_alive(120) // Seconds
        .set_request_channel_capacity(128); // Cached messages?
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    println!("Initializing display...");
    let mut eink = eink::EInk::new(White);
    eink.delay(500);

    println!("Start listening on MQTT...");
    mqtt_client.subscribe(config.channels.calls, AtMostOnce);


    let timestamped_calls_queue: Vec<TimestampedCalls> = Vec::new();
    let timestamped_calls_queue_mutex = Arc::new(Mutex::new(timestamped_calls_queue));
    let eink_mutex = Mutex::new(eink);

    let timestamped_calls_queue_mutexRecv = Arc::clone(&timestamped_calls_queue_mutex);
    let handler = thread::spawn( move || {
        loop {
            thread::sleep(Duration::from_millis(50));
            loop {
                let nextCalls: Option<TimestampedCalls> = timestamped_calls_queue_mutexRecv.lock().unwrap().pop();
                match nextCalls {
                    Some(timestampedCalls) => {
                        for timestampedCall in timestampedCalls.calls {
                            match &timestampedCall {
                                Display(call) => {
                                    if currentTime() - timestampedCalls.received_at > 3000 {
                                        println!("Discarded outdated call!");
                                        // To old and takes only more time!
                                    } else {
                                        timestampedCall.call(&mut eink_mutex.lock().unwrap());
                                    }
                                },
                                _ => { timestampedCall.call(&mut eink_mutex.lock().unwrap()); }
                            };
                        }
                    }
                    None => break
                }
            }
        }
    });

    for n in notifications {
        if let Publish(message) = n {
            // Decode mqtt payload
            if let Ok(plain_text) = String::from_utf8(message.payload.to_vec()) {
                // Parse and serialize
                match serde_json::from_str::<Vec<Call>>(&plain_text) {
                    Ok(calls) => {
                        timestamped_calls_queue_mutex.lock().unwrap().push(TimestampedCalls {
                            calls, received_at: currentTime() })
                    },
                    Err(err) => {
                        // Handle parsing/serialization error
                        mqtt_client.publish(&config.channels.error, AtMostOnce, false,
                                            format!("Couldn't parse or serialize json: {}", err));
                    }
                }
            }
        }
    }
}