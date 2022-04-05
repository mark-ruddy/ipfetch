use clap::Parser;
use reqwest;
use seeip::utils::GeoInfo;
use std::fs;
use std::io::Write;
use tabled::builder::Builder;

/// Print readable output of the IPs geographical info to the terminal including a country flag matching the IPs location
fn print_geo_info(info: &GeoInfo, raw_github_url: &str) -> Result<(), Box<dyn std::error::Error>> {
    let formatted_url = format!("{}/{}", raw_github_url, info.country.replace(" ", "_"));
    let flag = reqwest::blocking::get(formatted_url)?.text()?;

    let info_table = Builder::default()
        .add_row(["IP", info.ip.as_str()])
        .add_row(["Country", info.country.as_str()])
        .add_row(["Organization", info.organization.as_str()])
        .add_row(["Timezone", info.timezone.as_str()])
        .add_row(["Continent Code", info.continent_code.as_str()])
        .add_row(["Country Code", info.country_code.as_str()])
        .add_row(["Country Code 3", info.country_code3.as_str()])
        .add_row(["Region Code", info.region_code.as_str()])
        .add_row(["Postal Code", info.postal_code.as_str()])
        .add_row(["Area Code", info.area_code.to_string().as_str()])
        .add_row(["DMA Code", info.dma_code.to_string().as_str()])
        .add_row(["Offset", info.offset.to_string().as_str()])
        .add_row(["ASN", info.asn.to_string().as_str()])
        .add_row(["Longitude", info.longitude.to_string().as_str()])
        .add_row(["Latitude", info.latitude.to_string().as_str()])
        .build();

    print!("{}", flag);
    print!("{}", info_table);
    Ok(())
}

/// Write code-friendly output of the IPs geographical info to a file
fn write_geo_info(out: &String, info: &GeoInfo) -> Result<(), Box<dyn std::error::Error>> {
    let data = format!(
        "\
        ip: {}
        country: {}
        organization: {}
        timezone: {}
        continent_code: {}
        country_code: {}
        country_code3: {}
        region_code: {}
        postal_code: {}
        area_code: {}
        dma_code: {}
        offset: {}
        asn: {}
        longitude: {}
        latitude: {}\n",
        info.ip,
        info.country,
        info.organization,
        info.timezone,
        info.continent_code,
        info.country_code,
        info.country_code3,
        info.region_code,
        info.postal_code,
        info.area_code,
        info.dma_code,
        info.offset,
        info.asn,
        info.longitude,
        info.latitude
    );
    let mut out_file = fs::OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(out)?;
    out_file.write_all(data.replace(" ", "").as_bytes())?;
    Ok(())
}

/// Coordinate the writing the the file(if specified) and the printing to terminal
/// Useful so that the --ip and --my-ip flags can all be handled identically by calling handle_info
fn handle_info(info: &GeoInfo, output: &Option<String>) -> Result<(), Box<dyn std::error::Error>> {
    match output {
        Some(ref out) => write_geo_info(&out, &info)?,
        None => (),
    }
    print_geo_info(
        &info,
        "https://raw.githubusercontent.com/mark-ruddy/ipfetch/main/country_flags",
    )?;
    Ok(())
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// An IP to fetch
    #[clap(short, long)]
    ip: Option<Vec<String>>,
    /// Name of output file to store code-friendly fetched IP data
    #[clap(short, long)]
    output: Option<String>,

    /// Get info on your own IP
    #[clap(long, takes_value = false)]
    my_ip: bool,
    /// Get info on your own IPv4
    #[clap(long, takes_value = false)]
    my_ipv4: bool,
    /// Get info on your own IPv6
    #[clap(long, takes_value = false)]
    my_ipv6: bool,
}

fn main() {
    let args = Args::parse();

    if args.my_ip {
        match seeip::get_caller_geo() {
            Ok(info) => match handle_info(&info, &args.output) {
                Ok(_) => (),
                Err(e) => eprintln!("Error handling caller IP info: {}", e),
            },
            Err(e) => eprintln!("Error making API call with caller's IP: {}", e),
        }
    }

    if args.my_ipv4 {
        match seeip::get_caller_geo_v4() {
            Ok(info) => match handle_info(&info, &args.output) {
                Ok(_) => (),
                Err(e) => eprintln!("Error handling caller IPv4 info: {}", e),
            },
            Err(e) => eprintln!("Error making API call with caller's IPv4: {}", e),
        }
    }

    if args.my_ipv6 {
        match seeip::get_caller_geo_v6() {
            Ok(info) => match handle_info(&info, &args.output) {
                Ok(_) => (),
                Err(e) => eprintln!("Error handling caller IPv6 info: {}", e),
            },
            Err(e) => eprintln!("Error making API call with caller's IPv6: {}", e),
        }
    }

    // Iterate through each passed --ip flag now
    match args.ip {
        Some(ips) => {
            for ip in ips {
                match seeip::get_geo(ip.as_str()) {
                    Ok(info) => match handle_info(&info, &args.output) {
                        Ok(_) => (),
                        Err(e) => eprintln!("Error handling IP info: {}", e),
                    },
                    Err(e) => eprintln!("Error making API call with IP {}: {}", ip, e),
                }
            }
        }
        None => (),
    }

    match args.output {
        Some(out) => println!("\nOutput written to file: {}", out),
        None => (),
    }
}
