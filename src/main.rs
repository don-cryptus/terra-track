use chrono::Duration;
use dotenvy::dotenv;
use migration::sea_orm::Set;
use scanner::{ip_iterator::Ipv4Iter, scanner::Scanner, scripts::Script};
use service::{
    models::{ip_main_service, scan_batch_service},
    parser::parse_nmap_results,
    utils::date,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().expect(".env file not found");

    let scan = scan_batch_service::Query::next_scan_batch().await?;

    printlog!("Open scan: {:?}", scan);

    let year_ago = chrono::Utc::now().with_timezone(&chrono::FixedOffset::east_opt(0).unwrap())
        - Duration::days(365);
    let mut ip_iter = Ipv4Iter::batched(&scan.ip, scan.batch_size);
    while let Some(ip) = ip_iter.next() {
        printlog!("Scanning IP: {}", ip);

        let ip_main =
            ip_main_service::Query::find_ip_main_by_ip_older_then(&ip.to_string(), Some(year_ago)).await?;

        let ports = Scanner::new(ip.into()).run().await?;
        printlog!("Open ports: {:?}", ports);
        let script = Script::new(ip.into(), vec![]);
        let result = script.parse_nmap_xml();
        // let script = Script::new(ip.into(), ports);
        // let result = script.run();

        if let Ok(result) = result {
            printlog!("Script result: {:?}", result);
            parse_nmap_results(result).await?;
        }
    }

    scan_batch_service::Mutation::update_scan_batch(entity::scan_batch::ActiveModel {
        id: Set(scan.id),
        end: Set(Some(date())),
        ..Default::default()
    })
    .await?;

    // printlog!("Scan complete");

    Ok(())
}

#[macro_export]
macro_rules! printlog {
    ($($arg:tt)*) => {
        {
            use chrono::{Local, DateTime};
            let now: DateTime<Local> = Local::now();
            let millis = now.timestamp_subsec_millis();
            println!("{}.{:03}: {}", now.format("%Y-%m-%d %H:%M:%S"), millis, format!($($arg)*));
        }
    };
}

// //ignore linting
// #[allow(dead_code)]
// pub async fn scan(ip: &'static str) -> anyhow::Result<NmapXML> {
//     let ip: IpAddr = ip.parse()?;
//     let ports = Scanner::new(ip).run().await?;

//     println!("IP {:?} Open ports: {:?}", ip.to_string(), ports);

//     let script = Script::new(ip, ports);
//     let result = script.run();
//     if let Ok(result) = result {
//         return Ok(result);
//     }

//     Err(result.err().unwrap())
// }
