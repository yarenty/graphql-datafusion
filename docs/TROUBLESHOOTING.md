# Troubleshooting Guide

## 🚀 Overview

This guide helps you diagnose and resolve common issues with the GraphQL DataFusion API. It covers data discovery problems, AI integration issues, performance problems, and deployment challenges.

## 🔍 Quick Diagnostic Commands

### System Health Check

```bash
# Check if the server is running
curl -f http://localhost:8080/health

# Check available tables
curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ tables }"}'

# Check Ollama connection
curl http://localhost:11434/api/tags

# Check system resources
free -h
df -h
ps aux --sort=-%mem | head -5
```

### Log Analysis

```bash
# View real-time logs
tail -f /var/log/graphql-datafusion.log

# Search for errors
grep -i error /var/log/graphql-datafusion.log

# Search for warnings
grep -i warn /var/log/graphql-datafusion.log

# Check recent activity
tail -100 /var/log/graphql-datafusion.log
```

## 📊 Data Discovery Issues

### No Tables Found

**Problem**: The `tables` query returns an empty array.

**Symptoms**:
```json
{
  "data": {
    "tables": []
  }
}
```

**Diagnosis**:
```bash
# Check if data directory exists
ls -la /path/to/data/directory

# Check if files are readable
ls -la /path/to/data/directory/*.csv
ls -la /path/to/data/directory/*.parquet

# Check file permissions
stat /path/to/data/directory/sample.csv
```

**Solutions**:

1. **Verify data directory path**:
   ```bash
   # Check configuration
   echo $DATA_PATH
   
   # Create data directory if missing
   mkdir -p /path/to/data/directory
   ```

2. **Add sample data files**:
   ```bash
   # Create sample CSV file
   cat > /path/to/data/directory/sample.csv << EOF
   id,name,value
   1,Alice,100
   2,Bob,200
   3,Charlie,300
   EOF
   ```

3. **Check file format support**:
   ```bash
   # Verify file extension
   file /path/to/data/directory/sample.csv
   
   # Check if file is valid CSV
   head -5 /path/to/data/directory/sample.csv
   ```

### Schema Inference Errors

**Problem**: Tables are found but schema inference fails.

**Symptoms**:
```json
{
  "errors": [
    {
      "message": "Failed to infer schema for table 'sample'",
      "path": ["table_schema"]
    }
  ]
}
```

**Diagnosis**:
```bash
# Check file content
head -10 /path/to/data/directory/sample.csv

# Check for encoding issues
file -i /path/to/data/directory/sample.csv

# Check for malformed data
awk -F',' 'NF!=3' /path/to/data/directory/sample.csv
```

**Solutions**:

1. **Fix CSV formatting**:
   ```bash
   # Ensure proper CSV format with headers
   cat > /path/to/data/directory/sample.csv << EOF
   id,name,value
   1,Alice,100
   2,Bob,200
   3,Charlie,300
   EOF
   ```

2. **Handle encoding issues**:
   ```bash
   # Convert to UTF-8 if needed
   iconv -f ISO-8859-1 -t UTF-8 sample.csv > sample_utf8.csv
   ```

3. **Fix data type issues**:
   ```bash
   # Ensure consistent data types
   # Numbers should be numeric, dates should be consistent format
   ```

### Large File Issues

**Problem**: Large files cause memory or performance issues.

**Symptoms**:
- High memory usage
- Slow query responses
- Out of memory errors

**Diagnosis**:
```bash
# Check file sizes
du -h /path/to/data/directory/*

# Check memory usage
free -h
ps aux --sort=-%mem | head -5
```

**Solutions**:

1. **Increase memory limits**:
   ```bash
   export DATAFUSION_MEMORY_LIMIT=4294967296  # 4GB
   export DATAFUSION_BATCH_SIZE=16384
   ```

2. **Use Parquet format**:
   ```bash
   # Convert CSV to Parquet for better performance
   python -c "
   import pandas as pd
   df = pd.read_csv('large_file.csv')
   df.to_parquet('large_file.parquet', compression='snappy')
   "
   ```

3. **Partition large files**:
   ```bash
   # Split large CSV files
   split -l 100000 large_file.csv large_file_part_
   ```

## 🤖 AI Integration Issues

### Ollama Connection Problems

**Problem**: AI queries fail with connection errors.

**Symptoms**:
```json
{
  "errors": [
    {
      "message": "Failed to connect to Ollama service",
      "path": ["naturalLanguageQuery"]
    }
  ]
}
```

**Diagnosis**:
```bash
# Check if Ollama is running
curl -f http://localhost:11434/api/tags

# Check Ollama service status
sudo systemctl status ollama

# Check network connectivity
telnet localhost 11434
```

**Solutions**:

1. **Start Ollama service**:
   ```bash
   # Install Ollama if not installed
   curl -fsSL https://ollama.ai/install.sh | sh
   
   # Start Ollama service
   sudo systemctl start ollama
   sudo systemctl enable ollama
   ```

2. **Pull required model**:
   ```bash
   # Pull a model
   ollama pull llama2
   
   # List available models
   ollama list
   ```

3. **Check Ollama configuration**:
   ```bash
   # Verify Ollama URL in configuration
   echo $OLLAMA_BASE_URL
   
   # Test Ollama API directly
   curl -X POST http://localhost:11434/api/generate \
     -H "Content-Type: application/json" \
     -d '{"model": "llama2", "prompt": "Hello"}'
   ```

### Model Loading Issues

**Problem**: Ollama is running but models fail to load.

**Symptoms**:
- Long response times
- Model not found errors
- Out of memory errors

**Diagnosis**:
```bash
# Check available models
ollama list

# Check model status
ollama show llama2

# Check system resources
free -h
nvidia-smi  # If using GPU
```

**Solutions**:

1. **Use smaller model**:
   ```bash
   # Pull smaller model
   ollama pull llama2:7b
   
   # Update configuration
   export OLLAMA_MODEL=llama2:7b
   ```

2. **Increase system resources**:
   ```bash
   # Allocate more memory to Ollama
   export OLLAMA_HOST=0.0.0.0:11434
   export OLLAMA_ORIGINS=*
   ```

3. **Use CPU-only mode**:
   ```bash
   # Force CPU usage
   export CUDA_VISIBLE_DEVICES=""
   ollama pull llama2:7b
   ```

### AI Response Quality Issues

**Problem**: AI-generated SQL or insights are poor quality.

**Symptoms**:
- Incorrect SQL syntax
- Irrelevant insights
- Inconsistent responses

**Solutions**:

1. **Improve prompts**:
   ```bash
   # Update prompt templates in configuration
   export NATURAL_LANGUAGE_PROMPT="You are a SQL expert. Convert this query to SQL for DataFusion: {query}"
   export INSIGHTS_PROMPT="You are a data analyst. Provide business insights for: {data}"
   ```

2. **Use better model**:
   ```bash
   # Try different models
   ollama pull codellama:7b
   ollama pull mistral:7b
   
   # Update configuration
   export OLLAMA_MODEL=codellama:7b
   ```

3. **Provide context**:
   ```bash
   # Include schema information in prompts
   export NATURAL_LANGUAGE_PROMPT="Available tables: {tables}. Convert to SQL: {query}"
   ```

## ⚡ Performance Issues

### Slow Query Responses

**Problem**: Queries take too long to execute.

**Symptoms**:
- Response times > 5 seconds
- High CPU usage
- Memory pressure

**Diagnosis**:
```bash
# Check query performance
time curl -X POST http://localhost:8080/graphql \
  -H "Content-Type: application/json" \
  -d '{"query": "{ tables }"}'

# Monitor system resources
htop
iotop
```

**Solutions**:

1. **Optimize DataFusion settings**:
   ```bash
   # Increase batch size
   export DATAFUSION_BATCH_SIZE=32768
   
   # Enable optimizations
   export DATAFUSION_ENABLE_OPTIMIZATION=true
   ```

2. **Use indexing**:
   ```bash
   # Convert to Parquet for better performance
   # Parquet files are automatically indexed
   ```

3. **Implement caching**:
   ```bash
   # Enable query result caching
   export CACHE_ENABLED=true
   export CACHE_TTL=3600
   ```

### High Memory Usage

**Problem**: Application uses too much memory.

**Symptoms**:
- Out of memory errors
- System slowdown
- Process killed by OOM killer

**Diagnosis**:
```bash
# Check memory usage
free -h
ps aux --sort=-%mem | head -10

# Check memory limits
cat /proc/$(pgrep graphql-datafusion)/limits
```

**Solutions**:

1. **Limit memory usage**:
   ```bash
   # Set memory limits
   export DATAFUSION_MEMORY_LIMIT=1073741824  # 1GB
   export MAX_FILE_SIZE=536870912  # 512MB
   ```

2. **Optimize data loading**:
   ```bash
   # Use streaming for large files
   export STREAMING_ENABLED=true
   ```

3. **Restart with more memory**:
   ```bash
   # Restart with increased memory
   systemctl restart graphql-datafusion
   ```

## 🌐 Network and Connectivity Issues

### Port Already in Use

**Problem**: Server fails to start due to port conflicts.

**Symptoms**:
```
Error: Address already in use (os error 98)
```

**Diagnosis**:
```bash
# Check what's using the port
sudo netstat -tlnp | grep :8080
sudo lsof -i :8080
```

**Solutions**:

1. **Kill conflicting process**:
   ```bash
   # Find and kill process
   sudo kill -9 $(lsof -t -i:8080)
   ```

2. **Use different port**:
   ```bash
   # Change port in configuration
   export SERVER_PORT=8081
   ```

3. **Check for zombie processes**:
   ```bash
   # Clean up processes
   pkill -f graphql-datafusion
   ```

### CORS Issues

**Problem**: Browser blocks requests due to CORS policy.

**Symptoms**:
```
Access to fetch at 'http://localhost:8080/graphql' from origin 'http://localhost:3000' has been blocked by CORS policy
```

**Solutions**:

1. **Configure CORS**:
   ```bash
   # Allow all origins (development)
   export CORS_ALLOW_ORIGIN=*
   
   # Allow specific origins (production)
   export CORS_ALLOW_ORIGIN=http://localhost:3000,https://yourdomain.com
   ```

2. **Use proxy in development**:
   ```javascript
   // In your frontend development server
   proxy: {
     '/graphql': 'http://localhost:8080'
   }
   ```

## 🔧 Configuration Issues

### Environment Variable Problems

**Problem**: Configuration not applied correctly.

**Diagnosis**:
```bash
# Check environment variables
env | grep -E "(DATA|OLLAMA|SERVER)"

# Check configuration file
cat /etc/graphql-datafusion/config.toml
```

**Solutions**:

1. **Verify environment variables**:
   ```bash
   # Set required variables
   export DATA_PATH=/path/to/data
   export OLLAMA_BASE_URL=http://localhost:11434
   export SERVER_PORT=8080
   ```

2. **Use configuration file**:
   ```toml
   # config.toml
   data_path = "/path/to/data"
   ollama_base_url = "http://localhost:11434"
   server_port = 8080
   ```

3. **Restart service**:
   ```bash
   # Restart to apply changes
   systemctl restart graphql-datafusion
   ```

### File Permission Issues

**Problem**: Application cannot read data files or write logs.

**Symptoms**:
- Permission denied errors
- Cannot access data directory
- Log files not created

**Diagnosis**:
```bash
# Check file permissions
ls -la /path/to/data/directory/
ls -la /var/log/graphql-datafusion.log

# Check user permissions
whoami
groups
```

**Solutions**:

1. **Fix file permissions**:
   ```bash
   # Make data directory readable
   sudo chmod 755 /path/to/data/directory
   sudo chown -R $USER:$USER /path/to/data/directory
   ```

2. **Fix log permissions**:
   ```bash
   # Create log directory
   sudo mkdir -p /var/log/graphql-datafusion
   sudo chown $USER:$USER /var/log/graphql-datafusion
   ```

3. **Run with proper user**:
   ```bash
   # Run as specific user
   sudo -u graphql-datafusion ./graphql-datafusion
   ```

## 🐳 Docker Issues

### Container Won't Start

**Problem**: Docker container fails to start or crashes immediately.

**Diagnosis**:
```bash
# Check container logs
docker logs graphql-datafusion

# Check container status
docker ps -a

# Check resource usage
docker stats graphql-datafusion
```

**Solutions**:

1. **Check resource limits**:
   ```bash
   # Increase memory limit
   docker run --memory=2g graphql-datafusion
   ```

2. **Mount data directory**:
   ```bash
   # Mount data directory
   docker run -v /host/data:/data graphql-datafusion
   ```

3. **Check port conflicts**:
   ```bash
   # Use different port
   docker run -p 8081:8080 graphql-datafusion
   ```

### Volume Mount Issues

**Problem**: Data files not accessible inside container.

**Symptoms**:
- Empty tables list
- File not found errors

**Diagnosis**:
```bash
# Check volume mounts
docker inspect graphql-datafusion | grep -A 10 Mounts

# Check files inside container
docker exec graphql-datafusion ls -la /data
```

**Solutions**:

1. **Fix volume permissions**:
   ```bash
   # Set proper permissions on host
   sudo chmod 755 /host/data
   sudo chown 1000:1000 /host/data
   ```

2. **Use named volumes**:
   ```yaml
   # docker-compose.yml
   volumes:
     - data_volume:/data
   ```

## 📊 Monitoring and Debugging

### Enable Debug Logging

```bash
# Set debug level
export RUST_LOG=debug
export LOG_LEVEL=debug

# Restart service
systemctl restart graphql-datafusion

# Monitor logs
tail -f /var/log/graphql-datafusion.log
```

### Performance Profiling

```bash
# Install profiling tools
sudo apt-get install perf-tools-unstable

# Profile CPU usage
perf record -g -p $(pgrep graphql-datafusion)
perf report

# Monitor system calls
strace -p $(pgrep graphql-datafusion)
```

### Memory Profiling

```bash
# Monitor memory usage
watch -n 1 'ps aux | grep graphql-datafusion'

# Check memory leaks
valgrind --tool=memcheck --leak-check=full ./graphql-datafusion
```

## 🆘 Getting Help

### Collecting Debug Information

```bash
# Create debug report
cat > debug_report.txt << EOF
=== System Information ===
$(uname -a)
$(cat /etc/os-release)

=== Application Status ===
$(systemctl status graphql-datafusion)

=== Configuration ===
$(env | grep -E "(DATA|OLLAMA|SERVER)")

=== Logs ===
$(tail -100 /var/log/graphql-datafusion.log)

=== Resource Usage ===
$(free -h)
$(df -h)
EOF
```

### Common Error Messages

| Error | Cause | Solution |
|-------|-------|----------|
| `Address already in use` | Port conflict | Kill conflicting process or change port |
| `Permission denied` | File permissions | Fix file/directory permissions |
| `Connection refused` | Service not running | Start Ollama or check network |
| `Out of memory` | Insufficient RAM | Increase memory limits or optimize data |
| `File not found` | Missing data files | Add data files to configured directory |

### Support Resources

- **Documentation**: Check [API Documentation](API.md) and [Configuration Guide](CONFIGURATION.md)
- **GitHub Issues**: Report bugs and feature requests
- **Community**: Join discussions and get help from other users
- **Logs**: Always check logs first for detailed error information

## 🔗 Related Documentation

- [API Documentation](API.md) - Complete API reference
- [Configuration Guide](CONFIGURATION.md) - Configuration options
- [Deployment Guide](DEPLOYMENT.md) - Deployment instructions
