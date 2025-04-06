# API Documentation

This document describes the REST API endpoints provided by the GitHub Dashboard backend.

## Base URL

All API endpoints are relative to:
```
http://localhost:8080
```

## Authentication

The API uses GitHub Personal Access Tokens for authentication. Include the token in the `Authorization` header:

```
Authorization: Bearer your_github_token
```

## Endpoints

### Analytics

#### Get Repository Activity
```
GET /analytics/repository/{owner}/{repo}/activity
```

Query Parameters:
- `days` (optional): Number of days to fetch activity for (default: 30)

Response:
```json
{
  "commits": [
    {
      "sha": "string",
      "message": "string",
      "author": "string",
      "date": "string"
    }
  ],
  "activity": [
    {
      "date": "string",
      "count": number
    }
  ]
}
```

#### Get Repository Trends
```
GET /analytics/repository/{owner}/{repo}/trends
```

Query Parameters:
- `days` (optional): Number of days to analyze trends for (default: 30)

Response:
```json
{
  "metrics": [
    {
      "name": "string",
      "value": number,
      "trend": "string"
    }
  ],
  "period": {
    "start": "string",
    "end": "string"
  }
}
```

## Error Responses

### 400 Bad Request
```json
{
  "error": "Invalid request parameters"
}
```

### 401 Unauthorized
```json
{
  "error": "Invalid or missing GitHub token"
}
```

### 404 Not Found
```json
{
  "error": "Repository not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal server error"
}
```

## Rate Limiting

The API implements rate limiting based on GitHub's API limits. When approaching the limit, the API will return:

```json
{
  "error": "Rate limit exceeded",
  "reset_time": "timestamp"
}
```

## WebSocket Updates

The API provides WebSocket endpoints for real-time updates:

```
ws://localhost:8080/ws
```

### WebSocket Events

1. **Repository Activity Update**
```json
{
  "type": "activity_update",
  "data": {
    "repository": "string",
    "activity": number
  }
}
```

2. **Error Event**
```json
{
  "type": "error",
  "error": "string"
}
``` 