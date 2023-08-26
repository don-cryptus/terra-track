use itertools::{iproduct, Product};
use std::net::{IpAddr, SocketAddr};

pub struct SocketIterator<'s> {
    // product_it is a cartesian product iterator over
    // the slices of ports and IP addresses.
    //
    // The IP/port order is intentionally reversed here since we want
    // the itertools::iproduct! macro below to generate the pairs with
    // all the IPs for one port before moving on to the next one
    // ("hold the port, go through all the IPs, then advance the port...").
    // See also the comments in the iterator implementation for an example.
    product_it:
        Product<Box<std::slice::Iter<'s, u16>>, Box<std::slice::Iter<'s, std::net::IpAddr>>>,
}

/// An iterator that receives a slice of IPs and ports and returns a Socket
/// for each IP and port pair until all of these combinations are exhausted.
/// The goal of this iterator is to go over every IP and port combination
/// without generating a big memory footprint. The alternative would be
/// generating a vector containing all these combinations.
impl<'s> SocketIterator<'s> {
    pub fn new(ips: &'s [IpAddr], ports: &'s [u16]) -> Self {
        let ports_it = Box::new(ports.iter());
        let ips_it = Box::new(ips.iter());
        Self {
            product_it: iproduct!(ports_it, ips_it),
        }
    }
}

impl<'s> Iterator for SocketIterator<'s> {
    type Item = SocketAddr;

    /// Returns a socket based on the combination of one of the provided
    /// IPs and ports or None when these combinations are exhausted. Every
    /// IP will have the same port until a port is incremented.
    ///
    /// let it = SocketIterator::new(&["127.0.0.1", "192.168.0.1"], &[80, 443]);
    /// it.next(); // 127.0.0.1:80
    /// it.next(); // 192.168.0.1:80
    /// it.next(); // 127.0.0.1:443
    /// it.next(); // 192.168.0.1:443
    /// it.next(); // None
    fn next(&mut self) -> Option<Self::Item> {
        self.product_it
            .next()
            .map(|(port, ip)| SocketAddr::new(*ip, *port))
    }
}
