use clap::Parser;
use seeip;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The IP used to fetch
    #[clap(short, long, default_value = "")]
    ip: String,
    /// List of IPs to fetch
    #[clap(long, default_value = "")]
    ips: Vec<String>,
}

fn handle_ips(ips: Vec<&str>) {
    for ip in ips {
        println!("in handle_ips: {}", ip);
    }
}

fn main() {
    let args = Args::parse();

    // check if list of IPs is passed
    handle_ips(ip_vec);

    // execution will reach here only if no list of IPs is passed
    let ip = args.ip;
    if ip == "" {}
    println!("one IP: {}", ip);
}
