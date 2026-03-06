use crate::ScanResult;
use reqwest;
use tokio::time::Duration;

pub async fn send_webhook_notification(webhook_url: &str, result: &ScanResult) {
    let no_ping_count = result.devices.iter().filter(|d| !d.responds_to_ping).count();
    
    let payload = serde_json::json!({
        "text": format!("NetScan Completo"),
        "blocks": [
            {
                "type": "header",
                "text": {
                    "type": "plain_text",
                    "text": "NetScan Pro - Relatório"
                }
            },
            {
                "type": "section",
                "fields": [
                    {
                        "type": "mrkdwn",
                        "text": format!("*Subnet:*\n{}", result.subnet)
                    },
                    {
                        "type": "mrkdwn",
                        "text": format!("*Dispositivos:*\n{}", result.total_devices)
                    },
                    {
                        "type": "mrkdwn",
                        "text": format!("*Respondem Ping:*\n{}", result.total_devices - no_ping_count)
                    },
                    {
                        "type": "mrkdwn",
                        "text": format!("*Duração:*\n{:.1}s", result.scan_duration_secs)
                    }
                ]
            },
            {
                "type": "context",
                "elements": [
                    {
                        "type": "mrkdwn",
                        "text": format!("Scan realizado em: {}", result.scan_time.format("%d/%m/%Y %H:%M:%S"))
                    }
                ]
            }
        ]
    });
    
    match reqwest::Client::builder()
        .timeout(Duration::from_secs(10))
        .build()
    {
        Ok(client) => {
            match client.post(webhook_url)
                .json(&payload)
                .send()
                .await
            {
                Ok(_) => println!("Notificação enviada para webhook"),
                Err(e) => eprintln!("Erro ao enviar webhook: {}", e),
            }
        }
        Err(e) => eprintln!("Erro ao criar cliente HTTP: {}", e),
    }
}

pub fn truncate(s: &str, max_chars: usize) -> String {
    if s.len() <= max_chars {
        s.to_string()
    } else {
        format!("{}...", &s[..max_chars-3])
    }
}
