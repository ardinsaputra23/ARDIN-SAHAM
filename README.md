# ARDIN SAHAM

Personal Stock Portfolio & Investment Tracker untuk saham Indonesia. Aplikasi desktop lokal berbasis Tauri v2, React/TypeScript strict dan Rust/SQLite. **Versi pengembangan, belum siap produksi.** Tidak menggunakan API saham, login, scraping, atau telemetry. Harga dan analisis dimasukkan manual.

Hasil sesi terbaru tersedia di [audit 12 November 2026](docs/AUDIT-2026-11-12.md).

Lihat [status implementasi dan keterbatasan](docs/STATUS.md) sebelum menggunakan data penting. Spesifikasi lengkap tersedia di [docs/SPECIFICATION.md](docs/SPECIFICATION.md).

## Menjalankan di Windows

Prasyarat pengembangan: Node.js 24, Rust stable (MSVC), Microsoft C++ Build Tools dengan Windows SDK, dan WebView2 Runtime. Gunakan checkout yang tersedia; task cloud sudah terisolasi dan tidak memerlukan worktree baru.

```powershell
npm ci
cargo test --locked --manifest-path core/Cargo.toml
npm run tauri -- dev
```

`npm run dev` saja hanya membuka frontend; database aplikasi membutuhkan host Tauri. Tidak ada fallback localStorage atau data contoh yang dianggap sebagai data pengguna.

```powershell
npm run tauri -- build --bundles nsis -- --locked
```

Build yang berhasil menghasilkan installer di `src-tauri/target/release/bundle/nsis/`. Nama produk dan Start Menu: **ARDIN SAHAM**. Konfigurasi nama binary: `Ardin Saham.exe`. Installer dikonfigurasi menyertakan pemasang WebView2 offline. Build installer belum terverifikasi dalam lingkungan ini; jangan menganggap artifact tersedia sebelum workflow berhasil.

## GitHub Actions

Workflow `.github/workflows/windows.yml` menjalankan tes core, build frontend, tes browser dengan backend nyata, kemudian build NSIS pada Windows runner. Setelah kode berada di GitHub, buka **Actions → Windows validation and installer → Run workflow**. Unduh artifact `ARDIN-SAHAM-Windows-unverified-installer` hanya dari run yang berhasil. Workflow ini tidak menerbitkan release dan tidak menandatangani installer. Jalankan installer dan smoke test Windows secara manual sebelum distribusi: buka tanpa jaringan, simpan transaksi, tutup/buka ulang, backup dan restore. Build sukses tidak menggantikan tes pemasangan tersebut.

## Alur awal

1. Dashboard → Tambah saham: kode, nama perusahaan, sektor.
2. Catat `DEPOSIT` sebelum membeli, atau `OPENING` untuk saham yang sudah dimiliki. Opening menambah kontribusi biaya perolehan tanpa mengurangi kas.
3. `BUY`/`SELL`: lot bulat (100 lembar), harga per lembar, fee dalam nominal rupiah total. Jurnal boleh dilengkapi nanti.
4. Portfolio → catat harga manual. Tanpa harga, nilai pasar/P&L ditandai belum lengkap.
5. Pilih saham untuk membuka tab analisis, rules, rencana TP, katalis dan riwayat.
6. Dividen: buat jadwal, isi cum date, payment date, DPS dan potongan. Klik **Terima dividen** hanya setelah dana diterima. Lembar otomatis memakai kepemilikan pada cum date; koreksi memerlukan alasan. Perhitungan ini tidak memodelkan kalender settlement bursa.

## Perhitungan

Ledger diputar ulang berdasarkan tanggal `YYYY-MM-DD`, lalu ID (urutan pencatatan pada tanggal sama). Pembelian memakai weighted average cost termasuk fee. Penjualan mengalokasikan biaya proporsional; penjualan terakhir menghabiskan seluruh sisa biaya. Kas dan nilai transaksi memakai `rust_decimal`, bukan floating point. Hasil pembagian biaya memakai presisi desimal library (hingga 28 digit); frontend hanya memformat/menampilkan nilai, dan sebagian agregasi tampilan masih memakai JavaScript Number. Ini masih perlu diperketat sebelum produksi.

Win rate dan statistik kemenangan menggunakan **siklus posisi yang sudah ditutup**, bukan setiap penjualan parsial. TP memakai persentase dari seluruh lot aktif saat ini; lot dibulatkan ke bawah sehingga bisa ada sisa yang belum dialokasikan. Fee TP menggunakan basis points: 25 bps = 0,25%. Perubahan TP membuat revisi baru. Target dan stop-loss tidak mengeksekusi order.

## Backup dan restore

Database `ardin-saham.db` disimpan dalam direktori data aplikasi Tauri untuk identifier `id.ardin.saham` (umumnya `%APPDATA%\id.ardin.saham` di Windows). Data tidak disimpan dalam checkout atau temporary directory.

- Settings & Backup → Backup database: isi path lengkap **file baru**, misalnya `C:\Users\Nama\Documents\ardin-2026-11-12.db`. Folder induk harus sudah ada. SQLite Backup API membuat salinan konsisten.
- Restore database: isi path backup sumber dan path file baru untuk backup pengaman. Konfirmasi restore. Aplikasi memeriksa integritas, versi schema, relasi, dan replay ledger, lalu membuat backup pengaman sebelum restore.
- Memindahkan komputer: backup dari aplikasi lama, salin file backup ke komputer baru, pasang versi aplikasi yang kompatibel, lalu restore. Simpan salinan asli sampai hasil diverifikasi.
- Backup otomatis, pembatalan penerimaan dividen, dan pemulihan dari kegagalan disk mendadak belum tersedia/teruji. Jangan memakai database ini sebagai satu-satunya catatan keuangan.

## Validasi pengembangan

```sh
npm run build
cargo test --locked --manifest-path core/Cargo.toml
npx playwright install chromium
npm run test:e2e
```

Di lingkungan cloud saat ini, Rust terpasang di `/workspace/.ardin-tools`. Untuk mengaktifkannya:

```sh
export CARGO_HOME=/workspace/.ardin-tools/cargo
export RUSTUP_HOME=/workspace/.ardin-tools/rustup
export PATH="$CARGO_HOME/bin:$PATH"
CHROMIUM_PATH=/usr/bin/chromium npm run test:e2e
```

Tes browser memakai adaptor **khusus pengujian** yang memanggil core Rust dan SQLite nyata. Database tes terpisah di direktori sementara. Adaptor tidak dikemas dalam aplikasi dan tidak membuktikan komunikasi native Tauri/WebView2. Tes backup dan persistensi berjalan sebagai integration tests Rust.
