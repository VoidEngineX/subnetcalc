# subnetcalc

A fast, native Rust IPv4 subnet calculator CLI tool, similar to `ipcalc` with extra subnet generation features.

## Features
- Calculate subnets from any CIDR (e.g. /8 → /16 → /24 → /26)
- Show total addresses and usable hosts
- Normalizes host bits automatically

## Usage
```bash
subnetcalc 10.1.0.0/24 -s 26
subnetcalc 192.168.1.0/24