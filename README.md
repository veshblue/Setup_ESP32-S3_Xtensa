# ESP32-S3 Rust Deploy Template

Template ini dibuat supaya publik bisa langsung develop dan deploy firmware Rust ke ESP32-S3 tanpa setup manual yang rumit di setiap project baru.

Target utamanya: setelah ikuti README ini dari awal sampai akhir, user bisa menjalankan `cargo build` dan `cargo run` dengan minim error.

## Apa yang Sudah Disiapkan di Template Ini

- Target default `xtensa-esp32s3-none-elf`
- Arsitektur firmware `no_std` + `no_main`
- Panic dan backtrace via `esp-backtrace`
- Serial output via `esp-println`
- Runner otomatis flash + monitor via `espflash`
- Konfigurasi build Xtensa (`build-std = ["core"]`) agar lebih stabil untuk `cargo check/build`

## 1) Requirement

Siapkan hal berikut dulu:

- OS Windows (contoh command di README ini pakai PowerShell)
- Koneksi internet (untuk install dependency)
- [Git](https://git-scm.com/downloads)
- [Rustup/Rust](https://rustup.rs/)
- Board ESP32-S3
- Kabel USB data (bukan kabel charge-only)

Opsional tapi disarankan:

- Driver USB sesuai board ESP32-S3 kamu

## 2) Install Toolchain ESP-Rust (Sekali Saja)

Install `espup` lalu toolchain ESP:

```powershell
irm https://esp-rs.github.io/espup/install.ps1 | iex
espup install
```

Setelah selesai:

1. Tutup terminal
2. Buka terminal baru

Jika environment belum aktif, jalankan manual:

```powershell
. $HOME\export-esp.ps1
```

Lalu install tool deploy:

```powershell
cargo install ldproxy espflash
```

## 3) Clone Repository Template

Clone repo ini:

```powershell
git clone https://github.com/veshblue/Setup_ESP32-S3_Xtensa.git
cd Setup_ESP32-S3_Xtensa
```

## 4) Verifikasi Environment

Cek toolchain yang aktif:

```powershell
rustup show active-toolchain
```

Output yang diharapkan mengandung `esp`.

Kalau belum `esp`, jalankan:

```powershell
. $HOME\export-esp.ps1
rustup show active-toolchain
```

## 5) Build Firmware

Jalankan:

```powershell
cargo check
cargo build
```

Jika dua command ini sukses, artinya project siap deploy.

## 6) Flash ke ESP32-S3

Hubungkan board ke USB, lalu jalankan:

```powershell
cargo run
```

Template ini sudah mengatur runner:

```toml
espflash flash --monitor
```

Jadi `cargo run` akan:

1. Build firmware
2. Flash ke board
3. Buka serial monitor otomatis

## 7) Struktur File Penting

- `src/main.rs` - entry point firmware
- `Cargo.toml` - dependencies dan build profile
- `.cargo/config.toml` - target, rustflags, runner
- `rust-toolchain.toml` - pin channel `esp`
- `ESP32S3_SETUP.md` - catatan setup tambahan

## 8) Troubleshooting (Paling Sering Terjadi)

### Error: `can't find crate for core`

Penyebab: environment/toolchain ESP belum aktif.

Solusi:

```powershell
. $HOME\export-esp.ps1
cargo check
```

### Error akses serial/port tidak ketemu

Penyebab umum:

- Driver USB board belum terinstall
- Kabel USB bukan kabel data
- Port sedang dipakai aplikasi lain

Solusi cepat:

- Ganti kabel USB
- Tutup serial monitor lain
- Cabut/pasang board lalu ulangi `cargo run`

### `cargo run` tidak flash otomatis

Pastikan file `.cargo/config.toml` masih berisi:

- target `xtensa-esp32s3-none-elf`
- runner `espflash flash --monitor`

## 9) Cara Pakai Sebagai Template Project Baru

Kalau kamu ingin bikin project turunan:

1. Clone repo ini
2. Rename folder sesuai nama project baru
3. Ubah `name` di `Cargo.toml`
4. Edit `src/main.rs` sesuai kebutuhan aplikasi

Setelah itu alur pakai tetap sama:

```powershell
cargo check
cargo build
cargo run
```

## 10) Ringkasan Alur Cepat

Kalau mau versi singkat:

```powershell
irm https://esp-rs.github.io/espup/install.ps1 | iex
espup install
# restart terminal
cargo install ldproxy espflash
git clone https://github.com/veshblue/Setup_ESP32-S3_Xtensa.git
cd Setup_ESP32-S3_Xtensa
cargo check
cargo build
cargo run
```
