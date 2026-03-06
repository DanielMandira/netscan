use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::Duration;
use tokio::task;
use tokio::sync::Semaphore;
use std::sync::Arc;
use colored::*;
use dns_lookup::lookup_addr;
use std::process::Command;

#[derive(Debug, Clone)]
struct Device {
    ip: String,
    hostname: String,
    responds_to_ping: bool,
    detected_port: u16,
}

#[tokio::main]
async fn main() {
    let base_subnet = "10.67";
    let subnets = [56, 57]; // Range /23 (56.0 até 57.255)
    
    println!("{}", "===============================================".bright_cyan());
    println!("{}", "   SCANNER DE REDE /23 - FATEC DSM".bright_cyan().bold());
    println!("   Escaneando: {}.56.0 - {}.57.255", base_subnet, base_subnet);
    println!("{}", "===============================================\n".bright_cyan());

    // Limita a 100 conexões simultâneas para evitar bloqueio do Windows/Firewall
    let semaphore = Arc::new(Semaphore::new(100));
    let mut tasks = vec![];

    for s in subnets {
        for i in 1..255 {
            let ip = format!("{}.{}.{}", base_subnet, s, i);
            let sem_clone = Arc::clone(&semaphore);

            tasks.push(task::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                scan_device(ip).await
            }));
        }
    }

    println!("{:<16} | {:<25} | {:<10} | {:<10}", "IP", "HOSTNAME", "TCP", "ICMP PING");
    println!("{}", "-".repeat(75));

    let mut devices = vec![];
    for task in tasks {
        if let Ok(Some((ip, host, port))) = task.await {
            devices.push(Device { 
                ip: ip.clone(), 
                hostname: host.clone(),
                responds_to_ping: false,
                detected_port: port,
            });
        }
    }

    println!("\n{} dispositivos ativos encontrados via TCP.", devices.len());

    if !devices.is_empty() {
        println!("\n{}", "===============================================".bright_yellow());
        println!("{}", "   TESTANDO CONECTIVIDADE ICMP (PING)".bright_yellow().bold());
        println!("{}", "===============================================\n".bright_yellow());
        
        test_ping_all(&mut devices).await;
        display_results(&devices);
    }
}

async fn scan_device(ip: String) -> Option<(String, String, u16)> {
    let timeout = Duration::from_millis(400);
    // Portas variadas para detectar Windows, Linux e Equipamentos de Rede
    let ports = [445, 135, 80, 22]; 

    for port in ports {
        if let Ok(addr) = format!("{}:{}", ip, port).parse::<SocketAddr>() {
            // Usamos std::net::TcpStream dentro do spawn pois é simples para checagem rápida
            if TcpStream::connect_timeout(&addr, timeout).is_ok() {
                let ip_addr: IpAddr = ip.parse().ok()?;
                let hostname = lookup_addr(&ip_addr).unwrap_or_else(|_| "---".into());
                return Some((ip, hostname, port));
            }
        }
    }
    None
}

async fn test_ping_all(devices: &mut [Device]) {
    let semaphore = Arc::new(Semaphore::new(50));
    let mut tasks = vec![];
    
    // Testa ping da máquina local para cada dispositivo encontrado
    for device in devices.iter() {
        let ip = device.ip.clone();
        let sem_clone = Arc::clone(&semaphore);
        
        tasks.push(task::spawn(async move {
            let _permit = sem_clone.acquire().await.unwrap();
            let responds = ping_device(&ip).await;
            (ip, responds)
        }));
    }
    
    // Atualiza o status de ping de cada dispositivo
    for task_handle in tasks {
        if let Ok((ip, responds)) = task_handle.await {
            if let Some(device) = devices.iter_mut().find(|d| d.ip == ip) {
                device.responds_to_ping = responds;
            }
        }
    }
}

fn display_results(devices: &[Device]) {
    println!("\n{}", "===============================================".bright_cyan());
    println!("{}", "   RESULTADOS COMPLETOS".bright_cyan().bold());
    println!("{}", "===============================================\n".bright_cyan());
    
    println!("{:<16} | {:<25} | {:<15} | {:<12}", "IP", "HOSTNAME", "PORTA TCP", "PING (ICMP)");
    println!("{}", "-".repeat(80));
    
    let mut no_ping_count = 0;
    
    for device in devices {
        let port_info = get_port_description(device.detected_port);
        let ping_status = if device.responds_to_ping {
            "OK".green().bold()
        } else {
            no_ping_count += 1;
            "BLOQUEADO".red().bold()
        };
        
        println!("{:<16} | {:<25} | {:<15} | {}", 
            device.ip.bright_white(),
            device.hostname.yellow(),
            port_info.cyan(),
            ping_status
        );
    }
    
    println!("\n{}", "=".repeat(80));
    println!("{}", "RESUMO:".bright_white().bold());
    println!("  Total de dispositivos: {}", devices.len().to_string().green());
    println!("  Respondem a ping: {}", (devices.len() - no_ping_count).to_string().green());
    println!("  Bloqueiam ping (firewall): {}", no_ping_count.to_string().red());
    
    if no_ping_count > 0 {
        println!("\n{}", "⚠ DISPOSITIVOS COM ICMP BLOQUEADO:".red().bold());
        println!("{}", "-".repeat(80));
        for device in devices.iter().filter(|d| !d.responds_to_ping) {
            println!("  • {} ({}) - Porta {} aberta mas ping bloqueado", 
                device.ip.yellow(),
                device.hostname,
                device.detected_port
            );
        }
        println!("\n{}", "Estes dispositivos têm firewall bloqueando ICMP (ping).".yellow());
        println!("{}", "Eles estão ativos mas configurados para não responder ping.".yellow());
    }
}

fn get_port_description(port: u16) -> String {
    match port {
        445 => format!("{} (SMB/Win)", port),
        135 => format!("{} (RPC/Win)", port),
        80 => format!("{} (HTTP)", port),
        22 => format!("{} (SSH)", port),
        _ => format!("{}", port),
    }
}

async fn ping_device(ip: &str) -> bool {
    // Usa o comando ping do Windows com timeout de 1 segundo e 1 tentativa
    let output = Command::new("ping")
        .arg("-n")
        .arg("1")
        .arg("-w")
        .arg("1000")
        .arg(ip)
        .output();
    
    match output {
        Ok(result) => result.status.success(),
        Err(_) => false,
    }
}