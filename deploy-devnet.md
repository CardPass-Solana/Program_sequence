# Devnet 배포 가이드

## 사전 준비

1. **Solana 지갑 설정**
```bash
# Solana CLI가 devnet을 바라보도록 설정
solana config set --url https://api.devnet.solana.com

# 현재 설정 확인
solana config get

# 지갑 잔액 확인 (devnet SOL 필요)
solana balance
```

2. **Devnet SOL 에어드롭 (필요시)**
```bash
# 2 SOL 에어드롭 받기 (배포 및 테스트용)
solana airdrop 2

# 잔액 재확인
solana balance
```

## 프로그램 배포

```bash
# 프로그램 빌드
anchor build

# Devnet에 배포
anchor deploy

# 배포 성공 시 프로그램 ID 확인
solana account <PROGRAM_ID> --output json
```

## 배포된 프로그램 정보

- **Program ID**: `6ZcUqLAydNswng5yphmzZRAUkVhY2Kj2rpVAU55vhkrc`
- **Network**: Devnet
- **Explorer**: https://explorer.solana.com/address/6ZcUqLAydNswng5yphmzZRAUkVhY2Kj2rpVAU55vhkrc?cluster=devnet

## 기능 테스트 방법

### 1. 프로필 생성
```bash
# PDA 계산: ["profile", your_wallet_pubkey]
```

### 2. Job 생성
```bash
# PDA 계산: ["job", recruiter_pubkey, job_id_bytes]
```

### 3. 지원하기
```bash
# PDA 계산: ["application", job_pubkey, applicant_pubkey]
```

### 4. 스카우트 시스템
```bash
# PDA 계산: ["scout", recruiter_pubkey, profile_pubkey]
```

## 주요 명령어들

```bash
# 프로그램 재배포 (업데이트)
anchor upgrade target/deploy/job_platform.so --program-id 6ZcUqLAydNswng5yphmzZRAUkVhY2Kj2rpVAU55vhkrc

# 프로그램 계정 정보 확인
solana account 6ZcUqLAydNswng5yphmzZRAUkVhY2Kj2rpVAU55vhkrc

# IDL 업로드 (프론트엔드 연동용)
anchor idl init 6ZcUqLAydNswng5yphmzZRAUkVhY2Kj2rpVAU55vhkrc -f target/idl/job_platform.json
```

## Jenkins 연동 시 참고사항

1. **환경변수 설정**
   - `ANCHOR_WALLET`: 지갑 파일 경로
   - `ANCHOR_PROVIDER_URL`: https://api.devnet.solana.com

2. **배포 자동화 스크립트**
```bash
#!/bin/bash
solana config set --url https://api.devnet.solana.com
anchor build
anchor deploy
anchor idl init $PROGRAM_ID -f target/idl/job_platform.json
```