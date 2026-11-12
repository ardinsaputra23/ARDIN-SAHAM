# Status terbaru — 12 November 2026

Lihat [audit terbaru](AUDIT-2026-11-12.md): 22 tes Rust, 2 skenario browser termasuk offline, build frontend lulus. Kode baseline sudah di-commit lokal. Installer Windows belum tersedia. Catatan di bawah adalah baseline sebelum audit; angka pengujiannya bersifat historis.

# Status implementasi

Status: pengembangan sebagian, **bukan production-ready**. Repository yang dipilih pengguna: `ardinsaputra23/ARDIN-SAHAM`. Tidak ada perubahan pada `ardin-system`. Tidak ada push, release, publikasi lingkungan, atau installer terverifikasi dari sesi ini.

## Implementasi yang tersedia

- React strict, delapan menu, formulir modal, tabel scroll horizontal, empty/error states dan format rupiah penuh.
- Host Tauri, data directory permanen, core Rust mandiri, schema SQLite v1 dengan foreign keys dan migrasi transaksional; menolak schema versi lebih baru.
- Master saham, kas setoran/penarikan, opening position, BUY/SELL, jurnal bebas tiap transaksi, edit/hapus historis dengan replay dan penolakan ledger negatif.
- Multiple buy, average down/up/additional buy berdasarkan biaya termasuk fee, partial sell, close dan reopen cycle.
- Catatan manual watchlist, fundamental, teknikal, evaluasi, katalis, verifikasi aturan, harga, trading plan dan dividen tersimpan dalam records dengan referensi saham yang sama.
- Verifikasi tiga status dengan nilai/sumber wajib; syariah dan free float wajib untuk indikator screening; free float <25% divalidasi. Belum terverifikasi tidak dianggap lulus.
- TP1–TP3: validasi backend, estimasi lot/proceeds/profit/remaining lots, return, stop-loss dan risk/reward; rencana direvisi dengan append, bukan overwrite.
- Penerimaan dividen atomik, pengecekan lembar pada cum date, koreksi dengan alasan, potongan, kredit kas dan pencegahan klik penerimaan ganda. Receipt dan transaksi terkait dikunci dari edit/hapus sampai fitur koreksi khusus tersedia.
- Realized/unrealized, cash/equity sederhana, siklus ditutup, win rate; backend menghitung average win/loss, profit factor, holding days dan best/worst cycle. Sebagian statistik tambahan belum ditampilkan.
- Ekspor CSV transaksi dengan filter tanggal dan perlindungan formula spreadsheet.
- SQLite backup, pemeriksaan backup dan backup pengaman sebelum restore. Tes persistensi, rollback dan restore.
- Ikon asli, konfigurasi NSIS/Windows dan workflow CI (belum dieksekusi di GitHub).

## Belum selesai sesuai spesifikasi

| Area                  | Pekerjaan tersisa                                                                                                                                                                                                             |
| --------------------- | ----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------- |
| Database              | Model terstruktur dan constraints lengkap per entitas; pengujian migrasi multi-versi, audit trail edit transaksi, recovery crash/disk penuh, uji konkurensi multi-proses, single-instance guard           |
| Presisi               | Agregasi tampilan masih memakai Number; pindahkan seluruh valuation/return ke decimal backend; uji batas overflow dan pembagian berulang                                                                                      |
| Portfolio             | Bobot per saham/sektor/ekuitas lengkap, konsentrasi risiko, TP/SL pada tabel utama, grafik, snapshot periode dan time-travel ledger                                                                                           |
| Journal               | Field terstruktur semua pertanyaan beli/jual, strategi enum, confidence, lampiran lokal, multi-select kesalahan dan relationship yang dibatasi FK                                                                             |
| Watchlist             | Arsip, sorting/filter analisis lengkap, upside/downside otomatis, status otomatis setelah pembelian                                                                                                                           |
| Rules                 | Konfigurasi wajib/preferensi untuk aturan 3–9, pengaturan volatilitas, bukti lampiran, masa berlaku verifikasi dan riwayat UI yang lebih jelas                                                                                |
| Fundamental/technical | Validasi angka lengkap, derivasi rasio dan sumber per nilai, periode pelaporan terstruktur, OHLCV CSV; tidak ada indikator historis buatan                                                                                    |
| Trading plan          | Alokasi lot manual, persentase terhadap ekuitas, target status terkait transaksi jual, baseline posisi saat rencana dibuat, notifikasi perubahan setelah averaging, rencana per cycle                                         |
| Catalysts             | Enum/form terstruktur, arsip/filter, relasi transaksi yang diverifikasi dan lampiran                                                                                                                                          |
| Dividend              | Kalender settlement aktual, annual/monthly reports, pembatalan/koreksi receipt, validasi semua tanggal, update hak setelah edit historis; manual DIVIDEND masih tersedia dan tidak boleh digunakan untuk menduplikasi receipt |
| Corporate action      | Stock split, reverse split, rights issue dan corporate action lainnya belum diimplementasikan                                                                                                                                 |
| Performance           | TWR/XIRR atau return berbasis arus kas, alokasi sektor, strategi/disiplin, kesalahan terpopuler, evaluasi bulanan/tahunan, pembeda break-even lebih rinci                                                                     |
| Reports               | XLSX, laporan per saham/strategi/periode, snapshot historis, impor ledger dan validasi impor; CSV saat ini hanya transaksi                                                                                                    |
| Data protection       | Backup otomatis/retensi, file picker native, recovery disk gagal, robust single-instance/multi-process operation, restore fuzz/corruption tests                                                                               |
| UI                    | Navigasi keyboard/focus trap modal lengkap, loading per tindakan, layout multi-resolusi, seluruh statistik dan data provenance, error recovery                                                                                |
| Release               | Native build/IPC smoke test, Windows install/uninstall/update/offline tests, signing, artifact verifikasi dan user guide final                                                                                                |

## Bukti validasi

- 18 integration tests Rust lulus setelah penambahan penerimaan dividen: transaksi, average cost, fee, cash, overselling, cycles, tanggal, TP, rules, schema, persistensi dan backup/restore.
- Build TypeScript strict + Vite berhasil. Tidak membuktikan native desktop build.
- Tes browser menggunakan core Rust nyata: alur input saham, deposit, repeated buy, penolakan overselling, reload dan default rules lulus. Cakupan TP1–TP3 dan penerimaan dividen juga lulus dalam skenario browser yang sama (1 skenario menyeluruh).
- `cargo check` host Tauri gagal pada `glib-sys`: `glib-2.0.pc` tidak tersedia. GTK/WebKit development dependencies juga tidak tersedia; helper pemasangan sistem dan sudo tidak tersedia di environment.
- Workflow Windows tersimpan tetapi belum dijalankan. Belum ada file installer yang dapat diserahkan.
- Eksekusi desktop tanpa jaringan, update aplikasi dan pemulihan crash belum diuji. Kode aplikasi tidak mengambil data dari layanan eksternal; itu belum sama dengan lulus uji offline end-to-end.

## Prioritas berikutnya

1. Jalankan build dan smoke test native pada Windows; perbaiki error Tauri/installer bila ada.
2. Tuntaskan integritas data: migration/backup recovery, multi-process writes, semua perhitungan finansial di backend, histori receipt dan edit ledger.
3. Lengkapi model terstruktur journal/rules/plan/dividend/corporate actions serta pengujian edge cases.
4. Lengkapi performance, reports/XLSX, snapshots dan seluruh UI sesuai spesifikasi.
5. Uji offline dan instalasi/update pada Windows sebelum menyebut aplikasi final.
