# ipfetch
CLI Tool for easy IP information fetching, inspired by https://github.com/trakBan/ipfetch.

This Rust implementation aims to provide stability and extra features, if you are interested in using the underlying Rust IP API library see https://github.com/mark-ruddy/seeip.

Asciinema example: https://asciinema.org/a/DJWnX7EL0MpBeL21yEUKlAqPo  

[![ipfetch.png](https://i.postimg.cc/ry9QkWJ8/ipfetch.png)](https://postimg.cc/Z0CrPW2g)

## Installing
### Cargo
Install the binary using cargo:
`cargo install ipfetch`

### Compile
Clone the source code from github and compile yourself:
```
git clone https://github.com/mark-ruddy/ipfetch
cd ipfetch && cargo build --release
sudo mv target/release/ipfetch /usr/local/bin
```

## Examples
Run `ipfetch --help` to see available flags.  

Get info on your own IP:
```
ipfetch --my-ip
```

Some systems can have both an IPv4 and an IPv6 assigned, to specifically check both:
```
ipfetch --my-ipv4 --my-ipv6
```

Get info on an IP:
```
ipfetch --ip 208.67.222.222
```

Get info on multiple IPs, IPv6 is supported:
```
ipfetch --ip 208.67.222.222 --ip 2620:0:ccc::2
```

Print IP info to terminal for the Google and Cloudflare DNS servers, and save this info to a file:
```
ipfetch --ip 8.8.8.8 --ip 1.1.1.1 --output dns-servers-info
```

Combine the flags to produce a report on your IPv6 and others in one command:
```
ipfetch --my-ipv6 --ip 8.8.8.8 -ip 208.67.222.222 --output ip-info-data
```

## Contributions
Any and all contributions are appreciated - completely new features, bug fixes etc. Ensure your code is formatted with `rustfmt`.  
