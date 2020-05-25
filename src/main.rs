use crate::eink::Color::{Black, White};
use crate::eink::{Pos, Color, EInk};
use toml;
use std::path;
use std::process::exit;
use std::fs::{File, read_to_string};
use std::io::{Read, Write};
use serde::Deserialize;
use rand;
use rand::Rng;
use rumqtt::{MqttOptions, MqttClient};
use rumqtt::QoS::{ExactlyOnce, AtMostOnce};
use rumqtt::client::Notification::Publish;
use std::sync::Arc;
use libepd::Font20;
use crate::eink_calls::{ClearCall, TextCall, Callable, Call};
use crate::eink_calls::Call::{Text, Clear, Display};

mod eink;
mod eink_calls;


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
    call: String,
    error: String,
}

#[derive(Deserialize, Debug)]
struct CallContainer {
    call: Call
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
            call = \"cmnd/rpi-epaper-mqtt-receiver/call\"\n\
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
    println!("Call: {}", config.channels.call);
    println!("Error: {}", config.channels.error);
}

fn main() {
    println!("Loading toml...");
    let mut config = read_config_or_panic("config.toml");
    print_config(&mut config);
    let mqtt_options = MqttOptions::new(&config.id, &config.host, config.port);
    let (mut mqtt_client, notifications) = MqttClient::start(mqtt_options).unwrap();
    mqtt_client.subscribe(config.channels.call, AtMostOnce);

    println!("Initializing...");

    let mut eink = eink::EInk::new(Black);
    eink.to_partial_mode();
    eink.delay(1000);
    //eink.draw_line(Pos {x: 0, y: 10}, Pos {x: eink.get_width(), y: 10}, White, 1);
    //eink.display();
    //eink.delay(500);
    //eink.draw_string(Pos {x: 20,y: 40}, "Hello World!", 24, White);
    //eink.display();
    //eink.delay(1000);

    for n in notifications {
        if let Publish(message) = n {
            if let Ok(plain_text) = String::from_utf8(message.payload.to_vec()) {
                let mut error_text: Option<String> = None;
                match serde_json::from_str::<CallContainer>(&plain_text) {
                    Ok(container) => {
                        if let Err(reason) = container.call.call(&mut eink) {
                            error_text = Some(reason)
                        }
                    },
                    Err(e) => error_text = Some(format!("Couldn't parse or serialize json: {}", e))
                }

                if let Some(message) = error_text {
                    mqtt_client.publish(&config.channels.error, AtMostOnce,
                                        false, message.as_str());
                }
            }
        }
    }

    /*
    eink.to_full_mode();
    eink.clear(White, None, None);
    eink.draw_string(Pos {x: 20, y: 40}, "Hello World!", 24, Black);
    eink.display();*/
    //eink.drawLine()

    //EPD_2in13_V2_test(); // Ensure to have pic folder in working dir!
}