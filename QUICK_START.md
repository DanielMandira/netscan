# Quick Start - NetScan Pro

## Compilar
```bash
cargo build --release
```

## Comandos Mais Usados

### 1. Scan Básico (Default: 10.67.56-57)
```bash
cargo run --release
```

### 2. Scan Subnet Customizada
```bash
cargo run --release -- --subnet 192.168.1 --start-range 1 --end-range 1
```

### 3. Modo Completo + Exportar HTML
```bash
cargo run --release -- --mode full --export html
```

### 4. Dashboard Web
```bash
cargo run --release -- --web
# Abra http://localhost:8080
```

### 5. Monitoramento 24/7
```bash
cargo run --release -- --monitor --monitor-interval 300 --export json
```

### 6. Full Port Scan
```bash
cargo run --release -- --full-port-scan --timeout 200 --concurrency 200
```

### 7. Com Webhook (Slack/Discord)
```bash
cargo run --release -- --webhook https://your-webhook-url --export json
```

### 8. Modo Stealth (Evitar IDS)
```bash
cargo run --release -- --mode stealth --concurrency 20
```

## Ver Ajuda Completa
```bash
cargo run --release -- --help
```

## Casos de Uso

| Situação | Comando |
|----------|---------|
| Inventário rápido | `cargo run --release` |
| Documentação | `cargo run --release -- --mode full --export html` |
| Troubleshooting | `cargo run --release -- --mode full --timeout 200` |
| Monitoramento | `cargo run --release -- --monitor --webhook URL` |
| Scan completo | `cargo run --release -- --full-port-scan --export json` |

