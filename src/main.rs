#![no_std]
#![no_main]

use esp_backtrace as _;
use esp_hal::{clock::CpuClock, main, time::{Duration, Instant}, Config};
use esp_println::println;

#[main]
fn main() -> ! {
    let config = Config::default().with_cpu_clock(CpuClock::max());
    let _peripherals = esp_hal::init(config);

    println!("ESP32-S3 Rust runtime siap.");

    loop {
        println!("heartbeat");
        let start = Instant::now();
        while start.elapsed() < Duration::from_millis(1_000) {}
    }
}
