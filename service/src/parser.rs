use crate::entity::ip_main::ip_main_m;
use crate::entity::ip_service::ip_service_m;
use crate::entity::ip_service_script::ip_service_script_m;
use crate::mapper::ip_host_script_mapper;
use crate::mapper::ip_service_mapper::process_service;
use crate::mapper::ip_service_script_mapper::process_service_scripts;
use crate::printlog;
use scanner::types::Nmap;

pub const BATCH_SIZE: i32 = 1;

pub async fn parse_nmap_results(nmap: &Nmap) -> anyhow::Result<()> {
    printlog!("Parsing nmap results Start");
    let host = &nmap.nmaprun.host;
    let ip = &host.address.addr;
    let ports = &host.ports.port;

    let ip_main = ip_main_m::Mutation::upsert_ip_main_by_ip(ip, &host.address.addrtype).await?;

    let mut services_to_create = Vec::new();
    for port in ports {
        services_to_create.push(process_service(ip_main.id, &port));
    }

    let created_services =
        ip_service_m::Mutation::create_many_ip_services(services_to_create).await?;

    if let Some(host_script) = &host.hostscript {
        ip_host_script_mapper::process_host_script(ip_main.id, host_script).await?;
    }

    let mut script_models = Vec::new();
    for (created_service, port) in created_services.iter().zip(ports.iter()) {
        if let Some(script) = &port.script {
            script_models.extend(process_service_scripts(
                ip_main.id,
                created_service.id,
                script,
            ));
        }
    }

    ip_service_script_m::Mutation::create_many_ip_service_scripts(script_models).await?;

    printlog!("Parsing nmap results End");
    Ok(())
}
