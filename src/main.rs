use rand::Rng;
use log::info;
use log::LevelFilter;
use env_logger;
fn main() {
    env_logger::builder().filter_level(
        LevelFilter::Info
    ).init();
    let mut seed = rand::thread_rng();
    let n1:u8 = seed.gen();
    println!("{:?}",n1);
    let v: [i32; 4] = [1i32,2i32,3i32,4i32];
    println!("{:?}",v);
    info!("this is test info message");
    println!("Hello, world!");
}
