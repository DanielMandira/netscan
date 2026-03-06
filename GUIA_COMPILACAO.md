# Guia de Compilação e Teste - NetScan Pro

## Estrutura do Projeto

```
netscan/
├── src/
│   ├── main.rs       # Core do scanner e CLI
│   ├── export.rs     # Exportação JSON/CSV/HTML
│   ├── monitor.rs    # Modo monitoramento contínuo
│   ├── web.rs        # Dashboard web server
│   └── utils.rs      # Funções auxiliares e webhooks
├── Cargo.toml        # Dependências atualizadas
└── README.md         # Documentação completa
```

## Como Compilar

### 1. Baixar Dependências
```bash
cargo build --release
```

Isso irá baixar e compilar:
- clap 4.5 (CLI parsing)
- serde + serde_json (serialização)
- csv (export CSV)
- chrono (timestamps)
- indicatif (progress bars)
- reqwest (webhooks HTTP)
- warp (web server)
- colored (terminal colorido)
- dns-lookup (hostname resolution)
- rand (randomização para stealth)
- lazy_static (variáveis globais)

### 2. Build Release (Otimizado)
```bash
cargo build --release
```

Binário estará em: `target/release/rust_net_scanner.exe`

## Exemplos de Uso

### Scan Básico
```bash
cargo run --release
```

### Scan Personalizado
```bash
# Subnet customizada
cargo run --release -- --subnet 192.168.1 --start-range 1 --end-range 1

# Modo completo com exportação
cargo run --release -- --mode full --export html

# Full port scan
cargo run --release -- --full-port-scan --concurrency 200
```

### Dashboard Web
```bash
cargo run --release -- --web --mode full
# Abra: http://localhost:8080
```

### Monitoramento Contínuo
```bash
cargo run --release -- --monitor --monitor-interval 120 --export json
```

### Com Webhook (Slack/Discord)
```bash
cargo run --release -- --mode full --webhook https://hooks.slack.com/services/YOUR/WEBHOOK
```

### Modo Stealth
```bash
cargo run --release -- --mode stealth --timeout 1000 --concurrency 20
```

## Exemplos de Saída

### Terminal
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

Configuração do Scan:
   Subnet: 10.67
   Range: 10.67.56.0 - 10.67.57.255
   Portas: 445,135,80,22,3389,443,21,23
   Modo: Quick
   Concorrência: 100
   Timeout: 400ms

[========================================] 508/508 IPs (50/s) OK 45 dispositivos encontrados

Testando conectividade ICMP e detectando serviços...

═══════════════════════════════════════════════════════════
   RESULTADOS COMPLETOS
═══════════════════════════════════════════════════════════

IP               | HOSTNAME             | OS           | PING | TCP (ms) | ICMP (ms)
---------------------------------------------------------------------------------------------------
10.67.56.10      | PC-LAB-01           | Windows      | OK   | 12.5     | 8.3
10.67.56.15      | SERVER-DSM          | Linux        | NO   | 45.2     | ---
10.67.57.50      | SWITCH-CORE         | NetworkDevice| OK   | 5.1      | 2.8

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
```

### Exportação JSON
```json
{
  "scan_time": "2026-03-05T14:30:00-03:00",
  "subnet": "10.67.56-57",
  "total_devices": 45,
  "scan_duration_secs": 15.2,
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
      "first_seen": "2026-03-05T14:30:05-03:00",
      "last_seen": "2026-03-05T14:30:05-03:00"
    }
  ]
}
```

### Dashboard Web
Acesse `http://localhost:8080` para interface visual com:
- Cards com estatísticas em tempo real
- Tabela interativa de dispositivos
- Auto-refresh a cada 10 segundos
- Design moderno com gradientes

## Troubleshooting

### Erro de Compilação
Se houver erro de dependências:
```bash
cargo clean
cargo update
cargo build --release
```

### Permissões (Windows)
Execute PowerShell como Administrador se tiver problemas de ping/scan

### Firewall Bloqueando
Windows Defender pode bloquear. Adicione exceção:
```powershell
New-NetFirewallRule -DisplayName "NetScan" -Direction Outbound -Action Allow
```

## Próximos Passos

1. **Compile o projeto**: `cargo build --release`
2. **Teste básico**: `cargo run --release`
3. **Teste exportação**: `cargo run --release -- --export html`
4. **Teste dashboard**: `cargo run --release -- --web`
5. **Configure webhook**: Use URL do Slack/Discord

## Estrutura de Código

- **main.rs**: Lógica principal, CLI parsing, scan engine
- **export.rs**: Geração de JSON/CSV/HTML
- **monitor.rs**: Loop de monitoramento e detecção de mudanças
- **web.rs**: Servidor HTTP com warp e dashboard HTML
- **utils.rs**: Webhooks e funções auxiliares

## Dicas

- Use `--mode full` para análise completa
- `--full-port-scan` é lento mas detecta mais serviços
- `--mode stealth` evita detecção por IDS
- `--monitor` é ideal para NOC/SOC
- Exporte HTML para compartilhar relatórios

## Diferencial

Este é agora um scanner de rede **profissional** com:
- Interface CLI moderna
- Múltiplos formatos de exportação
- Dashboard web em tempo real
- Monitoramento contínuo
- Integração com webhooks
- Detecção avançada de OS/serviços
- Performance otimizada

Perfeito para troubleshooting de rede, inventário, monitoramento e documentação!

---
