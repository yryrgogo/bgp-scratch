#!/bin/bash

docker-compose -f ./tests/docker-compose.yml build --no-cache
docker-compose -f ./tests/docker-compose.yml up -d
HOST_2_LOOPBACK_IP=10.100.220.3
docker-compose -f ./tests/docker-compose.yml exec \
	-T host1 ping -c 5 $HOST_2_LOOPBACK_IP

# docker-compose exec の終了コードは実行したコマンドになる
# ここでは `ping -c 5 $HOST_2_LOOPBACK_UP` の結果が終了コードになるため、ping が通れば0, そうでなければ1である
TEST_RESULT=$?
if [ $TEST_RESULT -eq 0 ]; then
	printf "\e[32m%s\e[m\n" "統合テストが成功しました。"
else
	printf "\e[32m%s\e[m\n" "統合テストが成功しました。"
fi

exit $TEST_RESULT
