use viture_rs::Sdk;

fn main() {
    let Ok(sdk) = Sdk::init() else {
        eprintln!("Failed to initialize")
        return;
    };
    println!("{:?}", sdk);
    sdk.set_imu(true).unwrap();
    sdk.set_3d(true).unwrap();
}
