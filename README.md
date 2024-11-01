# Rust Project 2024

## 도구설치

윈도우 버전은 다음 링크에서 **rustup** 도구를 다운로드 받아 설치한다
https://www.rust-lang.org/tools/install
설치가 완료된 후 다음 명령을 cmd 창에서 실행하여 path 가 잘 연결되었는지 확인한다.
도구는 **C:/Users/USERNAME/.cargo/bin/** 폴더에 설치되어 있다
명령 실행에 오류가 발생하면 **환경변수** 설정을 통해 **path** 정보를 등록해 두어야 한다.

```bash
rustup --version
ruestc --version
cargo --version
```

## cargo 를 이용한 프로젝트 생성

프로젝트 이름은 snack-case or kebab-case 로 작성한다

```bash
cargo new [project]
```

cargo를 사용하여 bin 프로젝트 생성하기
`--bin` 옵션을 생략하면 라이브러리용 프로젝트를 생성한다

```bash
cargo new [project] --bin
```

프로젝트 **build** 는 다음명령으로

```bash
cargo build
```

**build** 된 프로젝트 실행

```bash
cargo run
```
