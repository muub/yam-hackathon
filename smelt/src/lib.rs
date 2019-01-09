
use std::fs;
use std::error::Error;
use std::env;

use std::net::IpAddr;
use std::str::FromStr;

use maxminddb::geoip2;
use maxminddb::MaxMindDBError;

use regex::Regex;

pub fn run (config: Config) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(config.filename)?;

    let results = if config.case_sensitive {
        search(&config.query, &contents)
    } else {
        search_case_insensitive(&config.query, &contents)
    };

    // let re = Regex::new(r"^\d{4}-\d{2}-\d{2}$").unwrap();
    // let re = Regex::new(r"/X-Forwarded-For.*\b=(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})/gm").unwrap();

    let re = Regex::new(r"X-Forwarded-For=(\d{1,3}\.\d{1,3}\.\d{1,3}\.\d{1,3})").unwrap();

    // let re = Regex::new(r"X-Forwarded-For=\d*").unwrap();

    // lookup("67.181.158.177");

    // println!("With text: \n{}", contents);
    for line in results {
        // println!("{}", line)

        for cap in re.captures_iter(line) {
            let xforward_str = &cap[0];
            let ip: Vec<&str> = xforward_str.split("=").collect();

            println!("IP Address: {:?}", &ip[1]);
            let lat_long_opt = lookup(&ip[1]);

            match lat_long_opt {
                Some(lat_long) => println!("{:?}", lat_long ),
                None => eprintln!("ip not found")
            }

        }
        // println!("{}", loc);

    }

    Ok(())
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


fn lookup<'a>(ip_query: &str) -> Option<maxminddb::geoip2::model::Location> {
   let reader = maxminddb::Reader::open_readfile("/usr/local/share/GeoIP/GeoLite2-City.mmdb").unwrap();
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
