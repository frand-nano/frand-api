# 실행 스테이지 (Nginx)
FROM nginx:1.27.4-alpine-slim

# 정적 파일용 디렉토리 생성
RUN mkdir -p /usr/share/nginx/static

# Nginx 실행
CMD ["nginx", "-g", "daemon off;"]