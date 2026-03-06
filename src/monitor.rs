use crate::{Cli, run_single_scan, Device, ScanResult};
use colored::*;
use std::collections::HashMap;
use tokio::time::{sleep, Duration};

pub async fn run_monitoring_mode(cli: Cli) {
    let mut previous_devices: HashMap<String, Device> = HashMap::new();
    let mut iteration = 0;
    
    loop {
        iteration += 1;
        println!("\n{}", format!("Iteração #{} - {}", iteration, chrono::Local::now().format("%H:%M:%S")).bright_blue().bold());
        
        let result = run_single_scan(&cli).await;
        
        if iteration > 1 {
            detect_changes(&previous_devices, &result);
        }
        
        // Atualiza mapa de dispositivos
        previous_devices.clear();
        for device in &result.devices {
            previous_devices.insert(device.ip.clone(), device.clone());
        }
        
        if let Some(webhook_url) = &cli.webhook {
            send_monitoring_update(webhook_url, &result).await;
        }
        
        println!("\nAguardando {} segundos até próximo scan...", cli.monitor_interval);
        sleep(Duration::from_secs(cli.monitor_interval)).await;
    }
}

fn detect_changes(previous: &HashMap<String, Device>, current: &ScanResult) {
    let current_ips: HashMap<String, &Device> = current.devices
        .iter()
        .map(|d| (d.ip.clone(), d))
        .collect();
    
    // Detecta novos dispositivos
    let new_devices: Vec<_> = current.devices
        .iter()
        .filter(|d| !previous.contains_key(&d.ip))
        .collect();
    
    if !new_devices.is_empty() {
        println!("\n{}", "NOVOS DISPOSITIVOS DETECTADOS:".green().bold());
        for device in new_devices {
            println!("   OK {} ({}) - {:?}", 
                device.ip.green(),
                device.hostname,
                device.os_type
            );
        }
    }
    
    // Detecta dispositivos removidos
    let removed_devices: Vec<_> = previous
        .iter()
        .filter(|(ip, _)| !current_ips.contains_key(*ip))
        .collect();
    
    if !removed_devices.is_empty() {
        println!("\n{}", "DISPOSITIVOS OFFLINE:".red().bold());
        for (ip, device) in removed_devices {
            println!("   NO {} ({}) - OFFLINE", 
                ip.red(),
                device.hostname
            );
        }
    }
    
    // Detecta mudanças de estado de ping
    for (ip, prev_device) in previous.iter() {
        if let Some(curr_device) = current_ips.get(ip) {
            if prev_device.responds_to_ping != curr_device.responds_to_ping {
                if curr_device.responds_to_ping {
                    println!("\n[+] {} agora responde a ping", ip.green());
                } else {
                    println!("\n[-] {} parou de responder ping", ip.red());
                }
            }
        }
    }
}

async fn send_monitoring_update(webhook_url: &str, result: &ScanResult) {
    let payload = serde_json::json!({
        "timestamp": result.scan_time.to_rfc3339(),
        "subnet": result.subnet,
        "total_devices": result.total_devices,
        "scan_duration": result.scan_duration_secs,
        "message": format!("Monitoramento: {} dispositivos ativos", result.total_devices)
    });
    
    if let Ok(client) = reqwest::Client::builder().timeout(Duration::from_secs(5)).build() {
        let _ = client.post(webhook_url)
            .json(&payload)
            .send()
            .await;
    }
}
