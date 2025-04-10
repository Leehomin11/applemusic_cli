# 🎵 TC - Terminal CurrentTrack

> Apple Music에서 지금 재생 중인 곡 정보를 실시간으로 보여주는 CLI 도구  
> ⏱️ 실시간 재생 시간까지 출력! `Rust` + `osascript` 기반

---

## 📦 설치

```bash
git clone https://github.com/Leehomin11/applemusic_cli.git
cd applemusic_cli
chmod +x install.sh
./install.sh
```
설치 후에는 어디서든 tc 명령어로 실행 가능!

## 🚀 사용 방법
```bash
tc
```
출력예시
```yaml
🎶 Now Playing - Apple Music

🎵 Title: Mask Off
🎤 Artist: Future
💿 Album: FUTURE
⏱️ Elapsed: 00:44
```
재생 중인 곡이 바뀌면 자동 갱신됩니다.
정지되면 30초 이후 노래가 정지되었거나 종료되었습니다. 라고 표시됩니다.

## 🛠️ 개발
- Rust 기반 CLI

- AppleScript를 통해 macOS Apple Music 제어

- colored, tempfile 사용

## 💻 지원 환경
- macOS (Apple Music 앱 설치 필수)

- Rust >= 1.70

## 🧹 제거 방법
```bash
rm ~/.cargo/bin/tc
```


