use rand::prelude::*;
use redis::Commands;
use serde::{Deserialize, Serialize};
use std::env;

// use std::thread;
// static NTHREADS: i32 = 10;

struct Config {
    redis_host: String,
}

#[derive(Serialize, Deserialize)]
struct Point {
    x: f32,
    y: f32,
}

#[derive(Serialize, Deserialize)]
struct Snap {
    id: u64,
    device_id: u8,
    location: Point,
    direction: u8,
    speed: f32,
    altitude: u8,
    gforce: u8,
}

fn generate_snap(id: u64) -> Snap {
    let location = Point {
        x: random(),
        y: random(),
    };
    return Snap {
        id,
        device_id: random(),
        location,
        direction: random(),
        speed: random(),
        altitude: random(),
        gforce: random(),
    };
}

fn redis_connect(url: &String) -> redis::Connection {
    let url: &str = &format!("redis://{}", url);
    let client = redis::Client::open(url).expect("Failed to read line");
    return client.get_connection().expect("Failed to read line 2");
}

fn publish(conn: &mut redis::Connection, id: u64) -> redis::RedisResult<()> {
    let snap = generate_snap(id);
    let msg: String = serde_json::to_string(&snap).expect("Oops");
    let _: () = conn.publish("test", msg)?;
    Ok(())
}

fn run(cfg: &Config) {
    let mut conn = redis_connect(&cfg.redis_host);
    // let mut pubsub = conn.as_pubsub();
    let mut nb: u64 = 0;

    loop {
        // conn.get("test").expect("Failed to read line");
        publish(&mut conn, nb).expect("Oops ther has been an error");

        nb = nb + 1;
    }

    // match TcpStream::connect(&format!("{}:{}", cfg.relay_ip, cfg.relay_port)) {
    //     Ok(stream) => {
    //         println!("Successfully connected to server in port 5000");

    //         process_queue(pubsub, stream);
    //     }
    //     Err(e) => println!("Failed to connect: {}", e),
    // };
}

fn main() {
    let redis_host = match env::var("REDIS_HOST") {
        Ok(val) => val,
        _ => String::from("localhost"),
    };

    // let relay_ip = match env::var("RELAY_IP") {
    //     Ok(val) => val,
    //     _ => String::from("127.0.0.1"),
    // };

    // let relay_port = match env::var("RELAY_PORT") {
    //     Ok(val) => val,
    //     _ => String::from("5000"),
    // };

    let config = Config { redis_host };

    run(&config)
}
