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
http://localhost:8080/api
```

## Endpoints

### Health Check

#### Check Service Health
```
GET /health
```

**Response**
```json
{
  "status": "ok",
  "timestamp": "2024-04-07T02:38:25.344758663+00:00"
}
```

### Analytics

#### Get Repository Activity
```
GET /analytics/repository/{owner}/{repo}/activity
```

**Query Parameters**
- `days`: Number of days to look back (default: 30)

**Response**
```json
{
  "dates": ["2024-04-01T00:00:00Z", "2024-04-02T00:00:00Z"],
  "total_activity": [10, 15],
  "commits": [5, 8]
}
```

#### Get Repository Trends
```
GET /analytics/repository/{owner}/{repo}/trends
```

**Query Parameters**
- `days`: Number of days to look back (default: 30)

**Response**
```json
{
  "dates": ["2024-04-01T00:00:00Z", "2024-04-02T00:00:00Z"],
  "commit_counts": [5, 8]
}
```

### Data Synchronization

#### Sync Repository Data
```
POST /sync/repository/{owner}/{repo}
```

**Response**
```json
{
  "status": "success",
  "message": "Successfully synced owner/repo"
}
```

**Error Response**
```json
{
  "status": "error",
  "message": "Failed to sync repository: error details"
}
```

### Repositories

#### List Repositories
```
GET /repositories
```

**Query Parameters**
- `page`: Page number (default: 1)
- `per_page`: Items per page (default: 10)

**Response**
```json
[
  {
    "id": 1,
    "owner": "username",
    "name": "repo-name",
    "description": "Repository description",
    "language": "Rust",
    "stars": 100,
    "forks": 50,
    "open_issues": 10,
    "is_private": false,
    "created_at": "2024-04-06T12:00:00Z",
    "updated_at": "2024-04-06T12:00:00Z"
  }
]
```

#### Get Repository Details
```
GET /repositories/{owner}/{repo}
```

**Response**
```json
{
  "id": 1,
  "owner": "username",
  "name": "repo-name",
  "description": "Repository description",
  "language": "Rust",
  "stars": 100,
  "forks": 50,
  "open_issues": 10,
  "is_private": false,
  "created_at": "2024-04-06T12:00:00Z",
  "updated_at": "2024-04-06T12:00:00Z"
}
```

#### Get Repository Activity
```
GET /repositories/{owner}/{repo}/activity
```

**Query Parameters**
- `page`: Page number (default: 1)
- `per_page`: Items per page (default: 10)

**Response**
```json
[
  {
    "id": 1,
    "repository_id": 1,
    "activity_type": "commit",
    "created_at": "2024-04-06T12:00:00Z"
  }
]
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

All endpoints may return the following error responses:

### 400 Bad Request
```json
{
  "error": "Bad Request",
  "message": "Invalid request parameters"
}
```

### 404 Not Found
```json
{
  "error": "Not Found",
  "message": "Resource not found"
}
```

### 500 Internal Server Error
```json
{
  "error": "Internal Server Error",
  "message": "An unexpected error occurred"
}
``` 