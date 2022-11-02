#![allow(unused_doc_comments)]
#![allow(non_snake_case)]
#![allow(unused_must_use)]
extern crate clap;

use clap::{Arg, App, ArgMatches};
use colored::Colorize;
use reqwest;
use reqwest::Error;
use reqwest::header::CONTENT_TYPE;
use serde::Deserialize;

/// Properties:
///
/// * `status`: Status of the request.
/// * `continent`: The name of the continent.
/// * `continentCode`: The two-letter continent code for the IP address.
/// * `country`: The country name.
/// * `countryCode`: The two-letter country code for the country associated with the IP address.
/// * `region`: The region name
/// * `regionName`: The name of the region
/// * `city`: City
/// * `district`: The district of the city.
/// * `zip`: The zip code of the IP address.
/// * `lat`: Latitude
/// * `lon`: longitude
/// * `timezone`: The timezone of the IP address.
/// * `currency`: The currency code for the country.
/// * `isp`: Internet Service Provider
/// * `org`: The name of the organization that owns the IP address.
/// * `asname`: The autonomous system number of the ISP.
/// * `reverse`: The reverse DNS of the IP address.
/// * `mobile`: Whether the IP is from a mobile device
/// * `proxy`: Whether the IP is a proxy or not.
/// * `hosting`: Whether the IP is a hosting provider
/// * `query`: The IP address that was looked up.
#[derive(Deserialize)]
struct IpInfo {
    status: String,
    continent: String,
    continentCode: String,
    country: String,
    countryCode: String,
    region: String,
    regionName: String,
    city: String,
    district: String,
    zip: String,
    lat: f64,
    lon: f64,
    timezone: String,
    currency: String,
    isp: String,
    org: String,
    asname: String,
    reverse: String,
    mobile: bool,
    proxy: bool,
    hosting: bool,
    query: String
}

/// It prints a banner to the terminal
fn print_banner() {
    println!("\n\n\n\n\n\n\n\n\n\n\n");
    println!("{}", format!("________              _____ ").green());
    println!("{}", format!("___  __ \\___  __________  /_").green());
    println!("{}", format!("__  /_/ /  / / /_  ___/  __/").green());
    println!("{}", format!("_  _, _// /_/ /_(__  )/ /_  ").green());
    println!("{}", format!("/_/ |_| \\__,_/ /____/ \\__/").green());
    println!("{}", format!("   ____   ___        _____             __                  __    _            ").bold().bright_blue());
    println!("{}", format!("  /  _/  / _ \\      / ___/ ___  ___   / / ___  ____ ___ _ / /_  (_) ___   ___ ").bold().bright_blue());
    println!("{}", format!(" _/ /   / ___/     / (_ / / -_)/ _ \\ / / / _ \\/ __// _ `// __/ / / / _ \\ / _ \\").bold().bright_blue());
    println!("{}", format!("/___/  /_/         \\___/  \\__/ \\___//_/  \\___/\\__/ \\_,_/ \\__/ /_/  \\___//_//_/\n").bold().bright_blue());
    println!("{}", format!("=====================").bold().bright_blue());
    println!("{}", format!(" Rust IP Geolocation").green());
    println!("{}", format!("=====================").bold().bright_blue());
    println!("{} {}", format!("HELP:  ").yellow().bold() ,format!("ip_geolocation -h or --help").purple().italic());
    println!("{} {}", format!("USAGE: ").yellow().bold() ,format!("ip_geolocation -i 216.58.216.238\n\n").purple().italic());
}

/// It prints the IP information to the console
///
/// Arguments:
///
/// * `info`: IpInfo - This is the struct that contains all the information about the IP address.
fn print_ip_information(info: IpInfo) {
    println!("{}", format!("{}", "IP ADDRESS INFO").green());
    println!("{}", format!("{}", "______________________________________________________").green());
    println!("{0: <15} {1: <7}", format!("{}", "STATUS:").green(), format!("{}", info.status).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "CONTINENT:").green(), format!("{}", info.continent).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "CONTINENT CODE:").green(), format!("{}", info.continentCode).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "COUNTRY:").green(), format!("{}", info.country).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "COUNTRY CODE:").green(), format!("{}", info.countryCode).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "REGION:").green(), format!("{}", info.region).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "REGION NAME:").green(), format!("{}", info.regionName).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "CITY:").green(), format!("{}", info.city).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "DISTRICT:").green(), format!("{}", info.district).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "ZIP:").green(), format!("{}", info.zip).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "LATITUDE:").green(), format!("{}", info.lat).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "LONGITUDE:").green(), format!("{}", info.lon).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "TIMEZONE:").green(), format!("{}", info.timezone).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "CURRENCY:").green(), format!("{}", info.currency).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "ISP:").green(), format!("{}", info.isp).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "ORGANIZATION:").green(), format!("{}", info.org).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "ASNAME:").green(), format!("{}", info.asname).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "REVERSE:").green(), format!("{}", info.reverse).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "MOBILE:").green(), format!("{}", info.mobile).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "PROXY:").green(), format!("{}", info.proxy).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "HOSTING:").green(), format!("{}", info.hosting).bright_blue());
    println!("{0: <15} {1: <7}", format!("{}", "QUERY:").green(), format!("{}", info.query).bright_blue());
    println!("{}", format!("{}", "______________________________________________________").green());
}

/// It creates a new instance of the App struct, which is provided by the clap crate.
fn app_structure() {
    let matches = App::new("Rust IP Geolocation")
        .version("1.0")
        .author("Domagoj Ratko")
        .about("Simple IP geolocation scanner.")
        .arg(
            Arg::with_name("ip")
                .short("i".parse().unwrap())
                .long("ip")
                .takes_value(true)
                .help("The target IP Address to scan")
        )
        .get_matches();

    app_logic(matches);
}

/// It takes the command line arguments, creates a URL, makes a request to the URL, parses the response,
/// and prints the response
///
/// Arguments:
///
/// * `matches`: ArgMatches - This is the struct that holds the command line arguments.
///
/// Returns:
///
/// A Result<(), Error>
#[tokio::main]
async fn app_logic(matches: ArgMatches) -> Result<(), Error> {
    let ip: &str = matches.value_of("ip").unwrap_or("216.58.216.238");
    let url: String = format!("http://ip-api.com/json/{}?fields=\
    status,\
    continent,\
    continentCode,\
    country,\
    countryCode,\
    region,\
    regionName,\
    city,district,\
    zip,\
    lat,\
    lon,\
    timezone,\
    currency,\
    isp,\
    org,\
    asname,\
    reverse,\
    mobile,\
    proxy,\
    hosting,\
    query", ip);

    let client = reqwest::Client::new();

    let info = client.get(url)
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?
        .json::<IpInfo>()
        .await?;

    print_ip_information(info);

    Ok(())
}

/// `main()` is the entry point of the application
fn main() {
    print_banner();
    app_structure();
}

