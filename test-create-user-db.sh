#!/bin/bash
set -x  # 명령어 실행 전에 명령어 자체를 출력

echo "=== Testing create-user-db ==="
curl -X POST http://localhost:3000/create-user-db \
  -H "Content-Type: application/json" \
  -d '{"name": "zorba house", "email": "zorba@example.com"}' | jq
echo -e "\n"
