use libtor::{HiddenServiceVersion, Tor, TorAddress, TorFlag};
use reqwest::Error;
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};
use tari_shutdown::Shutdown;
use tokio::task;

const SOCKS_PORT: u16 = 19_050;
const CHECK_URL: &str = "https://check.torproject.org";

/// blocking - libtor also has `start_background()` which uses `thread::spawn`
fn tor() {
    match Tor::new()
        .flag(TorFlag::DataDirectory("/tmp/tor-rust".into()))
        .flag(TorFlag::SocksPort(SOCKS_PORT))
        .flag(TorFlag::HiddenServiceDir("/tmp/tor-rust/hs-dir".into()))
        .flag(TorFlag::HiddenServiceVersion(HiddenServiceVersion::V3))
        .flag(TorFlag::HiddenServicePort(
            TorAddress::Port(8000),
            None.into(),
        ))
        .start()
    {
        Ok(r) => println!("tor exit result: {}", r),
        Err(e) => eprintln!("tor error: {}", e),
    };
}

fn dots() {
    loop {
        print!(".");
        io::stdout().flush().unwrap();
        thread::sleep(Duration::from_millis(100));
    }
}

async fn poll_reqs() -> Result<(), Error> {
    println!("---poll");
    let proxy = reqwest::Proxy::https(format!("socks5://127.0.0.1:{}", SOCKS_PORT))?;
    let client = reqwest::ClientBuilder::new().proxy(proxy).build()?;

    loop {
        thread::sleep(Duration::from_millis(500));
        let request = client.get(CHECK_URL).build()?;
        println!("---request");
        let response = client.execute(request).await;
        match response {
            Ok(r) => {
                let html = r.text().await.unwrap();
                match (html.find("Congratulations"), html.find("Sorry")) {
                    (Some(_), None) => {
                        println!("Tor is online!");
                    }
                    (None, Some(_)) => {
                        println!("Received a response but not through Tor...");
                    }
                    _ => panic!("...at the disco"),
                }
                break;
            }
            Err(e) => {
                println!("---error: {}", e.to_string());
                eprintln!("{}", e.to_string());
            }
        };
        io::stdout().flush().unwrap();
    }
    Ok(())
}

#[tokio::main]
async fn main() {
    let mut shutdown = Shutdown::new();
    let shutdown_signal = shutdown.to_signal();

    // the first ctrl+c is intercepted by the tor daemon to exit
    // the second ctrl+c fires this shutdown
    ctrlc::set_handler(move || {
        println!("received Ctrl+C!");
        shutdown.trigger().unwrap();
    })
    .unwrap();

    task::spawn_blocking(|| tor());
    task::spawn_blocking(|| dots());
    let handle = task::spawn(async move {
        poll_reqs().await.unwrap();
    });
    tokio::join!(handle).0.unwrap();

    tokio::select! {
        _ = shutdown_signal => {
            println!("received shutdown signal");
            std::process::exit(0);
        },
    }
}
