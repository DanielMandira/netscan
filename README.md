# NetScan Pro v2.0

Scanner de Rede Avançado desenvolvido em Rust para análise completa de redes TCP/IP com detecção de dispositivos, serviços, OS e monitoramento contínuo.

## Funcionalidades

### Core Features
- **Scan TCP Completo**: Detecta dispositivos via múltiplas portas (445, 135, 80, 22, 3389, 443, etc)
- **Teste ICMP (Ping)**: Identifica dispositivos com firewall bloqueando ping
- **Detecção de OS**: Identifica Windows, Linux e Network Devices
- **Análise de Latência**: Mede latência TCP e ICMP em tempo real
- **Resolução de Hostname**: DNS reverse lookup automático
- **Detecção de Serviços**: Identifica serviços rodando (HTTP, SSH, RDP, SMB, etc)

### Features Avançadas
- **CLI Configurável**: Argumentos flexíveis para customização total
- **Modos de Scan**: Quick, Full e Stealth
- **Scan de Portas Avançado**: Scan de 1-1024 portas
- **Barra de Progresso**: Visualização em tempo real do scan
- **Exportação**: JSON, CSV e HTML com relatórios profissionais
- **Dashboard Web**: Interface web em tempo real (localhost:8080)
- **Monitoramento Contínuo**: Detecta mudanças na rede
- **Notificações Webhook**: Integração com Slack/Discord/Teams

## Instalação

```bash
# Clone o repositório
git clone https://github.com/seu-usuario/netscan.git
cd netscan

# Build release
cargo build --release

# Executar
cargo run --release
```

## Uso Básico

### Scan Rápido (Padrão)
```bash
cargo run
```

### Scan Customizado
```bash
# Definir subnet e range
cargo run -- --subnet 192.168.1 --start-range 1 --end-range 1

# Definir portas específicas
cargo run -- --ports 22,80,443,3389,445

# Modo completo com exportação JSON
cargo run -- --mode full --export json

# Scan completo de portas
cargo run -- --full-port-scan --export html
```

## Argumentos CLI

| Argumento | Descrição | Padrão |
|-----------|-----------|--------|
| `-s, --subnet` | Subnet base (ex: 10.67) | `10.67` |
| `--start-range` | Range inicial do 3º octeto | `56` |
| `--end-range` | Range final do 3º octeto | `57` |
| `-t, --timeout` | Timeout TCP em ms | `400` |
| `-p, --ports` | Portas para scan | `445,135,80,22,3389,443,21,23` |
| `-m, --mode` | Modo: quick, full, stealth | `quick` |
| `-e, --export` | Exportar: json, csv, html | - |
| `-c, --concurrency` | Conexões simultâneas | `100` |
| `--monitor` | Modo monitoramento contínuo | - |
| `--monitor-interval` | Intervalo em segundos | `60` |
| `--web` | Ativar dashboard web | - |
| `--webhook` | URL webhook para notificações | - |
| `--full-port-scan` | Scan portas 1-1024 | - |

## Exemplos Avançados

### Monitoramento Contínuo
```bash
cargo run -- --monitor --monitor-interval 300 --export json
```

### Dashboard Web
```bash
cargo run -- --web --mode full
# Acesse: http://localhost:8080
```

### Com Notificações Webhook
```bash
cargo run -- --mode full --webhook https://hooks.slack.com/services/YOUR/WEBHOOK/URL
```

### Scan Stealth
```bash
cargo run -- --mode stealth --timeout 1000 --concurrency 20
```

## Formatos de Exportação

### JSON
Estrutura completa com todos os metadados:
```json
{
  "scan_time": "2026-03-05T10:30:00-03:00",
  "subnet": "10.67.56-57",
  "total_devices": 45,
  "devices": [...]
}
```

### CSV
Formato tabular para análise em Excel:
```csv
IP,Hostname,OS,Responds_Ping,Primary_Port,TCP_Latency_ms,ICMP_Latency_ms,Services
10.67.56.10,PC-LAB-01,Windows,true,445,12.5,8.3,SMB/CIFS
```

### HTML
Relatório visual profissional com gráficos e estatísticas

## Dashboard Web

Inicie o servidor web:
```bash
cargo run -- --web
```

Acesse `http://localhost:8080` para:
- Visualização em tempo real dos dispositivos
- Estatísticas de rede
- Tabela interativa de dispositivos
- Auto-refresh a cada 10 segundos

## 🔔 Integração Webhook

Suporta qualquer webhook compatível com JSON (Slack, Discord, Teams, etc):

```bash
cargo run -- --webhook https://discord.com/api/webhooks/YOUR_WEBHOOK
```

Payload enviado:
```json
{
  "text": "NetScan Completo",
  "blocks": [...]
}
```

## 📈 Modo Monitoramento

Detecta automaticamente:
- [OK] Novos dispositivos na rede
- [X] Dispositivos que ficaram offline
- [~] Mudanças no status de ping
- [#] Estatísticas em tempo real

```bash
cargo run -- --monitor --monitor-interval 60 --webhook YOUR_URL
```

## Detecção de OS

Baseado em portas e banner grabbing:
- **Windows**: Portas 445, 135, 3389 (SMB, RPC, RDP)
- **Linux**: Porta 22 com análise de banner SSH
- **Network Devices**: Telnet (23), banners Cisco/Huawei/Juniper

## Performance

- Scan de ~500 IPs em 10-30 segundos (modo quick)
- Até 1000 conexões simultâneas configuráveis
- Cache de DNS para otimização
- Modo stealth com delays randomizados

## Segurança

- Não é invasivo (apenas TCP connect)
- Respeita rate limiting do firewall
- Modo stealth para evitar detecção IDS
- Não explora vulnerabilidades

## Licença

MIT License - veja LICENSE para detalhes

## 👨‍💻 Autor

Desenvolvido para FATEC DSM - Análise e Troubleshooting de Redes

## 🤝 Contribuindo

PRs são bem-vindos! Para mudanças grandes, abra uma issue primeiro.

---

**AVISO**: Use apenas em redes que você tem autorização para escanear. O uso inadequado pode violar políticas de segurança.
