#!/bin/sh
set -e

# nginx.conf 템플릿에 환경 변수 적용
envsubst '${ROCKET_PORT} ${ROCKET_API_ENDPOINT} ${NGINX_HTTP_PORT} ${NGINX_HTTPS_PORT}' < /etc/nginx/conf.d/nginx.conf.template > /etc/nginx/conf.d/nginx.conf

# 환경 변수가 적용된 설정 파일 확인
# cat /etc/nginx/conf.d/nginx.conf

# Nginx 실행
exec nginx -g 'daemon off;'
