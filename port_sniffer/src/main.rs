use std::sync::mpsc::{channel, Sender};
use std::{
    env,
    net::{IpAddr, TcpStream},
    process,
    str::FromStr,
};
use std::{
    io::{self, Write},
    thread,
    time::Duration,
};

const MAX: u16 = 65535;

struct Arguments {
    flag: String,
    ip_addr: IpAddr,
    threads: u16,
}

impl Arguments {
    fn new(args: &[String]) -> Result<Arguments, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments");
        } else if args.len() > 4 {
            return Err("Too many arguments");
        }

        let unparsed_ip = args[1].clone();
        if let Ok(ip_addr) = IpAddr::from_str(&unparsed_ip) {
            return Ok(Arguments {
                flag: String::from(""),
                ip_addr,
                threads: 4,
            });
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || flag.contains("--help") && args.len() == 2 {
                println!(
                    "Usage: -j to select how many threads you want
                \r\n
                -h or --help to show this help message"
                );
                return Err("help");
            } else if flag.contains("-h") || flag.contains("--help") {
                return Err("Too many arguments");
            } else if flag.contains("-j") {
                let ip_addr = match IpAddr::from_str(&args[3]) {
                    Ok(s) => s,
                    Err(_) => return Err("Not a valid IP Address"),
                };
                let threads = match args[2].parse::<u16>() {
                    Ok(s) => s,
                    Err(_) => return Err("Failed to parse thread number"),
                };
                return Ok(Arguments {
                    flag,
                    ip_addr,
                    threads,
                });
            } else {
                return Err("Invalid Syntax");
            }
        }
    }
}

fn scan(trans: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut port: u16 = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                io::stdout().flush().unwrap();
                trans.send(port).unwrap();
            }
            Err(_) => {}
        }

        if (MAX - port) <= num_threads {
            break;
        }

        port += num_threads;
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let arguments = Arguments::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0);
        } else {
            eprintln!("{} problem parsing arguments: {}", program, err);
            process::exit(0);
        }
    });

    let num_threads = arguments.threads;
    let addr = arguments.ip_addr;
    let (trans, rcv) = channel();
    print!("Loading");
    for i in 0..num_threads {
        let trans = trans.clone();

        thread::spawn(move || {
            scan(trans, i, addr, num_threads);
        });
    }
    for _ in 0..20 {
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
