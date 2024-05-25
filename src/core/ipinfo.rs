use std::{
    error::Error,

    net::{
        IpAddr,
        ToSocketAddrs, 
    }
};

use ipgeolocate::{
    Locator,
    Service,
};

use whois_rust::{
    WhoIs,
    WhoIsLookupOptions,
};

use crate::core::download::Download;

pub struct IPInfo;

impl IPInfo {

    fn get_ip_address(hostname: &str) -> Result<String, Box<dyn Error>> {
        let addr = format!("{}:0", hostname);
        let mut addrs_iter = addr.to_socket_addrs()?;
        
        if let Some(socket_addr) = addrs_iter.next() {
            if let IpAddr::V4(ipv4_addr) = socket_addr.ip() {
                Ok(ipv4_addr.to_string())
            } else {
                Err("IPv6 address not supported".into())
            }
        } else {
            Err("Failed to resolve IP address".into())
        }
    }

    pub async fn get_info(args: Vec<String>) -> Result<(), Box<dyn Error>> {
        let cwd = &args[0];
        let hostname = &args[1];

        if args.len() != 2 {
            eprintln!("Usage: {} <hostname>", cwd);
            std::process::exit(1);
        }

        let service = Service::IpApi;
        let servers_file = Download::download(cwd).await?;
        let whois = WhoIs::from_path(servers_file).unwrap();

        match Self::get_ip_address(hostname) {
            Ok(ip_domain) => {
                match Locator::get(&ip_domain, service).await {
                    Ok(ip) => {
                        println!("Domain: {}", hostname);
                        println!("IP: {}", &ip_domain);
                        println!("Region: {} - {} ({})", ip.city, ip.region, ip.country);
                        println!("ISP: {}", ip.isp);
                        println!("Timezone: {}", ip.timezone);

                        let result = whois.lookup(
                            WhoIsLookupOptions::from_string(hostname).unwrap()
                        ).unwrap();

                        println!("Whois: \n{}", result);
                    },

                    Err(error) => println!("Error: {}", error),
                };       
            },

            Err(e) => eprintln!("Failed to resolve IP address: {}", e),
        }

        Ok(())
    }

}
