#!/bin/bash
set -x  # 명령어 실행 전에 명령어 자체를 출력

echo "=== Testing create-user ==="
curl -X POST http://localhost:3000/create-user
echo -e "\n"

echo "=== Testing list_users ==="
curl http://localhost:3000/users | jq
echo -e "\n"

echo "=== Testing show_item ==="
curl "http://localhost:3000/item/42?number=2"
echo -e "\n"

echo "=== Testing add_item ==="
curl -X POST http://localhost:3000/add-item \
    -H "Content-Type: application/json" \
    -d '{"title": "Some random item"}'
echo -e "\n"

echo "=== Testing delete_user ==="
curl -X DELETE http://localhost:3000/delete-user/2
echo -e "\n"

echo "=== Testing get_axum_users ==="
curl http://localhost:3000/axum-users | jq
echo -e "\n"

# echo "=== Testing get_app_state ==="
# curl http://localhost:3000/get_app_state | jq
# echo -e "\n"

echo "=== Testing get_app_state ==="
curl http://localhost:3000/get_app_state -H "X-Admin-API-Key: 2309oijq2309rafjkq230r980afj" | jq
echo -e "\n"

# curl -X POST http://localhost:3000/create-user

# curl http://localhost:3000/users | jq

# curl "http://localhost:3000/item/42?number=2"

# curl -X POST http://localhost:3000/add-item \
#      -H "Content-Type: application/json" \
#      -d '{"title": "Some random item"}'

# curl -X DELETE http://localhost:3000/delete-user/2

# curl http://localhost:3000/axum-users | jq
