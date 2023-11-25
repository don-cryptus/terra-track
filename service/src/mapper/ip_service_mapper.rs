use crate::utils::parse_os_from_nmap_output;
use entity::ip_service;
use scanner::types::Port;
use sea_orm::Set;

pub fn process_service(ip_main_id: i64, port: &Port) -> ip_service::ActiveModel {
    let service = &port.service;
    let mut cpuarch = None;
    let mut ostype = port.service.ostype.clone();
    if let Some(servicefp) = &service.servicefp {
        let (os_type, cpu_arch) = parse_os_from_nmap_output(servicefp.clone());
        if os_type.is_some() && ostype.is_none() {
            ostype = os_type;
        }
        cpuarch = cpu_arch;
    }
    ip_service::ActiveModel {
        ip_main_id: Set(ip_main_id),
        protocol: Set(port.protocol.clone()),
        port: Set(port.portid),
        name: Set(service.name.clone()),
        conf: Set(service.conf),
        version: Set(service.version.clone()),
        product: Set(service.product.clone()),
        extra_info: Set(service.extrainfo.clone()),
        tunnel: Set(service.tunnel.clone()),
        proto: Set(service.proto.clone()),
        rpcnum: Set(service.rpcnum.clone()),
        lowver: Set(service.lowver.clone()),
        highver: Set(service.highver.clone()),
        hostname: Set(service.hostname.clone()),
        method: Set(service.method.clone()),
        os_type: Set(ostype),
        cpu_arch: Set(cpuarch),
        device_type: Set(service.devicetype.clone()),
        service_fp: Set(service.servicefp.clone()),
        cpe: Set(service.cpe.clone()),
        ..Default::default()
    }
}