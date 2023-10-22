use futures::{stream::FuturesUnordered, StreamExt};
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::{
    collections::HashSet,
    net::{IpAddr, SocketAddr},
    num::NonZeroU8,
    time::Duration,
};
use tokio::io::{self, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::timeout;

pub const LOWEST_PORT_NUMBER: u16 = 1;
pub const TOP_PORT_NUMBER: u16 = 65535;

#[derive(Debug)]
pub struct Scanner {
    ip: IpAddr,
    batch_size: u16,
    timeout: Duration,
    tries: NonZeroU8,
}

impl Scanner {
    pub fn new(ip: IpAddr) -> Self {
        Self {
            batch_size: 4500,
            timeout: Duration::from_millis(1000),
            tries: NonZeroU8::new(std::cmp::max(1, 1)).unwrap(),
            ip,
        }
    }

    pub async fn run(&self) -> Vec<u16> {
        let mut ports: Vec<u16> = (LOWEST_PORT_NUMBER..=TOP_PORT_NUMBER).collect();
        ports.shuffle(&mut thread_rng());

        let mut open_ports: Vec<u16> = Vec::new();
        let mut ftrs = FuturesUnordered::new();
        let mut errors: HashSet<String> = HashSet::with_capacity(1000);

        for &port in &ports {
            let socket = SocketAddr::new(self.ip, port);
            ftrs.push(self.scan_socket(socket));

            // Check if we've reached the batch size
            if ftrs.len() == self.batch_size as usize {
                while let Some(result) = ftrs.next().await {
                    match result {
                        Ok(socket) => open_ports.push(socket.port()),
                        Err(e) => {
                            let error_string = e.to_string();
                            if errors.len() < 1000 {
                                errors.insert(error_string);
                            }
                        }
                    }
                }
            }
        }

        // Process remaining futures if any
        while let Some(result) = ftrs.next().await {
            match result {
                Ok(socket) => open_ports.push(socket.port()),
                Err(e) => {
                    let error_string = e.to_string();
                    if errors.len() < 1000 {
                        errors.insert(error_string);
                    }
                }
            }
        }

        open_ports
    }

    async fn scan_socket(&self, socket: SocketAddr) -> io::Result<SocketAddr> {
        let tries = self.tries.get();

        for nr_try in 1..=tries {
            match self.connect(socket).await {
                Ok(mut x) => {
                    if let Err(e) = x.shutdown().await {
                        println!("Shutdown stream error {}", &e);
                    }
                    println!("Open {}", socket.to_string());
                    return Ok(socket);
                }
                Err(e) => {
                    let mut error_string = e.to_string();

                    if nr_try == tries {
                        error_string.push(' ');
                        error_string.push_str(&socket.ip().to_string());
                        return Err(io::Error::new(io::ErrorKind::Other, error_string));
                    }
                }
            };
        }
        unreachable!();
    }

    async fn connect(&self, socket: SocketAddr) -> io::Result<TcpStream> {
        let stream = timeout(self.timeout, TcpStream::connect(socket)).await??;
        Ok(stream)
    }
}
