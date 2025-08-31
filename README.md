# 🐚 N3xT Shellcode Runner CLI

Shellcode를 로드하고 실행/디버깅할 수 있는 Rust 기반 CLI 도구임.  
분석 및 테스트 목적으로 설계됐으며, **VM 환경에서만 사용**하는 것을 권장함.

## 📜 목차
- [🐚 N3xT Shellcode Runner CLI](#-n3xt-shellcode-runner-cli)
  - [📜 목차](#-목차)
  - [🔍 프로젝트 소개](#-프로젝트-소개)
  - [✨ 주요 기능](#-주요-기능)
  - [🖼️ 실행 예시](#️-실행-예시)
  - [⚙️ 설치 및 빌드](#️-설치-및-빌드)
  - [▶️ 사용법](#️-사용법)
  - [📂 프로젝트 구조](#-프로젝트-구조)
  - [🧩 동작 원리](#-동작-원리)
  - [🔒 주의사항](#-주의사항)
  - [🐞 디버깅 가이드](#-디버깅-가이드)

## 🔍 프로젝트 소개
- **목적:** Shellcode를 메모리에 로드하고 Thread 단위로 실행할 수 있는 도구
- **플랫폼:** Windows x64/x86
- **주요 사용 사례:**
  - 악성코드 분석 및 PoC 테스트
  - Shellcode 디버깅 및 분석 환경 제공

## ✨ 주요 기능
- 파일에서 Shellcode 읽기 및 메모리 매핑
- Shellcode 실행 스레드 Suspend/Resume 지원
- HEX Preview로 Shellcode 일부 내용 출력

## 🖼️ 실행 예시
```powershell
> .\n3xtSCrun32.exe -f .\example.bin -s 0

+==============================================================+
|                                                              |
|           _____     _     ____   ____                        |
|     _ __ |___ /_  _| |_  / ___| / ___|  _ __ _   _ _ __      |
|    | '_ \  |_ \ \/ / __| \___ \| |     | '__| | | | '_ \     |
|    | | | |___) >  <| |_   ___) | |___  | |  | |_| | | | |    |
|    |_| |_|____/_/\_\\__| |____/ \____| |_|   \__,_|_| |_|    |
|                                                              |
+==============================================================+
[i] N3xT Shellcode Runner CLI 32-bit

── Input Parameters ─────────────────────────────────────────
[>] Parsing CLI arguments…
[+] CLI arguments parsed.
  Shellcode File Path          .\example.bin
  Shellcode Start Offset       0x0 (0)
  Shellcode Memory Size        None

── Load ─────────────────────────────────────────────────────
[>] Loading shellcode…
[+] Shellcode loaded.
  Entry Address                0x690000 (6881280)
  Aligned Size                 0x1000 (4096)
  Payload Size                 0x2 (2)
  Content Preview              EB FE

── Spawn ────────────────────────────────────────────────────
[>] Creating suspended thread…
[+] Thread created (suspended).
  Thread ID                    0x68A4 (26788)

── Debug ────────────────────────────────────────────────────
[!] Shellcode thread is suspended.
[!] Attach a debugger NOW if you want to analyze.
[>] Press ENTER to resume execution…
````

## ⚙️ 설치 및 빌드

```powershell
# 저장소 클론
git clone https://github.com/yourname/n3xt_shellcode_runner.git
cd n3xt_shellcode_runner

# 빌드 (x64)
cargo build --release

# 빌드 (x86)
rustup target add i686-pc-windows-msvc
cargo build --release --target i686-pc-windows-msvc
```

## ▶️ 사용법

```powershell
Usage: n3xtSCrun32.exe [OPTIONS] --file-path <FILE_PATH> --start-offset <START_OFFSET>

Options:
  -f, --file-path <FILE_PATH>
  -s, --start-offset <START_OFFSET>
  -m, --mem-size <MEMORY_SIZE>
  -h, --help                         Print help
  -V, --version                      Print version

# 예시
n3xtSCrun32.exe -f ./example.bin -s 0
n3xtSCrun32.exe -f ./example.bin -s 0 -m 0x100
```

| 옵션             | 설명                                                                                                    |
| -------------- | ----------------------------------------------------------------------------------------------------- |
| `-f`, `--file_path`    | 실행할 Shellcode 바이너리 파일 경로                                                                              |
| `-s`, `--start_offset` | Shellcode 시작 오프셋 (16진수 `0x` 표기 지원)                                                                    |
| `-m`, `--mem-size`   | 메모리 예약 크기. **지정하지 않으면 Shellcode 크기만큼 할당** |

> [!NOTE]
> OS 메모리 페이지 정렬(Alignment)에 맞춰 실제 할당 크기가 달라질 수 있음.

## 📂 프로젝트 구조

```
src/
 ├─ main.rs         # 엔트리 포인트
 ├─ runner.rs       # Runner 구조체, 실행 로직
 ├─ shellcode.rs    # Shellcode 타입 및 로드 로직
 ├─ thread.rs       # Thread 생성 및 제어
 ├─ logger.rs       # Logger 구조체, 출력 유틸
 └─ error.rs        # 오류 정의
```

## 🧩 동작 원리

1. Shellcode 파일을 읽어 `VirtualAlloc` API를 통해 메모리 할당함.
2. 전달된 Offset을 기준으로 Suspend 상태의 Thread를 생성함.
3. 사용자가 디버거를 붙인 후 Enter를 입력하면 Resume됨.

## 🔒 주의사항

* 이 도구는 학습 및 분석 용도임.
* 악성 코드 테스트는 반드시 샌드박스, 가상머신 환경에서 실행해야 함.

## 🐞 디버깅 가이드

* 프로그램이 Shellcode 실행 스레드를 **Suspend 상태**로 시작함.
* Attach 시점에 브레이크포인트 설정 후 Resume 가능함.