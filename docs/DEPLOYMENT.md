# Deployment Guide

## 🚀 Overview

This guide covers deploying the GraphQL DataFusion API in various environments, from local development to production clusters.

## 📋 Prerequisites

### System Requirements

- **OS**: Linux (Ubuntu 20.04+, CentOS 8+), macOS 10.15+, Windows 10+
- **CPU**: 2+ cores (4+ recommended for production)
- **Memory**: 4GB+ RAM (8GB+ recommended for production)
- **Storage**: 10GB+ available space
- **Network**: Internet access for dependencies

### Software Dependencies

- **Rust**: 1.80.0+ (automatically managed via rust-toolchain.toml)
- **Docker**: 20.10+ (for containerized deployment)
- **Ollama**: Latest version (for AI integration)
- **Git**: For source code management

## 🔧 Local Development Setup

### Quick Start

```bash
# Clone the repository
git clone <repository-url>
cd graphql-datafusion

# Install Rust (if not already installed)
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source ~/.cargo/env

# Build the project
cargo build --release

# Create data directory
mkdir -p data

# Add sample data files
# (CSV, Parquet, JSON files will be automatically discovered)

# Start the server
cargo run --release
```

### Development Environment

```bash
# Install development dependencies
cargo install cargo-watch
cargo install cargo-audit

# Run with hot reload
cargo watch -x run

# Run tests
cargo test

# Check for security vulnerabilities
cargo audit
```

### Environment Configuration

Create a `.env` file for local development:

```bash
# .env
DATA_PATH=./data
OLLAMA_BASE_URL=http://localhost:11434
OLLAMA_MODEL=llama2
SERVER_PORT=8080
RUST_LOG=debug
DEBUG_MODE=true
```

## 🐳 Docker Deployment

### Single Container

```dockerfile
# Dockerfile
FROM rust:1.80.0 AS builder

WORKDIR /app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim

RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/graphql-datafusion .

EXPOSE 8080

CMD ["./graphql-datafusion"]
```

### Docker Compose

```yaml
# docker-compose.yml
version: '3.8'

services:
  graphql-datafusion:
    build: .
    ports:
      - "8080:8080"
    volumes:
      - ./data:/data
      - ./config:/app/config
    environment:
      - DATA_PATH=/data
      - OLLAMA_BASE_URL=http://ollama:11434
      - RUST_LOG=info
    depends_on:
      - ollama
    restart: unless-stopped

  ollama:
    image: ollama/ollama:latest
    ports:
      - "11434:11434"
    volumes:
      - ollama_data:/root/.ollama
    restart: unless-stopped

volumes:
  ollama_data:
```

### Multi-Stage Production Build

```dockerfile
# Dockerfile.prod
FROM rust:1.80.0 AS builder

WORKDIR /app
COPY . .

# Install build dependencies
RUN apt-get update && apt-get install -y \
    pkg-config \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Build with optimizations
RUN cargo build --release --bin graphql-datafusion

FROM debian:bookworm-slim AS runtime

# Install runtime dependencies
RUN apt-get update && apt-get install -y \
    ca-certificates \
    libssl3 \
    && rm -rf /var/lib/apt/lists/*

# Create non-root user
RUN useradd -r -s /bin/false app

WORKDIR /app
COPY --from=builder /app/target/release/graphql-datafusion .

# Create data directory
RUN mkdir -p /data && chown app:app /data

USER app

EXPOSE 8080

HEALTHCHECK --interval=30s --timeout=3s --start-period=5s --retries=3 \
    CMD curl -f http://localhost:8080/health || exit 1

CMD ["./graphql-datafusion"]
```

## ☁️ Cloud Deployment

### AWS Deployment

#### ECS with Fargate

```yaml
# task-definition.json
{
  "family": "graphql-datafusion",
  "networkMode": "awsvpc",
  "requiresCompatibilities": ["FARGATE"],
  "cpu": "1024",
  "memory": "2048",
  "executionRoleArn": "arn:aws:iam::account:role/ecsTaskExecutionRole",
  "taskRoleArn": "arn:aws:iam::account:role/ecsTaskRole",
  "containerDefinitions": [
    {
      "name": "graphql-datafusion",
      "image": "your-registry/graphql-datafusion:latest",
      "portMappings": [
        {
          "containerPort": 8080,
          "protocol": "tcp"
        }
      ],
      "environment": [
        {
          "name": "DATA_PATH",
          "value": "/data"
        },
        {
          "name": "OLLAMA_BASE_URL",
          "value": "http://ollama:11434"
        }
      ],
      "mountPoints": [
        {
          "sourceVolume": "data",
          "containerPath": "/data",
          "readOnly": false
        }
      ],
      "logConfiguration": {
        "logDriver": "awslogs",
        "options": {
          "awslogs-group": "/ecs/graphql-datafusion",
          "awslogs-region": "us-west-2",
          "awslogs-stream-prefix": "ecs"
        }
      }
    }
  ],
  "volumes": [
    {
      "name": "data",
      "efsVolumeConfiguration": {
        "fileSystemId": "fs-12345678",
        "rootDirectory": "/",
        "transitEncryption": "ENABLED"
      }
    }
  ]
}
```

#### EKS Deployment

```yaml
# k8s-deployment.yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: graphql-datafusion
  labels:
    app: graphql-datafusion
spec:
  replicas: 3
  selector:
    matchLabels:
      app: graphql-datafusion
  template:
    metadata:
      labels:
        app: graphql-datafusion
    spec:
      containers:
      - name: graphql-datafusion
        image: your-registry/graphql-datafusion:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATA_PATH
          value: "/data"
        - name: OLLAMA_BASE_URL
          value: "http://ollama-service:11434"
        volumeMounts:
        - name: data-volume
          mountPath: /data
        resources:
          requests:
            memory: "1Gi"
            cpu: "500m"
          limits:
            memory: "2Gi"
            cpu: "1000m"
        livenessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 30
          periodSeconds: 10
        readinessProbe:
          httpGet:
            path: /health
            port: 8080
          initialDelaySeconds: 5
          periodSeconds: 5
      volumes:
      - name: data-volume
        persistentVolumeClaim:
          claimName: data-pvc
---
apiVersion: v1
kind: Service
metadata:
  name: graphql-datafusion-service
spec:
  selector:
    app: graphql-datafusion
  ports:
  - protocol: TCP
    port: 80
    targetPort: 8080
  type: LoadBalancer
```

### Google Cloud Platform

#### Cloud Run

```yaml
# cloud-run.yaml
apiVersion: serving.knative.dev/v1
kind: Service
metadata:
  name: graphql-datafusion
spec:
  template:
    metadata:
      annotations:
        autoscaling.knative.dev/minScale: "1"
        autoscaling.knative.dev/maxScale: "10"
    spec:
      containerConcurrency: 80
      timeoutSeconds: 300
      containers:
      - image: gcr.io/your-project/graphql-datafusion:latest
        ports:
        - containerPort: 8080
        env:
        - name: DATA_PATH
          value: "/data"
        - name: OLLAMA_BASE_URL
          value: "http://ollama-service:11434"
        resources:
          limits:
            cpu: "1000m"
            memory: "2Gi"
          requests:
            cpu: "500m"
            memory: "1Gi"
```

### Azure

#### Azure Container Instances

```yaml
# azure-container-instance.yaml
apiVersion: 2021-10-01
location: eastus
name: graphql-datafusion
properties:
  containers:
  - name: graphql-datafusion
    properties:
      image: your-registry.azurecr.io/graphql-datafusion:latest
      ports:
      - port: 8080
      environmentVariables:
      - name: DATA_PATH
        value: "/data"
      - name: OLLAMA_BASE_URL
        value: "http://ollama-service:11434"
      resources:
        requests:
          cpu: 1.0
          memoryInGB: 2.0
        limits:
          cpu: 2.0
          memoryInGB: 4.0
      volumeMounts:
      - name: data-volume
        mountPath: /data
  volumes:
  - name: data-volume
    properties:
      azureFile:
        shareName: data
        storageAccountName: yourstorageaccount
        storageAccountKey: your-storage-account-key
  osType: Linux
  restartPolicy: Always
  ipAddress:
    type: Public
    ports:
    - port: 8080
      protocol: TCP
```

## 🔒 Production Security

### SSL/TLS Configuration

```nginx
# nginx.conf
server {
    listen 443 ssl http2;
    server_name your-domain.com;

    ssl_certificate /etc/ssl/certs/graphql-datafusion.crt;
    ssl_certificate_key /etc/ssl/private/graphql-datafusion.key;
    ssl_protocols TLSv1.2 TLSv1.3;
    ssl_ciphers ECDHE-RSA-AES256-GCM-SHA512:DHE-RSA-AES256-GCM-SHA512;
    ssl_prefer_server_ciphers off;

    location / {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
    }
}
```

### Security Headers

```rust
// Add security headers middleware
use actix_web::middleware::{DefaultHeaders, Logger};

pub fn configure_security(app: &mut web::ServiceConfig) {
    app.wrap(
        DefaultHeaders::new()
            .add(("X-Content-Type-Options", "nosniff"))
            .add(("X-Frame-Options", "DENY"))
            .add(("X-XSS-Protection", "1; mode=block"))
            .add(("Strict-Transport-Security", "max-age=31536000; includeSubDomains"))
    );
}
```

### Network Security

```bash
# Firewall rules (UFW)
sudo ufw allow 22/tcp
sudo ufw allow 80/tcp
sudo ufw allow 443/tcp
sudo ufw deny 8080/tcp  # Only allow through reverse proxy
sudo ufw enable
```

## 📊 Monitoring and Observability

### Prometheus Metrics

```yaml
# prometheus.yml
global:
  scrape_interval: 15s

scrape_configs:
  - job_name: 'graphql-datafusion'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/metrics'
    scrape_interval: 5s
```

### Grafana Dashboard

```json
{
  "dashboard": {
    "title": "GraphQL DataFusion Metrics",
    "panels": [
      {
        "title": "Request Rate",
        "type": "graph",
        "targets": [
          {
            "expr": "rate(http_requests_total[5m])",
            "legendFormat": "{{method}} {{endpoint}}"
          }
        ]
      },
      {
        "title": "Response Time",
        "type": "graph",
        "targets": [
          {
            "expr": "histogram_quantile(0.95, rate(http_request_duration_seconds_bucket[5m]))",
            "legendFormat": "95th percentile"
          }
        ]
      }
    ]
  }
}
```

### Health Checks

```rust
// Health check endpoint
async fn health_check() -> impl Responder {
    let mut status = HashMap::new();
    
    // Check data directory
    status.insert("data_directory", std::path::Path::new("/data").exists());
    
    // Check Ollama connection
    status.insert("ollama", check_ollama_connection().await);
    
    // Check memory usage
    status.insert("memory_ok", get_memory_usage() < 0.9);
    
    HttpResponse::Ok().json(status)
}
```

## 🔄 CI/CD Pipeline

### GitHub Actions

```yaml
# .github/workflows/deploy.yml
name: Deploy to Production

on:
  push:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - uses: actions-rs/toolchain@v1
      with:
        toolchain: 1.80.0
    - run: cargo test
    - run: cargo audit

  build:
    needs: test
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Build Docker image
      run: docker build -t graphql-datafusion .
    - name: Push to registry
      run: |
        echo ${{ secrets.REGISTRY_PASSWORD }} | docker login -u ${{ secrets.REGISTRY_USERNAME }} --password-stdin
        docker tag graphql-datafusion ${{ secrets.REGISTRY_URL }}/graphql-datafusion:${{ github.sha }}
        docker push ${{ secrets.REGISTRY_URL }}/graphql-datafusion:${{ github.sha }}

  deploy:
    needs: build
    runs-on: ubuntu-latest
    steps:
    - name: Deploy to production
      run: |
        # Deploy to your cloud platform
        kubectl set image deployment/graphql-datafusion graphql-datafusion=${{ secrets.REGISTRY_URL }}/graphql-datafusion:${{ github.sha }}
```

### GitLab CI

```yaml
# .gitlab-ci.yml
stages:
  - test
  - build
  - deploy

test:
  stage: test
  image: rust:1.80.0
  script:
    - cargo test
    - cargo audit

build:
  stage: build
  image: docker:latest
  services:
    - docker:dind
  script:
    - docker build -t graphql-datafusion .
    - docker tag graphql-datafusion $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
    - docker push $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA

deploy:
  stage: deploy
  script:
    - kubectl set image deployment/graphql-datafusion graphql-datafusion=$CI_REGISTRY_IMAGE:$CI_COMMIT_SHA
```

## 🔧 Troubleshooting

### Common Issues

#### High Memory Usage
```bash
# Check memory usage
free -h
ps aux --sort=-%mem | head -10

# Increase memory limit
export DATAFUSION_MEMORY_LIMIT=2147483648  # 2GB
```

#### Slow Query Performance
```bash
# Enable query logging
export RUST_LOG=debug

# Check DataFusion metrics
curl http://localhost:8080/metrics | grep datafusion
```

#### Ollama Connection Issues
```bash
# Check Ollama service
curl http://localhost:11434/api/tags

# Restart Ollama
sudo systemctl restart ollama
```

### Log Analysis

```bash
# View logs
journalctl -u graphql-datafusion -f

# Search for errors
grep -i error /var/log/graphql-datafusion.log

# Monitor real-time logs
tail -f /var/log/graphql-datafusion.log | grep -E "(ERROR|WARN)"
```

## 🔗 Related Documentation

- [API Documentation](API.md) - Complete API reference
- [Configuration Guide](CONFIGURATION.md) - Configuration options
- [Troubleshooting Guide](TROUBLESHOOTING.md) - Common issues and solutions
