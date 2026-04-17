// This is a complete, ready-to-use Linux command-line tool called `subnetcalc`
// Fast native binary
// Usage examples:
//   subnetcalc 10.1.0.0/24 -s 26
//   subnetcalc 10.0.0.0/8 -s 16
//   subnetcalc 10.1.0.0/24          (just shows network info like total/usable hosts)
//   subnetcalc 192.168.1.100/24 -s 26   (automatically normalizes to network address)

use clap::Parser;
use std::net::Ipv4Addr;
use std::str::FromStr;

#[derive(Parser, Debug)]
#[command(author, version, about = "Fast IPv4 subnet calculator (like ipcalc + Python ipaddress.subnets)", long_about = None)]
struct Args {
    /// Base network in CIDR format (e.g. 10.1.0.0/24 or 192.168.1.100/24)
    #[arg(required = true)]
    network: String,

    /// New prefix length to create subnets (e.g. 26). If omitted, just shows network info.
    #[arg(short = 's', long = "subnet-prefix", value_parser = clap::value_parser!(u8))]
    new_prefix: Option<u8>,
}

fn main() {
    let args = Args::parse();

    // Parse CIDR input
    let parts: Vec<&str> = args.network.split('/').collect();
    if parts.len() != 2 {
        eprintln!("Error: Invalid CIDR format. Use <IP>/<prefix> (example: 10.1.0.0/24)");
        std::process::exit(1);
    }

    let ip_str = parts[0];
    let base_prefix: u8 = match parts[1].parse() {
        Ok(p) if p <= 32 => p,
        _ => {
            eprintln!("Error: Invalid prefix (must be 0-32)");
            std::process::exit(1);
        }
    };

    let base_ip = match Ipv4Addr::from_str(ip_str) {
        Ok(ip) => ip,
        Err(_) => {
            eprintln!("Error: Invalid IPv4 address");
            std::process::exit(1);
        }
    };

    // Convert to u32 for easy bit math
    let base_u32 = u32::from(base_ip);

    // Calculate network address (normalize any host bits to 0)
    let mask: u32 = !0u32 << (32 - base_prefix);
    let network_u32 = base_u32 & mask;
    let network_ip = Ipv4Addr::from(network_u32);

    if let Some(new_prefix) = args.new_prefix {
        if new_prefix <= base_prefix {
            eprintln!(
                "Error: New prefix must be larger than base prefix (you are making smaller subnets)"
            );
            std::process::exit(1);
        }

        let bits_borrowed = new_prefix - base_prefix;
        let num_subnets = 1u32 << bits_borrowed;
        let addresses_per_subnet = 1u32 << (32 - new_prefix);

        println!(
            "Subnets of {}/{} into /{} ({} subnets created):",
            network_ip, base_prefix, new_prefix, num_subnets
        );

        let subnet_size = addresses_per_subnet;
        for i in 0..num_subnets {
            let subnet_start = network_u32 + (i * subnet_size);
            let subnet_ip = Ipv4Addr::from(subnet_start);

            let usable_hosts = addresses_per_subnet.saturating_sub(2);

            println!(
                "  {}/{} → {} total addresses, {} usable hosts",
                subnet_ip, new_prefix, addresses_per_subnet, usable_hosts
            );
        }
    } else {
        // Just show info about the base network (ipcalc-style)
        let total_addresses = 1u32 << (32 - base_prefix);
        let usable_hosts = total_addresses.saturating_sub(2);

        let broadcast_u32 = network_u32 | !mask;
        let broadcast_ip = Ipv4Addr::from(broadcast_u32);

        println!("Network:          {}/{}", network_ip, base_prefix);
        println!("Subnet mask:      {}", Ipv4Addr::from(mask));
        println!("Broadcast:        {}", broadcast_ip);
        println!("Total addresses:  {}", total_addresses);
        println!("Usable hosts:     {}", usable_hosts);
    }
}
