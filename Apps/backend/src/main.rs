use std::{env};
use std::net::{UdpSocket};
use tokio::time::{self, Duration};
use ndarray::{Array2, Axis};

#[tokio::main]
async fn main() {
    
    // default parameters
    let host: &str = env!("HOST");
    let target: &str = env!("TARGET");
    let frequency: f64 = env!("FREQUENCY").parse::<f64>().unwrap();

    println!("Host: {host}");
    println!("Target: {target}");
    println!("Frequency: {:?} [Hz]", frequency);
    
    println!("Setting up UDP socket");
    let socket: UdpSocket = UdpSocket::bind(host).expect("couldn't bind to address.");
    println!("Created UDP socket");  
    socket.connect(target).expect("connect function failed");

    println!("Beginning to send ...");
    run(&socket, frequency).await;
}

async fn run(socket: &UdpSocket, frequency: f64) {
    let mut interval = time::interval(Duration::from_nanos((1.0 / frequency * 1_000_000_000.0) as u64));

    let f: f64 = env!("SIGNAL_FREQUENCY").parse::<f64>().unwrap();
    let dur: f64 = 1.0 / f;
    let a: f64 = env!("SIGNAL_AMPLITUDE").parse::<f64>().unwrap();
    let sig: &str = env!("SIGNAL_TYPE").to_lowercase();

    let step: f64 = 1.0 / frequency;
    let mut t: f64 = 0.0;

    let pi: f64 = std::f64::consts::PI;

    loop {
        // generate data
        let mut v: f64;
        match sig {
            "sin" => { v = a * (pi * 2.0 * f * t).sin(); },
            "cos" => { v = a * (pi * 2.0 * f * t).cos(); },
            "saw" => { v = a * ((t / dur) * 2.0 - 1.0) },
            "stp" => { if (t - (dur / 2.0) >= 0.0) { v = -a; } else { v = a; } },
            _ => { v = 0.0; }
        }
        let data: [u8; 8] = v.to_ne_bytes();
        t += step;
        while (t >= dur) { t -= dur; }

        // send packet
        match socket.send(&data) {
            Ok(n) => println!("sent value {:.4} as {n} bytes: {:?}", v, &data),
            Err(e) => println!("failed sending: {e:?}.")
        }

        // wait to match desired frequency
        interval.tick().await;
    }
}

fn sample_data_channel() -> [u8;30] {

    let mut data = Array2::<u8>::zeros((10, 3));
    for (_, mut row) in data.axis_iter_mut(Axis(0)).enumerate() {
        // Perform calculations and assign to `row`; this is a trivial example:
        row.fill(1);
    }
    let flat = data.into_shape(30).unwrap();
    let bytearray = flat.as_slice().expect("oops");
    bytearray.try_into().expect("ooops")
}
