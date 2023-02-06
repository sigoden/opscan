# opscan

[![CI](https://github.com/sigoden/opscan/actions/workflows/ci.yaml/badge.svg)](https://github.com/sigoden/opscan/actions/workflows/ci.yaml)
[![Crates](https://img.shields.io/crates/v/opscan.svg)](https://crates.io/crates/opscan)

A open port scanner.

<!-- ![screenshot]() -->

## Install

### With cargo

```
cargo install --force opscan
```

### Binaries on macOS, Linux, Windows

Download from [Github Releases](https://github.com/sigoden/opscan/releases), unzip and add opscan to your $PATH.

## Usage

```
A open port scanner

Usage: opscan [OPTIONS] [ADDRESSES]...

Arguments:
  [ADDRESSES]...  CIDRs, IPs, or hosts to be scanned

Options:
  -t, --timeout <TIMEOUT>  Milliseconds for waiting connection [default: 1500]
  -b, --batch <BATCH>      The batch size for port scanning, it increases or slows the speed of scanning [default: 4500]
  -p, --ports <PORTS>      A list of comma separed ports to be scanned e.g. 80,443,19-26 [default: top100]
  -h, --help               Print help
  -V, --version            Print version
```

Scan top 100 ports:
```
opscan 192.168.1.5
```

Scan a whole CIDR:
```
opscan 192.168.1.1/24 
```

Scan a domain
```
opscan www.bing.com
```

Scan specific ports:
```
opscan -p 80,443,21-23 192.168.1.5
```

Scan top-N ports:
```
opscan -p top10 192.168.1.5
opscan -p top50 192.168.1.5
opscan -p top1000 192.168.1.5
```

Scan all ports from 1-65535:
```
opscan -p full 192.168.1.5
```

Adjust batch size and timeout for faster scans：
```
opscan -p full 192.168.1.5 -b 65535 -t 1000
```

## License

Copyright (c) 2022 opscan-developers.

argc is made available under the terms of either the MIT License or the Apache License 2.0, at your option.

See the LICENSE-APACHE and LICENSE-MIT files for license details.