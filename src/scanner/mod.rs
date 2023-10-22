use async_std::io;
use async_std::net::TcpStream;
use async_std::prelude::*;
use futures::stream::FuturesUnordered;
use std::{
    collections::HashSet,
    net::{IpAddr, Shutdown, SocketAddr},
    num::NonZeroU8,
    time::Duration,
};

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
        Self::fisher_yates_shuffle(&mut ports);

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

    fn fisher_yates_shuffle<T>(items: &mut [T]) {
        let mut i = items.len();
        while i > 1 {
            i -= 1;
            let j = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_micros() as usize)
                % (i + 1);
            items.swap(i, j);
        }
    }

    async fn scan_socket(&self, socket: SocketAddr) -> io::Result<SocketAddr> {
        let tries = self.tries.get();

        for nr_try in 1..=tries {
            match self.connect(socket).await {
                Ok(x) => {
                    if let Err(e) = x.shutdown(Shutdown::Both) {
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
        let stream = io::timeout(
            self.timeout,
            async move { TcpStream::connect(socket).await },
        )
        .await?;
        Ok(stream)
    }
}
