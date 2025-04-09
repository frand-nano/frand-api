#!/bin/sh
set -e

# Nginx 설정 파일의 환경변수를 치환
envsubst '${NGINX_DOMAIN} ${TLS_CERT_FILE} ${TLS_KEY_FILE}' < /etc/nginx/conf.d/default.template > /etc/nginx/conf.d/default.conf

# 기본 nginx 엔트리포인트 실행
exec "$@"
