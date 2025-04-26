# Redis Setup for Personal GitHub Dashboard

## Connection Parameters

- **URL:** `redis://redis:6379`
- **Host:** redis (Docker service name)
- **Port:** 6379

## Docker Compose Configuration

- **Image:** redis:7
- **Memory Limit:** 256 MB (`--maxmemory 256mb`)
- **Eviction Policy:** LRU, evict any key (`--maxmemory-policy allkeys-lru`)
- **Persistence:**
  - **AOF:** Enabled (`--appendonly yes`)
  - **RDB Snapshots:** Enabled (`--save 900 1 300 10 60 10000`)
- **Volumes:** Data persisted at `/data` via `redis_data` Docker volume
- **Restart Policy:** `unless-stopped`

## Monitoring

- **Redis CLI:**
  - Connect from within the Docker network: `docker-compose exec redis redis-cli`
  - Useful commands: `INFO`, `MONITOR`, `MEMORY STATS`, `CLIENT LIST`
- **RedisInsight:**
  - Optionally run [RedisInsight](https://redis.com/redis-enterprise/redis-insight/) for advanced GUI monitoring
  - Connect to `redis://localhost:6379` from your host if port is exposed
- **Metrics:**
  - For production, consider exporting metrics using [redis_exporter](https://github.com/oliver006/redis_exporter) for Prometheus/Grafana

## Example: Connecting from Rust Backend

The backend uses the following environment variable:

```
REDIS_URL=redis://redis:6379
```

In Rust, this is loaded by the config system and used to initialize the Redis client.

## References

- [Redis Docker Hub](https://hub.docker.com/_/redis)
- [Redis Persistence Docs](https://redis.io/docs/management/persistence/)
- [Redis Memory Management](https://redis.io/docs/management/memory/)
- [Redis Monitoring](https://redis.io/docs/management/monitor/)
