use rand::Rng;
use log::info;
use log::LevelFilter;
use env_logger;
mod utils;

fn main() {
    env_logger::builder().filter_level(
        LevelFilter::Info
    ).init();
    let mut seed = rand::thread_rng();
    let n1: u8 = seed.gen();
    println!("{:?}", n1);
    let v: [i32; 4] = [1i32, 2i32, 3i32, 4i32];
    println!("{:?}", v);
    info!("this is test info message");
    let mut test = utils::arena::Arena::new();
    test.set(10i32);
    let res = test.allocate(4usize);
    println!("{:?}", res);
    println!("{:?}",test.get());
    let r = format!("Count: {:.2}  Average: {:.4}  StdDev: {:.2} \n",
                        1f64, 15.0,100.0);
    println!("{}",r);
}
