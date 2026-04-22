# ESP32-S3 Rust Project

Template project Rust (`no_std`) untuk board ESP32-S3 menggunakan ekosistem `esp-rs` (`esp-hal`, `espflash`, dan `espup`).

## Fitur

- Konfigurasi target default `xtensa-esp32s3-none-elf`
- Runtime `no_std` + `no_main`
- Logging serial lewat `esp-println`
- Panic/backtrace lewat `esp-backtrace`
- Runner otomatis untuk flash + monitor serial

## Requirement

- Windows PowerShell (atau shell lain yang kompatibel)
- Rust toolchain
- Board ESP32-S3 + kabel data USB

## Setup Environment (sekali saja)

Install ESP-Rust toolchain:

```powershell
irm https://esp-rs.github.io/espup/install.ps1 | iex
espup install
```

Setelah selesai, restart terminal. Jika perlu, load environment manual:

```powershell
. $HOME\export-esp.ps1
```

Install tools:

```powershell
cargo install ldproxy espflash
```

## Build dan Run

Masuk ke folder project:

```powershell
cd project
```

Cek kompilasi:

```powershell
cargo check
```

Build firmware:

```powershell
cargo build
```

Flash ke board + buka serial monitor:

```powershell
cargo run
```

## Konfigurasi Penting

`/.cargo/config.toml`:
- target default: `xtensa-esp32s3-none-elf`
- runner: `espflash flash --monitor`
- `build-std = ["core"]` untuk target Xtensa

`/rust-toolchain.toml`:
- channel: `esp`
- component: `rust-src`

## Struktur Singkat

- `src/main.rs` - entrypoint firmware
- `Cargo.toml` - dependency dan profile build
- `.cargo/config.toml` - target/runner/rustflags
- `rust-toolchain.toml` - pin toolchain ESP
- `ESP32S3_SETUP.md` - panduan setup versi lebih detail

## Catatan

- Jika serial monitor tidak muncul, cek port USB dan driver board ESP32-S3.
- Jika `cargo check` error terkait target `core`, pastikan environment dari `espup` sudah aktif (`export-esp.ps1` sudah dijalankan atau terminal sudah direstart).
