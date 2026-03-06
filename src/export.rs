use crate::{ScanResult, ExportFormat, Cli};
use std::fs;
use chrono::Local;

pub fn export_results(result: &ScanResult, format: ExportFormat, cli: &Cli) {
    let timestamp = Local::now().format("%Y%m%d_%H%M%S").to_string();
    let filename = match format {
        ExportFormat::Json => export_json(result, &timestamp),
        ExportFormat::Csv => export_csv(result, &timestamp),
        ExportFormat::Html => export_html(result, &timestamp, cli),
    };
    
    println!("\nResultados exportados: {}", filename);
}

fn export_json(result: &ScanResult, timestamp: &str) -> String {
    let filename = format!("scan_{}_{}.json", result.subnet.replace(".", "_"), timestamp);
    let json = serde_json::to_string_pretty(result).unwrap();
    fs::write(&filename, json).unwrap();
    filename
}

fn export_csv(result: &ScanResult, timestamp: &str) -> String {
    let filename = format!("scan_{}_{}.csv", result.subnet.replace(".", "_"), timestamp);
    let mut wtr = csv::Writer::from_path(&filename).unwrap();
    
    wtr.write_record(&[
        "IP", "Hostname", "OS", "Responds_Ping", "Primary_Port", 
        "TCP_Latency_ms", "ICMP_Latency_ms", "Services", "First_Seen"
    ]).unwrap();
    
    for device in &result.devices {
        wtr.write_record(&[
            &device.ip,
            &device.hostname,
            &format!("{:?}", device.os_type),
            &device.responds_to_ping.to_string(),
            &device.primary_port.to_string(),
            &format!("{:.2}", device.tcp_latency_ms),
            &device.icmp_latency_ms.map(|l| format!("{:.2}", l)).unwrap_or_else(|| "N/A".to_string()),
            &device.services.join(";"),
            &device.first_seen.to_string(),
        ]).unwrap();
    }
    
    wtr.flush().unwrap();
    filename
}

fn export_html(result: &ScanResult, timestamp: &str, _cli: &Cli) -> String {
    let filename = format!("scan_{}_{}.html", result.subnet.replace(".", "_"), timestamp);
    
    let mut rows = String::new();
    for device in &result.devices {
        let ping_status = if device.responds_to_ping {
            "<span style='color: green;'>OK</span>"
        } else {
            "<span style='color: red;'>NO</span>"
        };
        
        rows.push_str(&format!(
            "<tr>
                <td>{}</td>
                <td>{}</td>
                <td>{:?}</td>
                <td>{}</td>
                <td>{:.2}</td>
                <td>{}</td>
                <td>{}</td>
            </tr>",
            device.ip,
            device.hostname,
            device.os_type,
            ping_status,
            device.tcp_latency_ms,
            device.icmp_latency_ms.map(|l| format!("{:.2}ms", l)).unwrap_or_else(|| "N/A".to_string()),
            device.services.join(", ")
        ));
    }
    
    let html = format!(r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NetScan - {}</title>
    <style>
        body {{
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            margin: 20px;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: #333;
        }}
        .container {{
            max-width: 1400px;
            margin: 0 auto;
            background: white;
            padding: 30px;
            border-radius: 10px;
            box-shadow: 0 10px 40px rgba(0,0,0,0.2);
        }}
        h1 {{
            color: #667eea;
            text-align: center;
            margin-bottom: 10px;
        }}
        .stats {{
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
            gap: 20px;
            margin: 30px 0;
        }}
        .stat-card {{
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            padding: 20px;
            border-radius: 8px;
            text-align: center;
        }}
        .stat-card h3 {{
            margin: 0;
            font-size: 32px;
        }}
        .stat-card p {{
            margin: 10px 0 0 0;
            opacity: 0.9;
        }}
        table {{
            width: 100%;
            border-collapse: collapse;
            margin-top: 20px;
        }}
        th {{
            background: #667eea;
            color: white;
            padding: 12px;
            text-align: left;
        }}
        td {{
            padding: 10px;
            border-bottom: 1px solid #ddd;
        }}
        tr:hover {{
            background: #f5f5f5;
        }}
        .timestamp {{
            text-align: center;
            color: #666;
            margin-top: 20px;
        }}
    </style>
</head>
<body>
    <div class="container">
        <h1>NetScan Pro - Relatório de Rede</h1>
        <p style="text-align: center; color: #666;">Subnet: {} | Scan: {}</p>
        
        <div class="stats">
            <div class="stat-card">
                <h3>{}</h3>
                <p>Dispositivos Ativos</p>
            </div>
            <div class="stat-card">
                <h3>{}</h3>
                <p>Respondem Ping</p>
            </div>
            <div class="stat-card">
                <h3>{:.1}s</h3>
                <p>Tempo de Scan</p>
            </div>
        </div>
        
        <table>
            <thead>
                <tr>
                    <th>IP</th>
                    <th>Hostname</th>
                    <th>OS</th>
                    <th>Ping</th>
                    <th>TCP (ms)</th>
                    <th>ICMP (ms)</th>
                    <th>Serviços</th>
                </tr>
            </thead>
            <tbody>
                {}
            </tbody>
        </table>
        
        <p class="timestamp">Gerado em {} por NetScan Pro v2.0</p>
    </div>
</body>
</html>"#,
        result.subnet,
        result.subnet,
        result.scan_time.format("%d/%m/%Y %H:%M:%S"),
        result.total_devices,
        result.devices.iter().filter(|d| d.responds_to_ping).count(),
        result.scan_duration_secs,
        rows,
        Local::now().format("%d/%m/%Y %H:%M:%S")
    );
    
    fs::write(&filename, html).unwrap();
    filename
}
