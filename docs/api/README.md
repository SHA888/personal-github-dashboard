# Personal GitHub Dashboard API Documentation

This document provides detailed information about the Personal GitHub Dashboard API endpoints, request/response formats, and authentication.

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

### Authentication

#### Login with GitHub
```
POST /auth/github
```

**Request Body**
```json
{
  "code": "github_oauth_code"
}
```

**Response**
```json
{
  "token": "jwt_token",
  "user": {
    "id": 1,
    "username": "username",
    "email": "user@example.com",
    "subscription_tier": "free"
  }
}
```

### Repositories

#### List Repositories
```
GET /repos
```

**Query Parameters**
- `page`: Page number (default: 1)
- `per_page`: Items per page (default: 30)
- `org`: Filter by organization
- `type`: Filter by repository type (all, public, private)

**Response**
```json
{
  "repositories": [
    {
      "id": 1,
      "github_id": 123456789,
      "name": "repo-name",
      "full_name": "username/repo-name",
      "owner": "username",
      "description": "Repository description",
      "language": "Rust",
      "stars": 100,
      "forks": 50,
      "open_issues": 10,
      "is_private": false,
      "last_synced_at": "2024-04-06T12:00:00Z"
    }
  ],
  "pagination": {
    "total": 100,
    "page": 1,
    "per_page": 30,
    "total_pages": 4
  }
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
  "github_id": 123456789,
  "name": "repo-name",
  "full_name": "username/repo-name",
  "owner": "username",
  "description": "Repository description",
  "language": "Rust",
  "stars": 100,
  "forks": 50,
  "open_issues": 10,
  "is_private": false,
  "last_synced_at": "2024-04-06T12:00:00Z",
  "activity": {
    "commit_count": 500,
    "contributors": 15,
    "last_commit": "2024-04-06T12:00:00Z"
  }
}
```

### Organizations

#### List Organizations
```
GET /orgs
```

**Response**
```json
{
  "organizations": [
    {
      "id": 1,
      "github_id": 987654321,
      "name": "org-name",
      "description": "Organization description",
      "avatar_url": "https://github.com/orgs/org-name/avatar",
      "repositories_count": 50
    }
  ]
}
```

#### Get Organization Details
```
GET /orgs/{org}
```

**Response**
```json
{
  "id": 1,
  "github_id": 987654321,
  "name": "org-name",
  "description": "Organization description",
  "avatar_url": "https://github.com/orgs/org-name/avatar",
  "repositories": [
    {
      "id": 1,
      "name": "repo-name",
      "description": "Repository description",
      "stars": 100,
      "forks": 50
    }
  ],
  "members": [
    {
      "id": 1,
      "username": "username",
      "role": "admin"
    }
  ]
}
```

### Activity

#### Get Recent Activity
```
GET /activity
```

**Query Parameters**
- `page`: Page number (default: 1)
- `per_page`: Items per page (default: 50)
- `type`: Filter by activity type (commit, issue, pr, review)
- `repo`: Filter by repository
- `org`: Filter by organization
- `since`: Filter by date (ISO 8601)

**Response**
```json
{
  "activities": [
    {
      "id": 1,
      "github_id": 123456789,
      "type": "commit",
      "repository": {
        "id": 1,
        "name": "repo-name",
        "owner": "username"
      },
      "author": "username",
      "title": "Update documentation",
      "body": "Updated API documentation",
      "state": "closed",
      "created_at": "2024-04-06T12:00:00Z",
      "updated_at": "2024-04-06T12:00:00Z",
      "closed_at": "2024-04-06T12:00:00Z",
      "metadata": {
        "sha": "abc123",
        "url": "https://github.com/username/repo/commit/abc123"
      }
    }
  ],
  "pagination": {
    "total": 1000,
    "page": 1,
    "per_page": 50,
    "total_pages": 20
  }
}
```

### Analytics

#### Get Repository Analytics
```
GET /analytics/repos/{owner}/{repo}
```

**Query Parameters**
- `period`: Time period (day, week, month, year)
- `metrics`: Comma-separated list of metrics to include

**Response**
```json
{
  "commit_activity": {
    "daily": [10, 15, 20, 5, 8],
    "weekly": [100, 120, 90, 110],
    "monthly": [500, 600, 550],
    "trend": "up",
    "change_percentage": 15.5
  },
  "issue_metrics": {
    "open": 10,
    "closed": 50,
    "average_resolution_time": "2 days",
    "resolution_trend": "improving"
  },
  "pr_metrics": {
    "open": 5,
    "merged": 30,
    "average_merge_time": "1 day",
    "review_trend": "stable"
  },
  "contributor_stats": {
    "total": 15,
    "active": 8,
    "top_contributors": [
      {
        "username": "user1",
        "commits": 100,
        "additions": 5000,
        "deletions": 2000
      }
    ]
  }
}
```

#### Get Organization Analytics
```
GET /analytics/orgs/{org}
```

**Response**
```json
{
  "overview": {
    "total_repos": 50,
    "total_contributors": 100,
    "total_commits": 5000,
    "total_prs": 1000
  },
  "repository_metrics": [
    {
      "name": "repo1",
      "stars": 100,
      "forks": 50,
      "activity_score": 85,
      "health_score": 90
    }
  ],
  "team_metrics": {
    "total_teams": 5,
    "active_members": 30,
    "collaboration_score": 75
  }
}
```

## WebSocket API

### Connect to WebSocket
```
ws://localhost:8080/ws
```

**Authentication**
Include the JWT token in the connection URL:
```
ws://localhost:8080/ws?token=<jwt_token>
```

**Events**
```json
{
  "type": "activity",
  "data": {
    "id": 1,
    "type": "commit",
    "repository": "username/repo",
    "author": "username",
    "timestamp": "2024-04-06T12:00:00Z"
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
X-GitHub-Delivery: <delivery_id>
```

**Supported Events**
- push
- pull_request
- pull_request_review
- issues
- issue_comment
- create
- delete
- organization
- member
- team

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
  "details": "Invalid or expired token"
}
```

### 403 Forbidden
```json
{
  "error": "Forbidden",
  "details": "Insufficient permissions for this action"
}
```

### 404 Not Found
```json
{
  "error": "Not Found",
  "details": "Resource not found"
}
```

### 429 Too Many Requests
```json
{
  "error": "Too Many Requests",
  "details": "Rate limit exceeded",
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