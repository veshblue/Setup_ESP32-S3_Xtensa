# Setup Rust ke ESP32-S3

## 1) Install toolchain ESP-Rust

Jalankan di PowerShell:

```powershell
irm https://esp-rs.github.io/espup/install.ps1 | iex
espup install
```

Setelah itu restart terminal.

## 2) Install tools build/flash

```powershell
cargo install ldproxy espflash
```

## 3) Build project

Di folder `project`:

```powershell
cargo build
```

## 4) Flash ke board + monitor serial

```powershell
cargo run
```

Perintah `cargo run` akan memakai runner:

```toml
runner = "espflash flash --monitor"
```

sehingga firmware langsung di-flash lalu serial monitor otomatis terbuka.
