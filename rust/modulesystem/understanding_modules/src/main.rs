mod config;
mod health;
mod doctor;
fn main() {
    println!("Hello, world!");
    config::printconfig();
    health::healthtest::printhealth();
}
