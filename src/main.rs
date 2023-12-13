use std::io;
use std::io::BufRead;
use viture_rs::{CallbackImu, CallbackMcu, ImuData, Sdk};

pub struct Printer {}

impl CallbackImu for Printer {
    fn imu_message(data: &ImuData, ts: u32) {
        eprintln!(
            "roll: {:.2} pitch {:.2} yaw {:.2} ts {ts}",
            data.roll, data.pitch, data.yaw
        );
    }
}

impl CallbackMcu for Printer {}

fn process_commands(sdk: &mut Sdk) {
    let stdin = io::stdin();
    println!();
    println!("Available commands: 3d, 2d, 3d-state, imu, imnotu, imu-state, quit");
    for line in stdin.lock().lines().flatten() {
        match line.as_str() {
            "3d" => {
                println!("Entering the third dimension");
                sdk.set_3d(true).unwrap();
            }
            "2d" => {
                println!("Leaving the third dimension");
                sdk.set_3d(false).unwrap();
            }
            "3d-state" => {
                println!("3d state: {}", sdk.get_3d_state().unwrap())
            }
            "imu" => {
                println!("Engaging IMU");
                sdk.set_imu(true).unwrap();
            }
            "imnotu" => {
                println!("Disengaging IMU");
                sdk.set_imu(false).unwrap();
            }
            "imu-state" => {
                println!("imu state: {}", sdk.get_imu_state().unwrap())
            }
            "quit" => return,
            cmd => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}

fn main() {
    eprint!("Initializing...");
    let Ok(mut sdk) = Sdk::init::<Printer, Printer>() else {
        eprintln!(" failed to initialize");
        return;
    };
    eprintln!(" succeeded");
    process_commands(&mut sdk);
    eprint!("Deinitializing...");
    drop(sdk);
    eprintln!();
}
