# ⚙️ Sciigilo

**Sciigilo** is a lightweight, self-contained system monitoring and alerting tool written in Rust. It’s designed for self-hosters, developers, and anyone who needs simple resource monitoring without complex setup or dependencies.

> ⚡ *Sciigilo* means "the one who alerts" in Esperanto. It's fast, clean, and focused — just like it should be.

## ✨ Features

- 📈 Real-time and historical system metrics (CPU, RAM, Disk)
- ⚠️ Configurable alert system (thresholds + webhooks)
- 🧱 Embedded web UI (no external files or frontend frameworks)
- 🗃️ SQLite backend with auto-cleanup (retention policy)
- 🔐 Included TLS support via `rustls`
- 🚀 Single binary, Docker-ready

## 🛠️ Getting Started

### 🔧 Build from source
```bash
cargo build --release
./target/release/sciigilo
````

### 🐳 Or run with Docker

```bash
docker run -d -p 3000:3000 andyantares/sciigilo
```

Then open: [http://localhost:3000](http://localhost:3000)

## ⚙️ Configuration

You can configure Sciigilo using environment variables or a `.env` file:

| Variable          | Description                                          | Default     |
|------------------|------------------------------------------------------|-------------|
| `DATABASE_URL`    | Path to the SQLite database file                     | `metrics.db`|
| `PORT`            | Port to run the application                          | `3000`      |
| `ALERTS_FILE`     | Path to the alert rules JSON file                    | `alerts.json` |
| `LOG_FILE`        | Path to the alert log output                         | `sciigilo.log` |
| `RETENTION_PERIOD`| Number of **days** to retain historical metric data | `1`         |

Example `.env` file:
```env
PORT=8080
DATABASE_URL=./data/metrics.db
ALERTS_FILE=./config/alerts.json
LOG_FILE=./logs/alerts.log
RETENTION_PERIOD=3
```

## 📤 Alerts

Define alert rules in JSON:

```json
{
    "id": "61247a75-8141-49c3-8505-a789f61d9f4c",
    "metric_id": "cpu_usage_percentage",
    "logic": "Gt",
    "value": "50",
    "request": {
      "request_type": "post",
      "url": "https://example.com",
      "body": {
        "format": "xwwwformurlencoded",
        "payload": "test=test"
      }
    }
  }
  
```

## 📦 Tech Stack

* 🦀 Rust + Axum + Tokio
* 📊 sysinfo for metrics
* 💽 SQLite via sqlx
* 🌐 Static HTML embedded in binary

## 📌 License

Licensed under the [GNU GPLv3](LICENSE).


