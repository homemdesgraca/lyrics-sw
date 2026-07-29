# lyrics-sw
lyrics-sw (lyrics should work) is a super minimal, rust-based, cli tool for fetching lyrics from https://lrclib.net/ for songs without lyrics in your local music library.

## installation

### option 1 - download a binary

|  file  | platform | checksum |
|--------|----------|----------|
| [lyrics-sw-aarch64-apple-darwin.tar.xz](https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-aarch64-apple-darwin.tar.xz) | Apple Silicon macOS | [checksum](https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-aarch64-apple-darwin.tar.xz.sha256) |
| [lyrics-sw-x86_64-apple-darwin.tar.xz](https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-x86_64-apple-darwin.tar.xz) | Intel macOS | [checksum](https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-x86_64-apple-darwin.tar.xz.sha256) |
| [lyrics-sw-x86_64-pc-windows-msvc.zip](https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-x86_64-pc-windows-msvc.zip) | x64 Windows | [checksum](https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-x86_64-pc-windows-msvc.zip.sha256) |
| [lyrics-sw-x86_64-unknown-linux-gnu.tar.xz](https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-x86_64-unknown-linux-gnu.tar.xz) | x64 Linux | [checksum](https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-x86_64-unknown-linux-gnu.tar.xz.sha256) |

---
### option 2 - use installer scripts

shell:
```sh
curl --proto '=https' --tlsv1.2 -LsSf https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-installer.sh | sh
```
---
powershell:

```sh
powershell -ExecutionPolicy Bypass -c "irm https://github.com/homemdesgraca/lyrics-sw/releases/latest/download/lyrics-sw-installer.ps1 | iex"
```

---
### option 3 - build it using cargo

```sh
git clone https://github.com/homemdesgraca/lyrics-sw
cd lyrics-sw
cargo build --release # make sure cargo is installed on your system
```

---
## how to use

### 1 - run the binary on your favorite terminal
### 2 - input your path
### 3 - wait
### 4 - that's it.

---
## support these amazing projects s2
- [lofty-rs](https://github.com/Serial-ATA/lofty-rs)
- [lrclib](https://lrclib.net/)
