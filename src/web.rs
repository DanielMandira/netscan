use warp::Filter;
use std::sync::{Arc, Mutex};
use crate::ScanResult;

lazy_static::lazy_static! {
    static ref LATEST_SCAN: Arc<Mutex<Option<ScanResult>>> = Arc::new(Mutex::new(None));
}

pub async fn start_web_server() {
    let index = warp::path::end()
        .map(|| warp::reply::html(get_dashboard_html()));
    
    let api_status = warp::path!("api" / "status")
        .map(|| {
            let scan = LATEST_SCAN.lock().unwrap();
            warp::reply::json(&*scan)
        });
    
    let routes = index.or(api_status);
    
    println!("Dashboard disponível em: http://localhost:8080");
    warp::serve(routes).run(([127, 0, 0, 1], 8080)).await;
}

pub fn update_scan_data(result: ScanResult) {
    let mut scan = LATEST_SCAN.lock().unwrap();
    *scan = Some(result);
}

fn get_dashboard_html() -> String {
    r#"<!DOCTYPE html>
<html lang="pt-BR">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>NetScan Pro - Dashboard</title>
    <style>
        * { margin: 0; padding: 0; box-sizing: border-box; }
        body {
            font-family: 'Segoe UI', Tahoma, Geneva, Verdana, sans-serif;
            background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
            color: white;
            min-height: 100vh;
            padding: 20px;
        }
        .container {
            max-width: 1400px;
            margin: 0 auto;
        }
        h1 {
            text-align: center;
            margin-bottom: 30px;
            font-size: 48px;
            text-shadow: 2px 2px 4px rgba(0,0,0,0.3);
        }
        .stats {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
            gap: 20px;
            margin-bottom: 30px;
        }
        .stat-card {
            background: rgba(255, 255, 255, 0.1);
            backdrop-filter: blur(10px);
            padding: 30px;
            border-radius: 15px;
            text-align: center;
            border: 1px solid rgba(255,255,255,0.2);
        }
        .stat-card h2 {
            font-size: 48px;
            margin-bottom: 10px;
        }
        .stat-card p {
            opacity: 0.9;
            font-size: 18px;
        }
        .devices {
            background: rgba(255, 255, 255, 0.95);
            color: #333;
            padding: 30px;
            border-radius: 15px;
        }
        table {
            width: 100%;
            border-collapse: collapse;
        }
        th {
            background: #667eea;
            color: white;
            padding: 15px;
            text-align: left;
        }
        td {
            padding: 12px 15px;
            border-bottom: 1px solid #ddd;
        }
        tr:hover {
            background: #f0f0f0;
        }
        .status-online { color: #22c55e; font-weight: bold; }
        .status-offline { color: #ef4444; font-weight: bold; }
        #refresh-btn {
            background: white;
            color: #667eea;
            border: none;
            padding: 15px 30px;
            border-radius: 8px;
            font-size: 18px;
            cursor: pointer;
            margin: 20px auto;
            display: block;
            font-weight: bold;
        }
        #refresh-btn:hover {
            background: #f0f0f0;
        }
    </style>
</head>
<body>
    <div class="container">
        <h1>NetScan Pro Dashboard</h1>
        
        <div class="stats">
            <div class="stat-card">
                <h2 id="total-devices">--</h2>
                <p>Dispositivos Ativos</p>
            </div>
            <div class="stat-card">
                <h2 id="ping-count">--</h2>
                <p>Respondem Ping</p>
            </div>
            <div class="stat-card">
                <h2 id="scan-time">--</h2>
                <p>Último Scan</p>
            </div>
        </div>
        
        <button id="refresh-btn" onclick="loadData()">Atualizar Dados</button>
        
        <div class="devices">
            <h2 style="margin-bottom: 20px;">Dispositivos na Rede</h2>
            <table id="devices-table">
                <thead>
                    <tr>
                        <th>IP</th>
                        <th>Hostname</th>
                        <th>OS</th>
                        <th>Ping</th>
                        <th>Latência TCP</th>
                        <th>Serviços</th>
                    </tr>
                </thead>
                <tbody id="devices-body">
                    <tr><td colspan="6" style="text-align: center; padding: 50px;">Carregando dados...</td></tr>
                </tbody>
            </table>
        </div>
    </div>
    
    <script>
        function loadData() {
            fetch('/api/status')
                .then(response => response.json())
                .then(data => {
                    if (!data) {
                        document.getElementById('devices-body').innerHTML = 
                            '<tr><td colspan="6" style="text-align: center; padding: 50px;">Nenhum scan realizado ainda.</td></tr>';
                        return;
                    }
                    
                    document.getElementById('total-devices').textContent = data.total_devices;
                    document.getElementById('ping-count').textContent = 
                        data.devices.filter(d => d.responds_to_ping).length;
                    document.getElementById('scan-time').textContent = 
                        new Date(data.scan_time).toLocaleTimeString('pt-BR');
                    
                    const tbody = document.getElementById('devices-body');
                    tbody.innerHTML = data.devices.map(device => `
                        <tr>
                            <td>${device.ip}</td>
                            <td>${device.hostname}</td>
                            <td>${device.os_type}</td>
                            <td class="${device.responds_to_ping ? 'status-online' : 'status-offline'}">
                                ${device.responds_to_ping ? 'OK Online' : 'NO Bloqueado'}
                            </td>
                            <td>${device.tcp_latency_ms.toFixed(2)}ms</td>
                            <td>${device.services.join(', ') || 'N/A'}</td>
                        </tr>
                    `).join('');
                })
                .catch(err => console.error('Erro ao carregar dados:', err));
        }
        
        loadData();
        setInterval(loadData, 10000); // Atualiza a cada 10 segundos
    </script>
</body>
</html>"#.to_string()
}
