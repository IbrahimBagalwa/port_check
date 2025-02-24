use std::{
    env,
    io::{ self, Write },
    net::{ IpAddr, TcpStream },
    process,
    str::FromStr,
    sync::mpsc::{ channel, Sender },
    thread,
    vec,
};

const MAX: u16 = 65535;
struct ArgsFlag {
    flag: String,
    ip_addr: IpAddr,
    thread: u16,
}

impl ArgsFlag {
    fn new(args: &[String]) -> Result<ArgsFlag, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments. Please provide thread count and IP address.");
        } else if args.len() > 4 {
            return Err("Too many arguments. Please provide");
        }
        let f = args[1].clone();
        if let Ok(ip_addr) = IpAddr::from_str(&f) {
            Ok(ArgsFlag { flag: String::from(""), ip_addr, thread: 4 })
        } else {
            let flag = args[1].clone();
            if flag.contains("-h") || (flag.contains("-help") && flag.len() == 2) {
                println!(
                    "Usage: -j to select how many threads you want \r\n -h or -help for more information"
                );
                Err("Error: Invalid argument")
            } else if flag.contains("-h") || flag.contains("-help") {
                Err("Error: There are to many arguments")
            } else if flag.contains("-j") {
                let ip_addr = match IpAddr::from_str(&args[3]) {
                    Ok(ip) => ip,
                    Err(_) => {
                        return Err("Error: Invalid IP address");
                    }
                };
                let thread = match args[2].parse::<u16>() {
                    Ok(td) => td,
                    Err(_) => {
                        return Err("Error: failed to parse thread number");
                    }
                };
                Ok(ArgsFlag { thread, flag, ip_addr })
            } else {
                Err("Error: Invalid argument")
            }
        }
    }
}
fn main() {
    let args: Vec<String> = env::args().collect();
    let program = args[0].clone();
    let args_flag = ArgsFlag::new(&args).unwrap_or_else(|err| {
        if err.contains("help") {
            process::exit(0)
        } else {
            eprintln!("Error: {}", err);
            process::exit(0)
        }
    });

    let thread_number = args_flag.thread;
    let addr = args_flag.ip_addr;
    let (tx, rx) = channel();
    for i in 0..thread_number {
        let tx = tx.clone();
        thread::spawn(move || {
            check(tx, i, addr, thread_number);
        });
    }
    let mut out = vec![];
    drop(tx);
    for p in rx {
        out.push(p);
    }
    println!("");
    out.sort();

    for v in out {
        println!("{} is open", v);
    }
}

fn check(tx: Sender<u16>, start_port: u16, addr: IpAddr, thread_number: u16) {
    let mut port = start_port + 1;
    loop {
        match TcpStream::connect((addr, port)) {
            Ok(_) => {
                println!(".");
                io::stdout().flush().unwrap();
                tx.send(port).unwrap();
            }
            Err(_) => {}
        }
        if MAX - port <= thread_number {
            break;
        }
        port += thread_number;
    }
}
