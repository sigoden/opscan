# opscan

[![CI](https://github.com/sigoden/opscan/actions/workflows/ci.yaml/badge.svg)](https://github.com/sigoden/opscan/actions/workflows/ci.yaml)
[![Crates](https://img.shields.io/crates/v/opscan.svg)](https://crates.io/crates/opscan)
[![Docker Pulls](https://img.shields.io/docker/pulls/sigoden/opscan)](https://hub.docker.com/r/sigoden/opscan)

A open port scanner.

![screenshot](https://user-images.githubusercontent.com/4012553/217132939-42a8f375-fe66-4210-aacd-c02650a460f2.png)

## Install

### With cargo

```
cargo install --force opscan
```

### With docker

```
docker run --rm -it sigoden/opscan opscan.nmap.org
```

### Binaries on macOS, Linux, Windows

Download from [Github Releases](https://github.com/sigoden/opscan/releases), unzip and add opscan to your $PATH.


## Usage

```
A open port scanner

Usage: opscan [OPTIONS] [ADDRESSES]...

Arguments:
  [ADDRESSES]...  CIDRs, IPs, or hosts to scan ports

Options:
  -p, --ports <PORTS>              Ports to be scanned e.g. 22,80-443,top100
  -t, --timeout <TIMEOUT>          Maximum time in milliseconds to scan
  -c, --concurrency <CONCURRENCY>  Number of concurrent port scanning
  -h, --help                       Print help
  -V, --version                    Print version
```

Check if a port is open:
```
opscan scanme.nmap.org -p 80
```

Scan top-N ports on a domain:
```
opscan scanme.nmap.org -p top100 
opscan scanme.nmap.org -p top666
opscan scanme.nmap.org -p top1000
```

List open ports on localhost:
```
opscan                             # equal to `opscan 127.0.0.1 -p 1-65535`
```

Scan specific ports:
```
opscan 192.168.8.5 -p 80,443,3000-6000
opscan 192.168.8.5 -p 1-65535
```

Scan specific CIDRs:
```
opscan -p 22 192.168.8
opscan -p 22 192.168.8.0/24
opscan -p 22 192.168.8.0/192.168.255.255
```

Default scan ports and timeouts on private/non-dedicated networks:
```
opscan 127.0.0.1                   # ports: 1-65535, timeout: 1000
opscan scanme.nmap.org             # ports: top1000, timeout: 3000
```

Increase concurrency and decrease timeout for faster scans:
```
opscan scanme.nmap.org -t 1500 -c 8000
```

## License

Copyright (c) 2022 opscan-developers.

argc is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.