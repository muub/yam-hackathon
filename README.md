yam-hackathon
=====================
Rustful Log Parsing
---------------------

You will need to install `rustup` to use rust-nightly
https://rustup.rs
`curl https://sh.rustup.rs -sSf | sh`


Instructions for Building and running `smelt`
```
cd smelt
cargo build --release
./target/release/smelt "2019-01-08" /Users/matthew.boatman/Desktop/loggly-2019-01-08/api-members-cart.raw out/lat_long_member_cart_2019_01_08.csv
```

The format is `smelt "Date String" input_file_path output_file_path`
