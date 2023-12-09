use viture_rs::Sdk;

fn main() {
    let Some(sdk) = Sdk::safe_init() else {
        eprintln!("Failed to initialize");
        return;
    };
    println!("{:?}", sdk);
    sdk.set_imu(true).unwrap();
}
