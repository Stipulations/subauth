# SubAuth

An open source alternative to KeyAuth and Auth.gg, written in Rust.

## API Routes

### Health Check
**GET** `/`

Returns server health status and system metrics.

**Response:**
```json
{
  "status": "healthy",
  "cpu_usage": 8.655512,
  "memory_usage": {
    "used_mb": 5905,
    "total_mb": 8192,
    "usage_percent": 72.0932
  },
  "request_time": 1752757323,
  "response_time": 1752757323
}
```

## Development

This project uses some crates I'm not familiar with, so please let me know if you see any issues.