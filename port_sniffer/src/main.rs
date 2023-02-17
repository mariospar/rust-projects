use bpaf::Bpaf;
use std::io::{self, Write};
use std::net::{IpAddr, Ipv4Addr};
use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use tokio::net::TcpStream;
use tokio::task;

const IP_FALLBACK: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
const MAX_PORT: u16 = 65535;

#[derive(Debug, Clone, Bpaf)]
#[bpaf(options)]
pub struct Arguments {
    #[bpaf(long, short, fallback(IP_FALLBACK))]
    /// The address that you want to sniff. Must be a valid IPv4 address. Falls back to 127.0.0.1
    pub ip_addr: IpAddr,
    #[bpaf(
        long("start"),
        short('s'),
        guard(start_port_guard, "Must be greater than 0."),
        fallback(1u16)
    )]
    /// The start port for the sniffer (must be greater than 0)
    pub start_port: u16,
    #[bpaf(
        long("end"),
        short('e'),
        guard(end_port_guard, "Must be less than 65535."),
        fallback(MAX_PORT)
    )]
    /// The end port for the sniffer (must be less than or equal to 65535)
    pub end_port: u16,
}

fn start_port_guard(input: &u16) -> bool {
    *input > 0
}

fn end_port_guard(input: &u16) -> bool {
    *input <= MAX_PORT
}

async fn scan(trans: Sender<u16>, port: u16, addr: IpAddr) {
    match TcpStream::connect(format!("{}:{}", addr, port)).await {
        Ok(_) => {
            io::stdout().flush().unwrap();
            trans.send(port).unwrap();
        }
        Err(_) => {}
    }
}

#[tokio::main]
async fn main() {
    let opts: Arguments = arguments().run();
    let (trans, rcv) = channel();
    print!("Loading");
    for i in opts.start_port..opts.end_port {
        let trans = trans.clone();

        task::spawn(async move {
            scan(trans, i, opts.ip_addr).await;
        });
    }
    for _ in 0..3 {
        // Print a dot every second
        print!(".");
        std::io::stdout().flush().unwrap();
        thread::sleep(Duration::from_secs(1));
    }

    let mut open_ports = vec![];
    drop(trans);
    for rc in rcv {
        open_ports.push(rc);
    }

    println!("");
    open_ports.sort();
    for port in open_ports {
        println!("{} is open", port);
    }
}
