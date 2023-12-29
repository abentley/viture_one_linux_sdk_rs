use std::io;
use std::io::BufRead;
use viture_one_sdk::{CallbackImu, ImuData, ImuFrequency, Sdk, State3d};

pub struct Printer {}

impl CallbackImu for Printer {
    fn imu_message(data: ImuData, ts: u32) {
        eprint!(
            "\rroll: {:>7.2} pitch {:>7.2} yaw {:>7.2} ts {ts}",
            data.roll, data.pitch, data.yaw
        );
    }
}

fn help() {
    println!("Available commands: 3d, 2d, 3d-state, imu, imnotu, imu-state, quit");
}

fn process_commands(sdk: &mut Sdk) {
    let stdin = io::stdin();
    println!();
    help();
    for line in stdin.lock().lines().flatten() {
        match line.as_str() {
            "3d" => {
                println!("Entering the third dimension");
                sdk.set_3d(State3d::On).unwrap();
            }
            "2d" => {
                println!("Leaving the third dimension");
                sdk.set_3d(State3d::Off).unwrap();
            }
            "3d-state" => {
                println!("3d state: {:?}", sdk.get_3d_state().unwrap())
            }
            "imu" => {
                println!("Engaging IMU");
                sdk.set_imu(true).unwrap();
                eprintln!();
            }
            "imnotu" => {
                println!("Disengaging IMU");
                sdk.set_imu(false).unwrap();
                eprintln!();
            }
            "imu-state" => {
                println!("imu state: {}", sdk.get_imu_state().unwrap())
            }
            "imu-60" => {
                println!("Setting IMU to 60 Hz");
                sdk.set_imu_fq(ImuFrequency::Hz60).unwrap();
                eprintln!();
            }
            "imu-90" => {
                println!("Setting IMU to 90 Hz");
                sdk.set_imu_fq(ImuFrequency::Hz90).unwrap();
                eprintln!();
            }
            "imu-120" => {
                println!("Setting IMU to 120 Hz");
                sdk.set_imu_fq(ImuFrequency::Hz120).unwrap();
                eprintln!();
            }
            "imu-240" => {
                println!("Setting IMU to 240 Hz");
                sdk.set_imu_fq(ImuFrequency::Hz240).unwrap();
                eprintln!();
            }
            "imu-freq" => {
                println!("Imu Frequency: {:?}", sdk.get_imu_fq().unwrap());
                eprintln!();
            }
            "help" => help(),
            "?" => help(),
            "quit" => return,
            cmd => {
                println!("Unknown command: {}", cmd);
            }
        }
    }
}

fn main() {
    eprint!("Initializing...");
    let Ok(mut sdk) = Sdk::init::<Printer>() else {
        eprintln!(" failed to initialize.  Make sure to run this as root.");
        return;
    };
    eprintln!(" succeeded");
    process_commands(&mut sdk);
    eprint!("Deinitializing...");
    drop(sdk);
    eprintln!();
}
