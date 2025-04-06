# API Documentation

This document provides detailed information about the GitHub Dashboard API endpoints, request/response formats, and authentication.

## Authentication

All API endpoints require authentication using a GitHub Personal Access Token (PAT). The token should be included in the request header:

```
Authorization: Bearer <github_pat>
```

## Base URL

The API is available at:
```
http://localhost:8080/api/v1
```

## Endpoints

### Repositories

#### List Repositories
```
GET /repos
```

**Response**
```json
{
  "repositories": [
    {
      "id": 1,
      "name": "repo-name",
      "owner": "username",
      "url": "https://github.com/username/repo-name",
      "last_updated": "2024-04-06T12:00:00Z",
      "stats": {
        "stars": 100,
        "forks": 50,
        "open_issues": 10,
        "watchers": 25
      }
    }
  ]
}
```

#### Get Repository Details
```
GET /repos/{owner}/{repo}
```

**Response**
```json
{
  "id": 1,
  "name": "repo-name",
  "owner": "username",
  "url": "https://github.com/username/repo-name",
  "last_updated": "2024-04-06T12:00:00Z",
  "stats": {
    "stars": 100,
    "forks": 50,
    "open_issues": 10,
    "watchers": 25,
    "commit_count": 500,
    "contributors": 15
  }
}
```

### Activity

#### Get Recent Activity
```
GET /activity
```

**Query Parameters**
- `limit`: Number of activities to return (default: 50)
- `type`: Filter by activity type (commit, issue, pr)
- `repo`: Filter by repository

**Response**
```json
{
  "activities": [
    {
      "id": 1,
      "repo_id": 1,
      "type": "commit",
      "user": "username",
      "timestamp": "2024-04-06T12:00:00Z",
      "details": {
        "message": "Update documentation",
        "sha": "abc123",
        "url": "https://github.com/username/repo/commit/abc123"
      }
    }
  ]
}
```

### Tasks

#### List Tasks
```
GET /tasks
```

**Query Parameters**
- `status`: Filter by task status
- `priority`: Filter by priority level
- `repo`: Filter by repository

**Response**
```json
{
  "tasks": [
    {
      "id": 1,
      "repo_id": 1,
      "github_issue_id": 123,
      "title": "Fix bug in authentication",
      "priority": "high",
      "status": "open",
      "due_date": "2024-04-10T00:00:00Z"
    }
  ]
}
```

#### Create Task
```
POST /tasks
```

**Request Body**
```json
{
  "repo_id": 1,
  "title": "New task",
  "priority": "medium",
  "status": "open",
  "due_date": "2024-04-10T00:00:00Z"
}
```

#### Update Task
```
PUT /tasks/{id}
```

**Request Body**
```json
{
  "priority": "high",
  "status": "in-progress",
  "due_date": "2024-04-15T00:00:00Z"
}
```

#### Delete Task
```
DELETE /tasks/{id}
```

### Analytics

#### Get Repository Analytics
```
GET /analytics/repos/{owner}/{repo}
```

**Response**
```json
{
  "commit_activity": {
    "daily": [10, 15, 20, 5, 8],
    "weekly": [100, 120, 90, 110],
    "monthly": [500, 600, 550]
  },
  "issue_metrics": {
    "open": 10,
    "closed": 50,
    "average_resolution_time": "2 days"
  },
  "pr_metrics": {
    "open": 5,
    "merged": 30,
    "average_merge_time": "1 day"
  }
}
```

#### Get User Analytics
```
GET /analytics/users/{username}
```

**Response**
```json
{
  "contribution_stats": {
    "total_commits": 1000,
    "total_prs": 100,
    "total_issues": 50,
    "total_reviews": 200
  },
  "activity_trends": {
    "daily_activity": [5, 10, 15, 8, 12],
    "weekly_activity": [50, 60, 70, 55],
    "monthly_activity": [300, 350, 400]
  }
}
```

## Webhooks

### GitHub Webhook Endpoint
```
POST /webhooks/github
```

**Headers**
```
X-GitHub-Event: <event_type>
X-Hub-Signature: <signature>
```

**Supported Events**
- push
- pull_request
- issues
- issue_comment
- create
- delete

## Error Responses

### 400 Bad Request
```json
{
  "error": "Invalid request parameters",
  "details": "Missing required field: title"
}
```

### 401 Unauthorized
```json
{
  "error": "Unauthorized",
  "details": "Invalid or missing GitHub token"
}
```

### 404 Not Found
```json
{
  "error": "Not Found",
  "details": "Repository not found"
}
```

### 429 Too Many Requests
```json
{
  "error": "Rate Limit Exceeded",
  "details": "GitHub API rate limit reached",
  "retry_after": 60
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal Server Error",
  "details": "An unexpected error occurred"
}
```

## Rate Limiting

The API implements rate limiting to prevent abuse and ensure fair usage:

- 100 requests per minute per IP address
- 1000 requests per hour per GitHub token

Rate limit headers are included in responses:
```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 95
X-RateLimit-Reset: 1617724800
```

## WebSocket API

### Connection
```
ws://localhost:8080/api/v1/ws
```

**Authentication**
Include the GitHub token in the connection URL:
```
ws://localhost:8080/api/v1/ws?token=<github_pat>
```

### Events

#### Repository Updates
```json
{
  "type": "repo_update",
  "data": {
    "repo_id": 1,
    "event": "push",
    "timestamp": "2024-04-06T12:00:00Z"
  }
}
```

#### Task Updates
```json
{
  "type": "task_update",
  "data": {
    "task_id": 1,
    "status": "in-progress",
    "timestamp": "2024-04-06T12:00:00Z"
  }
}
```

## Pagination

Endpoints that return lists support pagination using cursor-based pagination:

**Request**
```
GET /repos?cursor=<base64_encoded_cursor>&limit=50
```

**Response**
```json
{
  "items": [...],
  "next_cursor": "<base64_encoded_cursor>",
  "has_more": true
}
``` 