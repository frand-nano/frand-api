#!/bin/sh
set -e

# nginx.conf 템플릿에 환경 변수 적용
envsubst '${NGINX_SERVER_NAME} ${NGINX_EXTERNAL_PORT} ${NGINX_EXTERNAL_HTTP_PORT} ${ROCKET_PORT} ${FRONTEND_API_ENDPOINT} ${LOG_LEVEL}' < /etc/nginx/conf.d/default.conf.template > /etc/nginx/conf.d/default.conf

# 환경 변수가 적용된 설정 파일 확인 (디버깅용)
echo "Nginx 설정 파일이 생성되었습니다:"
echo "-----------------------------------"
cat /etc/nginx/conf.d/default.conf
echo "-----------------------------------"

# Nginx 실행
exec nginx -g 'daemon off;'
