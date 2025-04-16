#!/bin/bash

# 자체 서명 인증서 생성 스크립트
# 이 스크립트는 Nginx에서 사용할 TLS 인증서와 개인 키를 생성합니다.

# 색상 코드 정의
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[0;33m'
NC='\033[0m' # No Color

DOMAIN="localhost"

# 인증서 디렉토리 생성
CERT_DIR="./deploy/secure/tls"
if [ ! -d "$CERT_DIR" ]; then
    mkdir -p "$CERT_DIR"
    echo "인증서 디렉토리 생성: $CERT_DIR"
fi

# OpenSSL 설치 확인
if ! command -v openssl &> /dev/null; then
    echo -e "${RED}오류: OpenSSL이 설치되어 있지 않습니다.${NC}"
    echo "Ubuntu/Debian: sudo apt-get install openssl"
    echo "CentOS/RHEL: sudo yum install openssl"
    echo "macOS: brew install openssl"
    exit 1
fi

echo -e "${GREEN}자체 서명 인증서 생성 시작...${NC}"

# 개인 키 및 인증서 경로 설정
CERT_PATH="$CERT_DIR/cert.pem"
KEY_PATH="$CERT_DIR/privkey.pem"

# 개인 키 및 자체 서명 인증서 생성
openssl req -x509 -newkey rsa:4096 -nodes \
    -keyout "$KEY_PATH" \
    -out "$CERT_PATH" \
    -days 365 \
    -subj "/C=KR/ST=Seoul/L=Seoul/O=Frand API/OU=Development/CN=$DOMAIN" \
    -addext "subjectAltName = DNS:$DOMAIN,DNS:www.$DOMAIN,IP:127.0.0.1"

# 생성 확인
if [ -f "$CERT_PATH" ] && [ -f "$KEY_PATH" ]; then
    echo -e "${GREEN}인증서 생성 완료!${NC}"
    echo "인증서 위치: $CERT_PATH"
    echo "개인 키 위치: $KEY_PATH"
    
    # 인증서 정보 출력
    echo -e "\n${YELLOW}인증서 정보:${NC}"
    openssl x509 -in "$CERT_PATH" -noout -text | grep -E 'Subject:|Issuer:|Not Before:|Not After :|DNS:'
    
    # 필요한 경우 권한 설정
    chmod 644 "$CERT_PATH"
    chmod 600 "$KEY_PATH"
else
    echo -e "${RED}인증서 생성 실패!${NC}"
    exit 1
fi