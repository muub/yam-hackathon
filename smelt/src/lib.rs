
use std::fs;
use std::error::Error;
use std::env;

use std::collections::HashMap;
use std::hash::{Hash, Hasher};

use std::net::IpAddr;
use std::str::FromStr;

use std::io::prelude::*;
use std::fs::File;
use std::path::Path;

use maxminddb::geoip2;
use maxminddb::MaxMindDBError;

use bigdecimal::{BigDecimal, FromPrimitive};

use regex::Regex;

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let path = Path::new("out/lat_long_counts.csv");
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
       Err(why) => panic!("couldn't create {}: {}",
                          display,
                          why.description()),
       Ok(file) => file,
    };


    // let results = if config.case_sensitive {
    //     search(&config.query, &contents)
    // } else {
    //     search_case_insensitive(&config.query, &contents)
    // };

    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // let re = Regex::new(r"/X-Forwarded-For.*\b=(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})/gm").unwrap();

    let re = Regex::new(r"X-Forwarded-For=(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();

    let reader = maxminddb::Reader::open_readfile("/usr/local/share/GeoIP/GeoLite2-City.mmdb").unwrap();


    // let re = Regex::new(r"X-Forwarded-For=\d*").unwrap();

    // lookup("67.181.158.177");

    // let mut location_counter: Vec<LatLongLine> = Vec::new();
    // let mut locations = &[5.02,-30.1, 5 ];

    // let mut loc_map: HashMap<_, i64> = HashMap::new();
    // let mut geo_thing: HashMap<Vec<f32>, i32> = HashMap::new();

    let mut lat_long_hash: HashMap<String, u32> = HashMap::new();

    // println!("With text: \n{}", contents);
    for line in contents.lines() {
        // println!("{}", line)

        for cap in re.captures_iter(line) {
            let xforward_str = &cap[0];
            let ip: Vec<&str> = xforward_str.split("=").collect();

            // println!("IP Address: {:?}", &ip[1]);
            let lat_long_opt = lookup(&ip[1], &reader);

            match lat_long_opt {
                Some(lat_long) => {

                    // let lat_dec = BigDecimal::from_f64(lat_long.latitude.unwrap());
                    // let long_dec = BigDecimal::from_f64(lat_long.longitude.unwrap());
                    // println!("{:?}, Latitude: {}, Longitude: {}", lat_long, lat_long.latitude.unwrap(), lat_long.longitude.unwrap())

                    // let thing = lat_dec.unwrap_or_default(0);

                    let rounded_lat: f64 = round_decimal(lat_long.latitude.unwrap());
                    let rounded_long: f64 = round_decimal(lat_long.longitude.unwrap());

                    // let temp_lat_long = LatLong{lat: rounded_lat.to_string(), long: rounded_long.to_string()};

                    let temp_lat_long = format!("{}|{}", rounded_lat, rounded_long);

                    // let count = match lat_long_hash.get(&temp_lat_long) {
                    //     Some(c) => c + 1,
                    //     None => 1;
                    //
                    // };
                    //
                    // lat_long_hash.entry(temp_lat_long).or_insert(count);

                    // lat_long_hash.get(temp_lat_long).set(count + 1);


                    let mut count = lat_long_hash.entry(temp_lat_long).or_insert(0);
                     *count += 1;

                    // println!("{:?}, Latitude: {}, Longitude: {}, Count: {}", lat_long, rounded_lat, rounded_long, count)
                },

                None => eprintln!("ip not found")
            }

        }
        // println!("{}", loc);

    }

    let mut output: String = String::new();

    println!("Writing out file lat_long_counts.csv");
    for ln in &lat_long_hash {
        output += &format!("{}|{} \n", ln.0, ln.1.to_string());
    }


    // // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    match file.write_all(output.as_bytes()) {
       Err(why) => {
           panic!("couldn't write to {}: {}", display,
                                              why.description())
       },
       Ok(_) => println!("successfully wrote to {}", display),
    }



    Ok(())
}

// #[derive(PartialEq, Eq, Hash)]
struct LatLong {
    lat: String,
    long: String
}

// struct LatLong(str, f64);


// impl Hash for LatLong {
//     fn hash<H>(&self, state: &mut H) where H: Hasher {
//         state.write_u16(4);
//     }
// }

pub enum CoordData {
    Longitude(f32),
    Latitude(f32),
    Total(i32)
}

pub struct LatLongLine {
    pub lat: f32,
    pub long: f32,
    pub cnt: u32
}

fn round_decimal(x: f64) -> f64 {
    return (x * 100.0).round() / 100.0
}


pub struct Config {
    pub query: String,
    pub filename: String,
    pub case_sensitive: bool,
}

impl Config {
    pub fn new (args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 3 {
            return Err("not enough arguments");
        }

        let query = args[1].clone();
        let filename = args[2].clone();

        let case_sensitive = env::var("CASE_INSENSITIVE").is_err();

        Ok(Config { query, filename, case_sensitive })
    }
}


fn lookup<'a>(ip_query: &str, reader: &maxminddb::Reader<Vec<u8>>) -> Option<maxminddb::geoip2::model::Location> {
   // let reader = maxminddb::Reader::open_readfile("/usr/local/share/GeoIP/GeoLite2-City.mmdb").unwrap();
   // let ip: IpAddr = FromStr::from_str("89.160.20.128").unwrap();
   // let ip: IpAddr = FromStr::from_str("67.181.158.177").unwrap();

   let ip: IpAddr = FromStr::from_str(ip_query).unwrap();

   let lookup_result: Result<geoip2::City, MaxMindDBError> = reader.lookup(ip);

   match lookup_result {
       Ok(city) => city.location,
       Err(err) => {
           eprintln!("{}", err);
           None
       }
   }


   // let ip = ip_result.unwrap_or_else(|err| {
        // return None;
   // });

   // let city: geoip2::City = reader.lookup(ip.unwrap()).unwrap();

   // return city.location;


   // let location: geoip2::model::Location = city.location.unwrap();
   // println!("{:?}", location);

   // println!("{:?}", city);

   // return city.location;
}

fn search<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

fn search_case_insensitive<'a>(query: &str, contents: &'a str) -> Vec<&'a str> {
    let query = query.to_lowercase();
    let mut results = Vec::new();

    for line in contents.lines() {
        if line.to_lowercase().contains(&query) {
            results.push(line);
        }
    }

    results
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case_sensitive() {
        let query = "duct";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Duct tape.";

        assert_eq!(
            vec!["safe, fast, productive."],
            search(query, contents)
        );
    }

    #[test]
    fn case_insensitive() {
        let query = "rUst";
        let contents = "\
Rust:
safe, fast, productive.
Pick three.
Trust me.";

        assert_eq!(
            vec!["Rust:", "Trust me."],
            search_case_insensitive(query, contents)
        )
    }
}
