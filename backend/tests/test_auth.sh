#!/bin/bash

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
NC='\033[0m' # No Color

# Base URL for the API
API_URL="http://localhost:8080/api/v1"

echo "Testing Authentication System"
echo "==========================="

# Test 1: Health Check (Public Route)
echo -n "Testing health check endpoint (public): "
response=$(curl -s -w "%{http_code}" "${API_URL}/health")
http_code=${response: -3}
if [ "$http_code" = "200" ]; then
    echo -e "${GREEN}PASSED${NC}"
else
    echo -e "${RED}FAILED${NC} (HTTP $http_code)"
fi

# Test 2: Protected Route Without Auth
echo -n "Testing protected route without auth: "
response=$(curl -s -w "%{http_code}" "${API_URL}/auth/test")
http_code=${response: -3}
if [ "$http_code" = "401" ]; then
    echo -e "${GREEN}PASSED${NC} (Got expected 401 Unauthorized)"
else
    echo -e "${RED}FAILED${NC} (Expected 401, got $http_code)"
fi

# Test 3: GitHub Login Redirect
echo -n "Testing GitHub login redirect: "
response=$(curl -s -w "%{http_code}" -L "${API_URL}/auth/github")
http_code=${response: -3}
if [ "$http_code" = "200" ] || [ "$http_code" = "302" ]; then
    echo -e "${GREEN}PASSED${NC}"
else
    echo -e "${RED}FAILED${NC} (HTTP $http_code)"
fi

# Test 4: Protected Route With Invalid JWT
echo -n "Testing protected route with invalid JWT: "
response=$(curl -s -w "%{http_code}" -H "Authorization: Bearer invalid.token.here" "${API_URL}/auth/test")
http_code=${response: -3}
if [ "$http_code" = "401" ]; then
    echo -e "${GREEN}PASSED${NC} (Got expected 401 Unauthorized)"
else
    echo -e "${RED}FAILED${NC} (Expected 401, got $http_code)"
fi

# Test 5: Logout Without Being Logged In
echo -n "Testing logout without being logged in: "
response=$(curl -s -w "%{http_code}" -X POST "${API_URL}/auth/logout")
http_code=${response: -3}
if [ "$http_code" = "200" ]; then
    echo -e "${GREEN}PASSED${NC}"
else
    echo -e "${RED}FAILED${NC} (HTTP $http_code)"
fi

echo -e "\nManual Testing Required:"
echo "1. Visit ${API_URL}/auth/github in a browser"
echo "2. Complete GitHub OAuth flow"
echo "3. After redirect, check that you have an 'auth_token' cookie"
echo "4. Try accessing ${API_URL}/auth/test with the cookie"
echo "5. Test logout by calling POST ${API_URL}/auth/logout"
