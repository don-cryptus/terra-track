use crate::types::{Nmaprun, Ports};
use quick_xml;
use quickxml_to_serde::{xml_string_to_json, Config};
use serde_xml_rs::from_str;
use std::fs::{create_dir_all, File};
use std::io::{Read, Write};
use std::net::IpAddr;
use std::path::Path;
use std::process::Command;

pub struct Script {
    ip: IpAddr,
    open_ports: Vec<u16>,
    xml: String,
}

impl Script {
    pub fn new(ip: IpAddr, open_ports: Vec<u16>) -> Self {
        Self {
            ip,
            open_ports,
            xml: format!("./{}.xml", ip.to_string()),
        }
    }

    pub fn run(self) -> anyhow::Result<Nmaprun> {
        // Convert ports to string and join with commas
        let ports_str = self
            .open_ports
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join(",");

        // Construct the list of arguments with the IP and ports

        let binding = self.ip.to_string();
        let arguments = vec![
            "nmap",
            "-vvvvvv",
            "-T2",
            "-n",
            "-sV",
            "-Pn",
            "-sC",
            "-oX",
            self.xml.as_str(),
            "-p",
            &ports_str,
            binding.as_str(),
        ];

        let script = self.execute_script(arguments);
        match script {
            Ok(_x) => self.parse_nmap_xml(),
            Err(_) => Err(anyhow::anyhow!("Script failed")),
        }
    }

    // Separate method to create directory based on the XML path
    fn create_directory(&self) {
        if let Some(parent) = Path::new(&self.xml).parent() {
            if !parent.exists() {
                create_dir_all(parent).expect("Failed to create directory");
            }
        }
    }

    fn execute_script(&self, arguments: Vec<&str>) -> anyhow::Result<String> {
        // The first argument is always the command
        let command = arguments[0];

        // Execute the command with the given arguments
        let process = Command::new(command).args(&arguments[1..]).output()?;

        // Check if the process was successful and return the output or an error
        if process.status.success() {
            Ok(String::from_utf8_lossy(&process.stdout).into_owned())
        } else {
            Err(anyhow::anyhow!(
                "Exit code = {}",
                process.status.code().unwrap_or(-1)
            ))
        }
    }

    pub fn parse_nmap_xml(&self) -> anyhow::Result<Nmaprun> {
        self.create_directory();
        let mut file = File::open(self.xml.clone())?;
        let mut contents = String::new();

        file.read_to_string(&mut contents)?;
        // let nmap: Ports = quick_xml::de::from_str(&contents).unwrap();
        // convert the XML string into JSON with default config params
        let json = xml_string_to_json(
            contents.clone(),
            &Config {
                xml_attr_prefix: "".to_string(),
                xml_text_node_prop_name: "value".to_string(),
                ..Default::default()
            },
        )
        .unwrap()
        .to_string();

        File::create(self.xml.clone().replace(".xml", ".json"))
            .unwrap()
            .write_all(json.as_bytes())
            .unwrap();

        let nmap: Nmaprun = serde_json::from_str(json.as_str())?;
        Ok(nmap)
    }
}
