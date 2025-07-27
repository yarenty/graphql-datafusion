# Deployment Guide

## Prerequisites

### System Requirements

- Rust 1.80.0 or higher
- PostgreSQL 14+
- Redis 6+
- Node.js 18+ (for frontend)
- Docker 20+
- Docker Compose 2+
- Kubernetes (optional)

### Recommended Hardware

- CPU: 4+ cores
- RAM: 8GB+
- Storage: 100GB+
- Network: 1Gbps+

## Deployment Options

### Docker Compose

```yaml
version: '3'
services:
  graphql-datafusion:
    build: .
    ports:
      - "8000:8000"          # GraphQL API
      - "8001:8001"          # WebSocket
      - "9090:9090"          # Prometheus
    environment:
      - AGENT_API_URL=https://api.x.ai/grok
      - AGENT_API_KEY=${AGENT_API_KEY}
      - JWT_SECRET=${JWT_SECRET}
      - DATABASE_URL=postgresql://datafusion:password@db/datafusion
      - CACHE_URL=redis://cache:6379
      - LOG_LEVEL=info
      - RUST_LOG=info
      - RUST_TRACING=info
    depends_on:
      - db
      - cache
      - prometheus

  db:
    image: postgres:14
    ports:
      - "5432:5432"
    environment:
      - POSTGRES_USER=datafusion
      - POSTGRES_PASSWORD=password
      - POSTGRES_DB=datafusion
    volumes:
      - postgres_data:/var/lib/postgresql/data

  cache:
    image: redis:6
    ports:
      - "6379:6379"
    volumes:
      - redis_data:/data

  prometheus:
    image: prom/prometheus:latest
    ports:
      - "9090:9090"
    volumes:
      - ./prometheus/prometheus.yml:/etc/prometheus/prometheus.yml
      - prometheus_data:/prometheus

  grafana:
    image: grafana/grafana:latest
    ports:
      - "3000:3000"
    environment:
      - GF_SECURITY_ADMIN_PASSWORD=${GRAFANA_PASSWORD}
    volumes:
      - grafana_data:/var/lib/grafana

volumes:
  postgres_data:
  redis_data:
  prometheus_data:
  grafana_data:
```

### Kubernetes

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: graphql-datafusion
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
        - containerPort: 8000
        - containerPort: 8001
        - containerPort: 9090
        env:
        - name: AGENT_API_URL
          value: "https://api.x.ai/grok"
        - name: AGENT_API_KEY
          valueFrom:
            secretKeyRef:
              name: datafusion-secrets
              key: agent_api_key
        - name: JWT_SECRET
          valueFrom:
            secretKeyRef:
              name: datafusion-secrets
              key: jwt_secret
        - name: DATABASE_URL
          valueFrom:
            secretKeyRef:
              name: datafusion-secrets
              key: database_url
        - name: CACHE_URL
          valueFrom:
            secretKeyRef:
              name: datafusion-secrets
              key: cache_url
        - name: LOG_LEVEL
          value: "info"
        resources:
          limits:
            memory: "2Gi"
            cpu: "2"
          requests:
            memory: "1Gi"
            cpu: "1"

---

apiVersion: v1
kind: Service
metadata:
  name: graphql-datafusion
spec:
  selector:
    app: graphql-datafusion
  ports:
  - port: 8000
    targetPort: 8000
    name: http
  - port: 8001
    targetPort: 8001
    name: websocket
  - port: 9090
    targetPort: 9090
    name: metrics
  type: LoadBalancer
```

## Production Configuration

### Environment Variables

```bash
# Security
JWT_SECRET="production-secret-key"
AGENT_API_KEY="production-api-key"

# Performance
CACHE_TTL=3600
CACHE_SIZE=1000000

# Monitoring
METRICS_ENABLED=true
METRICS_PORT=9090

# Connection Pooling
DATABASE_POOL_SIZE=10
CACHE_POOL_SIZE=5
AGENT_POOL_SIZE=3

# Rate Limiting
RATE_LIMIT_WINDOW=60
RATE_LIMIT_COUNT=1000
BURST_LIMIT=10
```

### Logging

```yaml
version: 1
formatters:
  default:
    format: '%(asctime)s - %(name)s - %(levelname)s - %(message)s'

handlers:
  console:
    class: logging.StreamHandler
    formatter: default
    level: info
    stream: ext://sys.stdout

  file:
    class: logging.handlers.RotatingFileHandler
    formatter: default
    level: info
    filename: logs/app.log
    maxBytes: 10485760
    backupCount: 10

  error:
    class: logging.handlers.RotatingFileHandler
    formatter: default
    level: error
    filename: logs/error.log
    maxBytes: 10485760
    backupCount: 10

loggers:
  root:
    level: info
    handlers: [console, file, error]

  datafusion:
    level: info
    handlers: [console, file, error]

  agents:
    level: info
    handlers: [console, file, error]
```

### Monitoring

#### Prometheus Configuration

```yaml
scrape_configs:
  - job_name: 'graphql-datafusion'
    static_configs:
      - targets: ['localhost:9090']

  - job_name: 'agent-metrics'
    static_configs:
      - targets: ['localhost:9091']

  - job_name: 'database'
    static_configs:
      - targets: ['localhost:5432']

  - job_name: 'redis'
    static_configs:
      - targets: ['localhost:6379']
```

#### Grafana Dashboard

Create a dashboard with panels for:

1. Request Metrics
   - Request rate
   - Error rate
   - Response time
   - Cache hit rate

2. System Metrics
   - CPU usage
   - Memory usage
   - Disk usage
   - Network traffic

3. Database Metrics
   - Connection pool
   - Query rate
   - Slow queries
   - Cache metrics

### Security

1. Use HTTPS
2. Configure proper CORS
3. Enable rate limiting
4. Use secure JWT secrets
5. Enable security headers
6. Configure proper logging
7. Set up monitoring alerts
8. Regular security audits

### Backup Strategy

1. Database
   - Daily backups
   - Retention: 30 days
   - Encryption
   - Offsite storage

2. Cache
   - Periodic snapshots
   - Retention: 7 days
   - Compression
   - Offsite storage

3. Configuration
   - Version control
   - Backup
   - Audit logs

### Scaling Strategy

1. Horizontal Scaling
   - Add more replicas
   - Load balancing
   - Session management

2. Vertical Scaling
   - Increase resources
   - Optimize queries
   - Cache optimization

3. Auto-scaling
   - CPU-based
   - Memory-based
   - Request rate-based

### Disaster Recovery

1. Backup Plan
   - Regular backups
   - Offsite storage
   - Encryption
   - Verification

2. Recovery Plan
   - Restore procedure
   - Data validation
   - Testing
   - Documentation

3. Monitoring
   - Backup status
   - Recovery time
   - Data integrity

## Maintenance

### Regular Tasks

1. Log Rotation
2. Backup Verification
3. Performance Monitoring
4. Security Updates
5. Dependency Updates
6. Configuration Review

### Monitoring Alerts

1. Error rate thresholds
2. Response time warnings
3. Resource usage alerts
4. Backup failure notifications
5. Security violation alerts

### Performance Optimization

1. Query optimization
2. Cache tuning
3. Connection pooling
4. Resource allocation
5. Load testing

## Troubleshooting Guide

### Common Issues

1. **Connection Issues**
   - Check network connectivity
   - Verify ports are open
   - Check firewall rules
   - Verify service status

2. **Performance Issues**
   - Monitor resource usage
   - Check query performance
   - Review cache hits/misses
   - Check connection pool

3. **Security Issues**
   - Verify JWT token validity
   - Check rate limits
   - Review security headers
   - Monitor suspicious activity

4. **Data Issues**
   - Verify data integrity
   - Check database connections
   - Review query results
   - Check cache status

### Debugging Steps

1. Check logs
2. Monitor metrics
3. Review configuration
4. Test connections
5. Check resource usage
6. Review error messages

### Recovery Procedures

1. **Service Recovery**
   - Restart service
   - Check dependencies
   - Verify configuration
   - Monitor restart

2. **Data Recovery**
   - Restore from backup
   - Verify data
   - Test connections
   - Monitor performance

3. **Security Recovery**
   - Rotate secrets
   - Update configuration
   - Review access logs
   - Monitor activity
