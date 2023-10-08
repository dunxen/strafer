#![no_std]
#![no_main]

mod comms;

use comms::Communicator;
use esp_backtrace as _;
use esp_wifi::current_millis;

#[hal::entry]
fn main() -> ! {
	#[cfg(feature = "log")]
	esp_println::logger::init_logger(log::LevelFilter::Info);

	let mut communicator = Communicator::new();

	let mut next_send_time = current_millis() + 5 * 1000;
	loop {
		communicator.receive_broadcast();

		if current_millis() >= next_send_time {
			next_send_time = current_millis() + 5 * 1000;
			communicator.broadcast(b"0123456789");
		}
	}
}
