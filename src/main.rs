use libtor::{HiddenServiceVersion, Tor, TorAddress, TorFlag};
use std::{thread::sleep, time::Duration};
use tokio::{task, time};

#[tokio::main]
async fn main() {
    task::spawn(async move {
        match Tor::new()
            .flag(TorFlag::DataDirectory("/tmp/tor-rust".into()))
            .flag(TorFlag::SocksPort(19050))
            .flag(TorFlag::HiddenServiceDir("/tmp/tor-rust/hs-dir".into()))
            .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
            .flag(TorFlag::HiddenServicePort(
                TorAddress::Port(8000),
                None.into(),
            ))
            .start()
        {
            Ok(r) => println!("{}", r),
            Err(e) => eprintln!("{}", e),
        }
    });

    loop {
        print!(".");
        time::sleep(Duration::from_secs(1)).await;
    }
}
