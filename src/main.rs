mod export;
mod monitor;
mod web;
mod utils;

use export::export_results;
use monitor::run_monitoring_mode;
use web::start_web_server;
use utils::{send_webhook_notification, truncate};
use rand::Rng;

use std::net::{IpAddr, TcpStream, SocketAddr};
use std::time::{Duration, Instant};
use std::io::Read;
use tokio::task;
use tokio::sync::Semaphore;
use std::sync::Arc;
use std::collections::HashMap;
use colored::*;
use dns_lookup::lookup_addr;
use std::process::Command;
use clap::{Parser, ValueEnum};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Local};
use indicatif::{ProgressBar, ProgressStyle, MultiProgress};

#[derive(Parser)]
#[command(name = "NetScan Pro")]
#[command(about = "Scanner de Rede Avançado - FATEC DSM", long_about = None)]
struct Cli {
    /// Subnet base (ex: 10.67 ou 192.168.1)
    #[arg(short, long, default_value = "10.67")]
    subnet: String,

    /// Range inicial do terceiro octeto (ex: 56)
    #[arg(long, default_value_t = 56)]
    start_range: u8,

    /// Range final do terceiro octeto (ex: 57)
    #[arg(long, default_value_t = 57)]
    end_range: u8,

    /// Timeout para conexões TCP em milissegundos
    #[arg(short, long, default_value_t = 400)]
    timeout: u64,

    /// Portas para scan (separadas por vírgula)
    #[arg(short, long, default_value = "445,135,80,22,3389,443,21,23")]
    ports: String,

    /// Modo de scan
    #[arg(short, long, value_enum, default_value_t = ScanMode::Quick)]
    mode: ScanMode,

    /// Formato de exportação
    #[arg(short, long, value_enum)]
    export: Option<ExportFormat>,

    /// Número de conexões simultâneas
    #[arg(short, long, default_value_t = 100)]
    concurrency: usize,

    /// Ativar modo monitoramento contínuo
    #[arg(long)]
    monitor: bool,

    /// Intervalo de monitoramento em segundos
    #[arg(long, default_value_t = 60)]
    monitor_interval: u64,

    /// Ativar servidor web dashboard (porta 8080)
    #[arg(long)]
    web: bool,

    /// URL webhook para notificações
    #[arg(long)]
    webhook: Option<String>,

    /// Scan completo de portas (1-1024)
    #[arg(long)]
    full_port_scan: bool,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ScanMode {
    /// Scan rápido (apenas detecção)
    Quick,
    /// Scan completo (com serviços e latência)
    Full,
    /// Scan stealth (mais lento, menos detecções)
    Stealth,
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum ExportFormat {
    Json,
    Csv,
    Html,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Device {
    ip: String,
    hostname: String,
    responds_to_ping: bool,
    detected_ports: Vec<u16>,
    primary_port: u16,
    os_type: OsType,
    services: Vec<String>,
    tcp_latency_ms: f64,
    icmp_latency_ms: Option<f64>,
    mac_address: Option<String>,
    first_seen: DateTime<Local>,
    last_seen: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
enum OsType {
    Windows,
    Linux,
    NetworkDevice,
    Unknown,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
struct ScanResult {
    scan_time: DateTime<Local>,
    subnet: String,
    total_devices: usize,
    devices: Vec<Device>,
    scan_duration_secs: f64,
}

impl Device {
    fn new(ip: String, hostname: String, port: u16, tcp_latency_ms: f64) -> Self {
        let now = Local::now();
        Device {
            ip,
            hostname,
            responds_to_ping: false,
            detected_ports: vec![port],
            primary_port: port,
            os_type: OsType::Unknown,
            services: vec![],
            tcp_latency_ms,
            icmp_latency_ms: None,
            mac_address: None,
            first_seen: now,
            last_seen: now,
        }
    }
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    
    print_banner();
    
    if cli.web {
        println!("{}", "Iniciando servidor web em http://localhost:8080...".bright_blue());
        tokio::spawn(start_web_server());
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    if cli.monitor {
        println!("{}", "Modo monitoramento ativado".bright_yellow());
        println!("   Intervalo: {} segundos\n", cli.monitor_interval);
        run_monitoring_mode(cli).await;
    } else {
        let result = run_single_scan(&cli).await;
        
        if let Some(format) = cli.export {
            export_results(&result, format, &cli);
        }

        if let Some(webhook_url) = &cli.webhook {
            send_webhook_notification(webhook_url, &result).await;
        }
    }
}

fn print_banner() {
    println!("{}", "═══════════════════════════════════════════════════════════".bright_cyan());
    println!("{}", "   ███╗   ██╗███████╗████████╗███████╗ ██████╗ █████╗ ███╗   ██╗".bright_cyan().bold());
    println!("{}", "   ████╗  ██║██╔════╝╚══██╔══╝██╔════╝██╔════╝██╔══██╗████╗  ██║".bright_cyan().bold());
    println!("{}", "   ██╔██╗ ██║█████╗     ██║   ███████╗██║     ███████║██╔██╗ ██║".bright_cyan().bold());
    println!("{}", "   ██║╚██╗██║██╔══╝     ██║   ╚════██║██║     ██╔══██║██║╚██╗██║".bright_cyan().bold());
    println!("{}", "   ██║ ╚████║███████╗   ██║   ███████║╚██████╗██║  ██║██║ ╚████║".bright_cyan().bold());
    println!("{}", "   ╚═╝  ╚═══╝╚══════╝   ╚═╝   ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝".bright_cyan().bold());
    println!("{}", "   Scanner de Rede Profissional - FATEC DSM v2.0".bright_white());
    println!("{}", "═══════════════════════════════════════════════════════════\n".bright_cyan());
}

async fn run_single_scan(cli: &Cli) -> ScanResult {
    let start_time = Instant::now();
    let scan_start = Local::now();
    
    let ports: Vec<u16> = if cli.full_port_scan {
        (1..=1024).collect()
    } else {
        cli.ports.split(',')
            .filter_map(|p| p.trim().parse::<u16>().ok())
            .collect()
    };

    println!("Configuração do Scan:");
    println!("   Subnet: {}", cli.subnet.bright_white());
    println!("   Range: {}.{}.0 - {}.{}.255", cli.subnet, cli.start_range, cli.subnet, cli.end_range);
    println!("   Portas: {}", if cli.full_port_scan { "1-1024 (Full)".to_string() } else { cli.ports.clone() });
    println!("   Modo: {:?}", cli.mode);
    println!("   Concorrência: {}", cli.concurrency);
    println!("   Timeout: {}ms\n", cli.timeout);

    // Calcula total de IPs para scan
    let total_ips: usize = (cli.start_range..=cli.end_range)
        .map(|_| 254)
        .sum();

    let multi_progress = MultiProgress::new();
    let main_pb = multi_progress.add(ProgressBar::new(total_ips as u64));
    main_pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {pos}/{len} IPs ({per_sec}) {msg}")
            .unwrap()
            .progress_chars("#>-")
    );

    let semaphore = Arc::new(Semaphore::new(cli.concurrency));
    let mut tasks = vec![];

    for s in cli.start_range..=cli.end_range {
        for i in 1..=254 {
            let ip = format!("{}.{}.{}", cli.subnet, s, i);
            let sem_clone = Arc::clone(&semaphore);
            let ports_clone = ports.clone();
            let timeout = cli.timeout;
            let pb_clone = main_pb.clone();
            let mode = cli.mode;

            tasks.push(task::spawn(async move {
                let _permit = sem_clone.acquire().await.unwrap();
                let result = scan_device_advanced(ip, &ports_clone, timeout, mode).await;
                pb_clone.inc(1);
                result
            }));
        }
    }

    main_pb.set_message("Escaneando rede...");

    let mut devices = vec![];
    for task_handle in tasks {
        if let Ok(Some(device)) = task_handle.await {
            devices.push(device);
        }
    }

    main_pb.finish_with_message(format!("OK {} dispositivos encontrados", devices.len()));

    if !devices.is_empty() && cli.mode != ScanMode::Stealth {
        println!("\nTestando conectividade ICMP e detectando serviços...\n");
        enrich_device_info(&mut devices, cli).await;
    }

    let scan_duration = start_time.elapsed().as_secs_f64();

    ScanResult {
        scan_time: scan_start,
        subnet: format!("{}.{}-{}", cli.subnet, cli.start_range, cli.end_range),
        total_devices: devices.len(),
        devices: devices.clone(),
        scan_duration_secs: scan_duration,
    }
}

async fn scan_device_advanced(ip: String, ports: &[u16], timeout_ms: u64, mode: ScanMode) -> Option<Device> {
    let timeout = Duration::from_millis(timeout_ms);
    
    // Em modo stealth, adiciona delay randômico
    if mode == ScanMode::Stealth {
        let delay = {
            let mut rng = rand::thread_rng();
            rng.gen_range(50..150)
        };
        tokio::time::sleep(Duration::from_millis(delay)).await;
    }

    for &port in ports {
        if let Ok(addr) = format!("{}:{}", ip, port).parse::<SocketAddr>() {
            let start = Instant::now();
            if TcpStream::connect_timeout(&addr, timeout).is_ok() {
                let latency = start.elapsed().as_secs_f64() * 1000.0;
                let ip_addr: IpAddr = ip.parse().ok()?;
                let hostname = lookup_addr(&ip_addr).unwrap_or_else(|_| "---".into());
                return Some(Device::new(ip, hostname, port, latency));
            }
        }
    }
    None
}

async fn enrich_device_info(devices: &mut [Device], cli: &Cli) {
    let semaphore = Arc::new(Semaphore::new(50));
    let pb = ProgressBar::new(devices.len() as u64);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.green/blue}] {pos}/{len} Analisando dispositivos...")
            .unwrap()
    );

    let mut tasks = vec![];
    
    for device in devices.iter() {
        let ip = device.ip.clone();
        let primary_port = device.primary_port;
        let sem_clone = Arc::clone(&semaphore);
        let pb_clone = pb.clone();
        let full_scan = cli.full_port_scan;
        let ports: Vec<u16> = cli.ports.split(',')
            .filter_map(|p| p.trim().parse::<u16>().ok())
            .collect();

        tasks.push(task::spawn(async move {
            let _permit = sem_clone.acquire().await.unwrap();
            
            // Teste ICMP
            let (responds, icmp_latency) = ping_with_latency(&ip).await;
            
            // Detecção de OS
            let os = detect_os(primary_port, &ip).await;
            
            // Scan de portas adicionais
            let detected_ports = if full_scan {
                scan_all_ports(&ip, &ports).await
            } else {
                vec![primary_port]
            };
            
            // Detecção de serviços
            let services = detect_services(&detected_ports).await;
            
            pb_clone.inc(1);
            (ip, responds, icmp_latency, os, detected_ports, services)
        }));
    }

    for task_handle in tasks {
        if let Ok((ip, responds, icmp_latency, os, ports_found, services)) = task_handle.await {
            if let Some(device) = devices.iter_mut().find(|d| d.ip == ip) {
                device.responds_to_ping = responds;
                device.icmp_latency_ms = icmp_latency;
                device.os_type = os;
                device.detected_ports = ports_found;
                device.services = services;
            }
        }
    }

    pb.finish_and_clear();
    display_results(devices, cli);
}

async fn ping_with_latency(ip: &str) -> (bool, Option<f64>) {
    let start = Instant::now();
    let output = Command::new("ping")
        .arg("-n")
        .arg("1")
        .arg("-w")
        .arg("1000")
        .arg(ip)
        .output();
    
    match output {
        Ok(result) if result.status.success() => {
            let latency = start.elapsed().as_secs_f64() * 1000.0;
            (true, Some(latency))
        }
        _ => (false, None),
    }
}

async fn detect_os(primary_port: u16, ip: &str) -> OsType {
    match primary_port {
        445 | 135 | 3389 => OsType::Windows,
        22 => {
            // Tenta banner grabbing SSH para diferenciar Linux de Network Device
            if let Some(banner) = grab_banner(ip, 22).await {
                if banner.to_lowercase().contains("cisco") || 
                   banner.to_lowercase().contains("huawei") ||
                   banner.to_lowercase().contains("juniper") {
                    return OsType::NetworkDevice;
                }
            }
            OsType::Linux
        },
        80 | 443 => {
            // Tenta detectar por headers HTTP
            OsType::Unknown
        },
        23 => OsType::NetworkDevice, // Telnet geralmente é dispositivo de rede
        _ => OsType::Unknown,
    }
}

async fn grab_banner(ip: &str, port: u16) -> Option<String> {
    let addr = format!("{}:{}", ip, port);
    if let Ok(socket_addr) = addr.parse::<SocketAddr>() {
        if let Ok(mut stream) = TcpStream::connect_timeout(&socket_addr, Duration::from_millis(2000)) {
            stream.set_read_timeout(Some(Duration::from_millis(2000))).ok()?;
            let mut buffer = [0u8; 512];
            if let Ok(n) = stream.read(&mut buffer) {
                return String::from_utf8_lossy(&buffer[..n]).to_string().into();
            }
        }
    }
    None
}

async fn scan_all_ports(ip: &str, ports: &[u16]) -> Vec<u16> {
    let mut open_ports = vec![];
    for &port in ports {
        if let Ok(addr) = format!("{}:{}", ip, port).parse::<SocketAddr>() {
            if TcpStream::connect_timeout(&addr, Duration::from_millis(200)).is_ok() {
                open_ports.push(port);
            }
        }
    }
    open_ports
}

async fn detect_services(ports: &[u16]) -> Vec<String> {
    ports.iter().filter_map(|&port| {
        Some(match port {
            20 | 21 => "FTP".to_string(),
            22 => "SSH".to_string(),
            23 => "Telnet".to_string(),
            25 => "SMTP".to_string(),
            53 => "DNS".to_string(),
            80 => "HTTP".to_string(),
            110 => "POP3".to_string(),
            135 => "RPC/Windows".to_string(),
            143 => "IMAP".to_string(),
            443 => "HTTPS".to_string(),
            445 => "SMB/CIFS".to_string(),
            3306 => "MySQL".to_string(),
            3389 => "RDP/Windows".to_string(),
            5432 => "PostgreSQL".to_string(),
            5900 => "VNC".to_string(),
            8080 => "HTTP-Alt".to_string(),
            _ => format!("Port {}", port),
        })
    }).collect()
}

fn display_results(devices: &[Device], cli: &Cli) {
    println!("\n{}", "═══════════════════════════════════════════════════════════".bright_cyan());
    println!("{}", "   RESULTADOS COMPLETOS".bright_cyan().bold());
    println!("{}", "═══════════════════════════════════════════════════════════\n".bright_cyan());
    
    println!("{:<16} | {:<20} | {:<12} | {:<10} | {:<8} | {:<8}", 
        "IP", "HOSTNAME", "OS", "PING", "TCP (ms)", "ICMP (ms)");
    println!("{}", "-".repeat(100));
    
    let mut no_ping_count = 0;
    let mut by_os: HashMap<String, usize> = HashMap::new();
    
    for device in devices {
        let os_str = format!("{:?}", device.os_type);
        *by_os.entry(os_str.clone()).or_insert(0) += 1;
        
        let ping_status = if device.responds_to_ping {
            "OK".green()
        } else {
            no_ping_count += 1;
            "NO".red()
        };
        
        let icmp_latency = device.icmp_latency_ms
            .map(|l| format!("{:.1}", l))
            .unwrap_or_else(|| "---".to_string());
        
        println!("{:<16} | {:<20} | {:<12} | {:<10} | {:<8.1} | {:<8}", 
            device.ip.bright_white(),
            truncate(&device.hostname, 20).yellow(),
            os_str.cyan(),
            ping_status,
            device.tcp_latency_ms,
            icmp_latency
        );
        
        // Mostra portas e serviços se modo full
        if cli.mode == ScanMode::Full && !device.services.is_empty() {
            println!("                   └─ Serviços: {}", 
                device.services.join(", ").bright_blue());
        }
    }
    
    println!("\n{}", "═".repeat(100));
    println!("{}", "📈 RESUMO ESTATÍSTICO:".bright_white().bold());
    println!("   Total de dispositivos: {}", devices.len().to_string().green().bold());
    println!("   Respondem a ping (ICMP): {} ({}%)", 
        (devices.len() - no_ping_count).to_string().green(),
        ((devices.len() - no_ping_count) * 100 / devices.len().max(1)).to_string().green()
    );
    println!("   Bloqueiam ping: {} ({}%)", 
        no_ping_count.to_string().red(),
        (no_ping_count * 100 / devices.len().max(1)).to_string().red()
    );
    
    println!("\n   Distribuição por OS:");
    for (os, count) in by_os.iter() {
        println!("      • {}: {}", os.cyan(), count.to_string().bright_white());
    }
    
    let avg_tcp = devices.iter().map(|d| d.tcp_latency_ms).sum::<f64>() / devices.len().max(1) as f64;
    let avg_icmp: f64 = devices.iter()
        .filter_map(|d| d.icmp_latency_ms)
        .sum::<f64>() / devices.iter().filter(|d| d.icmp_latency_ms.is_some()).count().max(1) as f64;
    
    println!("\n   Latência Média:");
    println!("      • TCP: {:.2}ms", avg_tcp);
    println!("      • ICMP: {:.2}ms", avg_icmp);
    
    if no_ping_count > 0 {
        println!("\n{}", "AVISO: DISPOSITIVOS COM FIREWALL BLOQUEANDO ICMP:".red().bold());
        println!("{}", "-".repeat(100));
        for device in devices.iter().filter(|d| !d.responds_to_ping) {
            println!("   • {} ({}) - {} - Portas: {:?}", 
                device.ip.yellow(),
                device.hostname,
                format!("{:?}", device.os_type).cyan(),
                device.detected_ports
            );
        }
    }
}