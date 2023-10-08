use esp_backtrace as _;

use esp_println::println;
use esp_wifi::esp_now::{PeerInfo, BROADCAST_ADDRESS};
use esp_wifi::{initialize, EspWifiInitFor};
use hal::clock::ClockControl;
use hal::{peripherals::Peripherals, prelude::*};
use hal::{systimer::SystemTimer, Rng};

pub struct Communicator<'a> {
	esp_now: esp_wifi::esp_now::EspNow<'a>,
}

impl<'a> Communicator<'a> {
	pub fn new() -> Self {
		let peripherals = Peripherals::take();

		let system = peripherals.SYSTEM.split();
		let clocks = ClockControl::max(system.clock_control).freeze();

		let timer = SystemTimer::new(peripherals.SYSTIMER).alarm0;
		let init = initialize(
			EspWifiInitFor::Wifi,
			timer,
			Rng::new(peripherals.RNG),
			system.radio_clock_control,
			&clocks,
		)
		.unwrap();

		let (wifi, ..) = peripherals.RADIO.split();
		let esp_now = esp_wifi::esp_now::EspNow::new(&init, wifi).unwrap();

		println!("esp-now version {}", esp_now.get_version().unwrap());

		Communicator { esp_now }
	}

	pub fn receive_broadcast(&mut self) {
		let r = self.esp_now.receive();
		if let Some(r) = r {
			println!("Received {:?}", r);

			if r.info.dst_address == BROADCAST_ADDRESS {
				if !self.esp_now.peer_exists(&r.info.src_address) {
					self.esp_now
						.add_peer(PeerInfo {
							peer_address: r.info.src_address,
							lmk: None,
							channel: None,
							encrypt: false,
						})
						.unwrap();
				}
				let status = self
					.esp_now
					.send(&r.info.src_address, b"Hello Peer")
					.unwrap()
					.wait();
				println!("Send hello to peer status: {:?}", status);
			}
		}
	}

	pub fn broadcast(&mut self, data: &[u8]) {
		println!("Send");
		let status = self.esp_now.send(&BROADCAST_ADDRESS, data).unwrap().wait();
		println!("Send broadcast status: {:?}", status)
	}
}
