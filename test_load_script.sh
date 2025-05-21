#!/bin/bash

# 총 반복 횟수
TOTAL_ITERATIONS=100

# 시작 시간 기록
start_time=$(date +%s%N) # 나노초까지 기록

echo "Starting $TOTAL_ITERATIONS iterations of tests..."

for i in $(seq 1 $TOTAL_ITERATIONS)
do
    echo "--- Iteration $i / $TOTAL_ITERATIONS ---"

    echo "Testing create-user (Iteration $i)"
    curl -s -o /dev/null -X POST http://localhost:3000/create-user
    # echo -e "\n"

    echo "Testing list_users (Iteration $i)"
    curl -s http://localhost:3000/users | jq > /dev/null
    # echo -e "\n"

    echo "Testing show_item (Iteration $i)"
    curl -s -o /dev/null "http://localhost:3000/item/42?number=2"
    # echo -e "\n"

    echo "Testing add_item (Iteration $i)"
    curl -s -o /dev/null -X POST http://localhost:3000/add-item \
        -H "Content-Type: application/json" \
        -d '{"title": "Some random item"}'
    # echo -e "\n"

    echo "Testing delete_user (Iteration $i)"
    curl -s -o /dev/null -X DELETE http://localhost:3000/delete-user/2
    # echo -e "\n"

    echo "Testing get_axum_users (Iteration $i)"
    curl -s http://localhost:3000/axum-users | jq > /dev/null
    # echo -e "\n"

    echo "Testing get_app_state with auth-key (Iteration $i)"
    curl -s http://localhost:3000/admin/get_app_state -H "X-Admin-API-Key: 2309oijq2309rafjkq230r980afj" | jq > /dev/null
    # echo -e "\n"

done

echo "--- All iterations completed ---"

# 종료 시간 기록
end_time=$(date +%s%N)

# 총 실행 시간 계산 (나노초)
duration_ns=$((end_time - start_time))

# 초 단위로 변환 (소수점 3자리까지)
duration_s=$(echo "scale=3; $duration_ns / 1000000000" | bc)

echo "Total execution time: $duration_s seconds" 