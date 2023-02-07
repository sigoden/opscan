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

Usage: opscan [OPTIONS] <ADDRESSES>...

Arguments:
  <ADDRESSES>...  CIDRs, IPs, or hosts to scan ports

Options:
  -t, --timeout <TIMEOUT>        Maximum time in seconds to scan [default: 3]
  -b, --batch-size <BATCH_SIZE>  Number of parallel port scanning [default: 4000]
  -p, --ports <PORTS>            Ports to be scanned e.g. 80,443,19-26 [default: top1000]
  -h, --help                     Print help
  -V, --version                  Print version
```

Scan a single port of a single host

```
opscan 192.168.8.5 -p 22
```

Scan all ports from 1-65535 by default
```
opscan 192.168.8.5
```

Scan a whole CIDR:
```
opscan 192.168.8.1/24 
```

Scan a domain
```
opscan scanme.nmap.org
```

Scan specific ports:
```
opscan 192.168.8.5 -p 80,443,21-23 
```

Scan top-N ports:
```
opscan -p top100 192.168.8.5
opscan -p top250 192.168.8.5
opscan -p top1000 192.168.8.5
```

Adjust batch size and timeout for faster scans：
```
opscan 192.168.8.5 -b 8000 -t 1
```

## License

Copyright (c) 2022 opscan-developers.

argc is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.