# Exemplos de Output - NetScan Pro

## Banner Inicial

```
═══════════════════════════════════════════════════════════
   ███╗   ██╗███████╗████████╗███████╗ ██████╗ █████╗ ███╗   ██╗
   ████╗  ██║██╔════╝╚══██╔══╝██╔════╝██╔════╝██╔══██╗████╗  ██║
   ██╔██╗ ██║█████╗     ██║   ███████╗██║     ███████║██╔██╗ ██║
   ██║╚██╗██║██╔══╝     ██║   ╚════██║██║     ██╔══██║██║╚██╗██║
   ██║ ╚████║███████╗   ██║   ███████║╚██████╗██║  ██║██║ ╚████║
   ╚═╝  ╚═══╝╚══════╝   ╚═╝   ╚══════╝ ╚═════╝╚═╝  ╚═╝╚═╝  ╚═══╝
   Scanner de Rede Profissional - FATEC DSM v2.0
═══════════════════════════════════════════════════════════
```

## Configuração do Scan

```
Configuração do Scan:
   Subnet: 10.67
   Range: 10.67.56.0 - 10.67.57.255
   Portas: 445,135,80,22,3389,443,21,23
   Modo: Full
   Concorrência: 100
   Timeout: 400ms

[========================================] 508/508 IPs (42.3 IPs/s) OK 45 dispositivos encontrados
```

## Progress Bar em Ação

```
Testando conectividade ICMP e detectando serviços...

[████████████████████████████████████████] 45/45 Analisando dispositivos...
```

## Resultados Detalhados - Modo Full

```
═══════════════════════════════════════════════════════════
   RESULTADOS COMPLETOS
═══════════════════════════════════════════════════════════

IP               | HOSTNAME             | OS            | PING | TCP (ms) | ICMP (ms)
────────────────────────────────────────────────────────────────────────────────────
10.67.56.10      | PC-LAB-01            | Windows       | OK   | 12.5     | 8.3
                   └─ Serviços: SMB/CIFS, RPC/Windows, RDP/Windows
10.67.56.15      | SERVER-DSM           | Linux         | NO   | 45.2     | ---
                   └─ Serviços: SSH, HTTP
10.67.56.20      | PC-ALUNO-05          | Windows       | OK   | 18.7     | 12.1
                   └─ Serviços: SMB/CIFS, RPC/Windows
10.67.56.25      | PRINTER-HP-01        | Unknown       | OK   | 8.4      | 5.2
                   └─ Serviços: HTTP
10.67.56.30      | PC-LAB-02            | Windows       | OK   | 15.3     | 9.8
                   └─ Serviços: SMB/CIFS, RPC/Windows, RDP/Windows
10.67.57.1       | GATEWAY-FATEC        | NetworkDevice | OK   | 1.2      | 0.8
                   └─ Serviços: SSH, Telnet, HTTP
10.67.57.10      | SWITCH-CORE-01       | NetworkDevice | OK   | 2.5      | 1.5
                   └─ Serviços: SSH, HTTP
10.67.57.50      | SERVER-BD            | Linux         | NO   | 38.9     | ---
                   └─ Serviços: SSH, MySQL, PostgreSQL

═══════════════════════════════════════════════════════════
RESUMO ESTATÍSTICO:
   Total de dispositivos: 45
   Respondem a ping (ICMP): 30 (66%)
   Bloqueiam ping: 15 (33%)

   Distribuição por OS:
      • Windows: 25
      • Linux: 15
      • NetworkDevice: 5

   Latência Média:
      • TCP: 18.34ms
      • ICMP: 6.72ms

AVISO: DISPOSITIVOS COM FIREWALL BLOQUEANDO ICMP:
────────────────────────────────────────────────────────────────────────────────────
   • 10.67.56.15 (SERVER-DSM) - Linux - Portas: [22, 80]
   • 10.67.57.50 (SERVER-BD) - Linux - Portas: [22, 3306, 5432]
   • 10.67.56.100 (PC-ADM-01) - Windows - Portas: [445, 135, 3389]
   ...

Estes dispositivos têm firewall bloqueando ICMP (ping).
Eles estão ativos mas configurados para não responder ping.

Resultados exportados: scan_10_67_56-57_20260305_143522.html
```

## Dashboard Web Output

Ao acessar `http://localhost:8080`:

```
╔═══════════════════════════════════════════════╗
║        NetScan Pro Dashboard                  ║
║        Subnet: 10.67.56-57                    ║
║        Scan: 14:35:22                         ║
╚═══════════════════════════════════════════════╝

┌─────────────────┬─────────────────┬─────────────────┐
│       45        │       30        │     15.2s       │
│  Dispositivos   │  Respondem      │  Tempo de Scan  │
│     Ativos      │     Ping        │                 │
└─────────────────┴─────────────────┴─────────────────┘

[Atualizar Dados]

Dispositivos na Rede
┌─────────────────┬──────────────────┬──────────┬──────┬──────────┬──────────────────┐
│ IP              │ Hostname         │ OS       │ Ping │ TCP (ms) │ Serviços         │
├─────────────────┼──────────────────┼──────────┼──────┼──────────┼──────────────────┤
│ 10.67.56.10     │ PC-LAB-01        │ Windows  │ OK   │ 12.50    │ SMB, RPC, RDP    │
│ 10.67.56.15     │ SERVER-DSM       │ Linux    │ NO   │ 45.20    │ SSH, HTTP        │
│ 10.67.57.1      │ GATEWAY-FATEC    │ Network  │ OK   │ 1.20     │ SSH, Telnet      │
└─────────────────┴──────────────────┴──────────┴──────┴──────────┴──────────────────┘
```

## Modo Monitoramento

```
Modo monitoramento ativado
   Intervalo: 60 segundos

Iteração #1 - 14:30:00
═══════════════════════════════════════════════════════════
[... scan normal ...]

Aguardando 60 segundos até próximo scan...

Iteração #2 - 14:31:00
═══════════════════════════════════════════════════════════
[... scan ...]

NOVOS DISPOSITIVOS DETECTADOS:
   OK 10.67.56.200 (PC-TEMP-01) - Windows

DISPOSITIVOS OFFLINE:
   NO 10.67.56.15 (SERVER-DSM) - OFFLINE

10.67.56.25 agora responde a ping

Notificação enviada para webhook

Aguardando 60 segundos até próximo scan...
```

## JSON Export

```json
{
  "scan_time": "2026-03-05T14:35:22.123456-03:00",
  "subnet": "10.67.56-57",
  "total_devices": 45,
  "scan_duration_secs": 15.234,
  "devices": [
    {
      "ip": "10.67.56.10",
      "hostname": "PC-LAB-01",
      "responds_to_ping": true,
      "detected_ports": [445, 135, 3389],
      "primary_port": 445,
      "os_type": "Windows",
      "services": ["SMB/CIFS", "RPC/Windows", "RDP/Windows"],
      "tcp_latency_ms": 12.5,
      "icmp_latency_ms": 8.3,
      "mac_address": null,
      "first_seen": "2026-03-05T14:35:05.123456-03:00",
      "last_seen": "2026-03-05T14:35:05.123456-03:00"
    },
    {
      "ip": "10.67.56.15",
      "hostname": "SERVER-DSM",
      "responds_to_ping": false,
      "detected_ports": [22, 80],
      "primary_port": 22,
      "os_type": "Linux",
      "services": ["SSH", "HTTP"],
      "tcp_latency_ms": 45.2,
      "icmp_latency_ms": null,
      "mac_address": null,
      "first_seen": "2026-03-05T14:35:08.456789-03:00",
      "last_seen": "2026-03-05T14:35:08.456789-03:00"
    }
  ]
}
```

## CSV Export

```csv
IP,Hostname,OS,Responds_Ping,Primary_Port,TCP_Latency_ms,ICMP_Latency_ms,Services,First_Seen
10.67.56.10,PC-LAB-01,Windows,true,445,12.50,8.30,SMB/CIFS;RPC/Windows;RDP/Windows,2026-03-05 14:35:05
10.67.56.15,SERVER-DSM,Linux,false,22,45.20,N/A,SSH;HTTP,2026-03-05 14:35:08
10.67.57.1,GATEWAY-FATEC,NetworkDevice,true,22,1.20,0.80,SSH;Telnet;HTTP,2026-03-05 14:35:12
```

## Webhook Payload (Slack)

```json
{
  "text": "NetScan Completo",
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
          "text": "*Subnet:*\n10.67.56-57"
        },
        {
          "type": "mrkdwn",
          "text": "*Dispositivos:*\n45"
        },
        {
          "type": "mrkdwn",
          "text": "*Respondem Ping:*\n30"
        },
        {
          "type": "mrkdwn",
          "text": "*Duração:*\n15.2s"
        }
      ]
    },
    {
      "type": "context",
      "elements": [
        {
          "type": "mrkdwn",
          "text": "Scan realizado em: 05/03/2026 14:35:22"
        }
      ]
    }
  ]
}
```

## Código de Cores no Terminal

- **Verde** (OK): Dispositivos respondendo ping
- **Vermelho** (NO): Ping bloqueado/firewall
- **Cyan**: Informações de porta/serviço
- **Yellow**: Hostnames e warnings
- **White**: IPs e dados principais
- **Blue**: Headers e progresso

## Performance Metrics

```
Scan de 508 IPs em 15.2 segundos
Taxa: 33.4 IPs/segundo
Latência média TCP: 18.34ms
Latência média ICMP: 6.72ms
Concorrência: 100 conexões simultâneas
```

## Casos de Uso Reais

### Inventário de Rede
```bash
$ cargo run --release -- --mode full --export csv
→ Gera planilha com todos dispositivos para documentação
```

### Troubleshooting de Conectividade
```bash
$ cargo run --release -- --mode full
→ Identifica quais máquinas não pingam e porquê
```

### Monitoramento 24/7
```bash
$ cargo run --release -- --monitor --webhook https://hooks.slack.com/...
→ Alerta em tempo real sobre mudanças na rede
```

### Relatório para Gestão
```bash
$ cargo run --release -- --mode full --export html
→ HTML profissional para apresentações
```

---

**Tip**: Todos os outputs com cores funcionam melhor em terminais modernos (Windows Terminal, VS Code Terminal)
