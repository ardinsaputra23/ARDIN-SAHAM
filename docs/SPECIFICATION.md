# MASTER PROMPT FINAL V3.0
# ARDIN SAHAM
## Production-Ready Personal Stock Portfolio & Investment Tracker

### INSTRUKSI UTAMA

Bertindak sebagai Senior Desktop Software Engineer, Financial Application Architect, Database Engineer, UI/UX Designer, dan Quality Assurance Engineer.

Bangun aplikasi desktop Windows baru bernama **ARDIN SAHAM**.

Aplikasi ini digunakan secara pribadi untuk mencatat transaksi saham Indonesia, mengelola portofolio, melakukan screening saham berdasarkan aturan investasi pribadi, membuat rencana entry dan exit, mencatat katalis saham, mengevaluasi keputusan investasi, serta mengukur kinerja portofolio.

Saya ingin aplikasi yang benar-benar siap digunakan, stabil, akurat, profesional, dan dapat dipakai selama minimal 5–10 tahun.

**PENTING: Jangan hanya membangun prototype, mockup, dashboard statis, atau sebagian fitur. Implementasikan seluruh fitur yang dijelaskan, integrasikan satu sama lain, lalu lakukan pengujian menyeluruh.**

Jangan mengklaim bahwa aplikasi sudah production-ready apabila ada fungsi utama yang belum selesai atau belum diuji. Laporkan keterbatasan secara transparan.

Gunakan repository `ardin-stock-journal`. Jangan mengubah atau menghapus proyek `ardin-system`.

---

# A. ARSITEKTUR DAN TEKNOLOGI

## 1. Teknologi utama

Gunakan teknologi berikut jika kompatibel dengan lingkungan pengembangan:

- Tauri v2.
- React.
- TypeScript dengan strict mode.
- SQLite lokal.
- Rust backend untuk operasi database dan perhitungan yang memerlukan integritas tinggi.
- Tailwind CSS atau pendekatan styling konsisten lainnya.
- Komponen UI profesional.
- Library grafik yang berfungsi offline.
- Database migrations.
- Unit tests dan integration tests.

Gunakan dependency stabil yang kompatibel.

Jangan menggunakan backend cloud atau database online.

## 2. Offline-first

Aplikasi harus:

1. Berjalan sepenuhnya offline setelah instalasi.
2. Tidak menggunakan API saham.
3. Tidak menggunakan scraping.
4. Tidak mengambil berita secara otomatis.
5. Tidak membutuhkan akun, login, atau autentikasi online.
6. Tidak memerlukan internet untuk membuka aplikasi atau mengakses data.
7. Menyimpan semua informasi pada perangkat pengguna.
8. Tidak menggunakan telemetry atau mengirim data pribadi ke server luar.
9. Menampilkan informasi harga saham sebagai data manual, bukan real-time.
10. Tetap berfungsi ketika laptop tidak tersambung ke internet.

Semua data pasar dan perusahaan dimasukkan oleh pengguna.

Perhitungan dilakukan otomatis berdasarkan data yang tersimpan.

## 3. Identitas aplikasi

Nama resmi: **ARDIN SAHAM**

Subtitle: Personal Stock Portfolio & Investment Tracker.

Nama executable: `Ardin Saham.exe`, mengikuti ketentuan penamaan dan packaging Tauri.

Nama database: `ardin-saham.db`.

Nama aplikasi pada Windows Start Menu dan installer harus ARDIN SAHAM.

Gunakan ikon aplikasi profesional bertema investasi, dengan identitas visual sendiri dan tanpa menggunakan logo perusahaan lain tanpa izin.

## 4. Desain visual

Gunakan:

- Hijau tua sebagai warna primer.
- Latar putih atau netral terang.
- Hijau untuk keuntungan.
- Merah untuk kerugian.
- Abu-abu untuk informasi netral.
- Font profesional yang mudah dibaca.
- Sidebar navigasi modern.
- Kartu ringkasan yang tidak berlebihan.
- Tabel data yang rapi.
- Grafik yang informatif.
- Modal dan formulir yang konsisten.
- Empty states dan loading/error states.

Gunakan format mata uang Indonesia.

Contoh:

Rp 125.750.000

Jangan gunakan singkatan seperti Rp125,7 juta atau 125.7M untuk nominal utama.

Sediakan horizontal scrolling pada tabel yang lebar.

Gunakan desain desktop yang nyaman pada resolusi laptop umum.

Jangan mengorbankan keterbacaan hanya demi estetika.

---

# B. NAVIGASI APLIKASI

Buat delapan menu utama:

1. Dashboard.
2. Portfolio.
3. Transactions.
4. Watchlist.
5. Performance & Evaluation.
6. Dividend Tracker.
7. History & Reports.
8. Settings & Backup.

Pada halaman detail saham, gunakan tab:

- Overview.
- Investment Rules.
- Technical & Fundamental.
- Trading Plan.
- Catalysts & News.
- Transaction History.

Jurnal investasi tidak perlu menjadi menu input terpisah. Jurnal harus menjadi bagian dari setiap transaksi beli dan jual.

Semua tab harus merujuk ke data saham yang sama agar tidak terjadi duplikasi catatan.

---

# C. DATABASE DAN MODEL DATA

## 5. Database

Gunakan SQLite sebagai database utama.

Sediakan tabel atau entitas yang sesuai untuk:

- Stocks.
- Cash Movements.
- Transactions.
- Portfolio Positions.
- Position Cycles.
- Manual Price History.
- Portfolio Valuation Snapshots.
- Watchlist.
- Investment Rules.
- Rule Verifications.
- Technical Analysis.
- Fundamental Analysis.
- Trading Plans.
- Take Profit Targets.
- Catalyst Journal.
- Transaction Decision Journal.
- Transaction Evaluations.
- Dividends.
- Corporate Actions.
- Settings.
- Backup Metadata.

Gunakan foreign keys, constraints, unique indexes, serta database transactions sesuai kebutuhan.

Gunakan migrasi database berversi.

Jangan menghapus atau menimpa data pengguna ketika schema berubah.

Simpan tanggal pembuatan dan pembaruan catatan.

## 6. Presisi finansial

Gunakan integer untuk nominal rupiah yang bulat dan satuan lembar saham.

Untuk nominal yang memerlukan pecahan, seperti fee, gunakan decimal arithmetic atau fixed-point representation.

Jangan menggunakan floating-point tanpa pengendalian presisi untuk ledger keuangan.

Perhitungan harus konsisten, dapat diuji, dan dapat direproduksi dari riwayat transaksi.

---

# D. CASH & PORTFOLIO MANAGEMENT

## 7. Cash ledger

Sediakan:

- Setoran dana.
- Penarikan dana.
- Saldo kas.
- Riwayat mutasi.
- Catatan transaksi kas.

Pembelian saham mengurangi kas.

Penjualan saham menambah kas setelah fee.

Dividen bersih yang benar-benar diterima menambah kas.

Pastikan transaksi tidak dapat menggunakan saldo kas melebihi yang tersedia, kecuali ada mekanisme saldo awal yang valid.

Setoran dan penarikan dana bukan keuntungan atau kerugian investasi.

## 8. Portfolio

Tampilkan:

- Kode saham.
- Nama perusahaan.
- Sektor.
- Jumlah lot.
- Jumlah lembar saham.
- Average buy price.
- Total biaya perolehan.
- Harga pasar manual terakhir.
- Tanggal pembaruan harga.
- Nilai pasar.
- Unrealized P/L rupiah.
- Unrealized P/L persentase.
- Bobot posisi terhadap nilai saham.
- Bobot posisi terhadap total ekuitas.
- TP1, TP2, TP3.
- Stop-loss.
- Status posisi.

Pisahkan posisi aktif dan posisi yang sudah ditutup.

Jangan menghapus riwayat posisi tertutup.

Sediakan fitur opening balance untuk portofolio lama, termasuk harga perolehan dan jumlah lot awal.

---

# E. MULTIPLE BUY, AVERAGE DOWN & AVERAGE UP

## 9. Pembelian berkali-kali

Aplikasi wajib mendukung pembelian saham yang sama lebih dari satu kali.

Contoh:

Pembelian pertama:
- NCKL.
- Harga Rp1.000.
- 100 lot.

Pembelian kedua:
- Harga Rp900.
- 100 lot.

Pembelian ketiga:
- Harga Rp800.
- 200 lot.

Total 400 lot atau 40.000 lembar.

Total biaya pembelian sebelum fee Rp35.000.000.

Average buy price sebelum fee Rp875.

Setiap transaksi harus disimpan secara terpisah, tetapi digabungkan ke satu posisi portofolio aktif.

## 10. Klasifikasi pembelian tambahan

Tampilkan:

- Average Down: pembelian di bawah average cost sebelumnya.
- Average Up: pembelian di atas average cost sebelumnya.
- Additional Buy: pembelian pada harga setara.

Gunakan perbandingan berbasis biaya perolehan dengan definisi yang jelas.

Pada setiap pembelian tambahan, hitung ulang:

- Total lot.
- Total lembar.
- Total biaya perolehan.
- Weighted average price.
- Unrealized profit/loss.
- Bobot saham dalam portofolio.
- Konsentrasi risiko.
- Rencana take profit yang masih berlaku.

Setiap pembelian tambahan wajib mempunyai jurnal keputusan sendiri.

Jangan menghapus jurnal pembelian sebelumnya.

## 11. Metode average cost

Gunakan weighted average cost yang memperhitungkan biaya pembelian.

Penjualan sebagian harus mengurangi basis biaya secara proporsional.

Penjualan sebagian tidak boleh mengubah average cost hanya karena jumlah saham berkurang.

Jika seluruh posisi dijual, posisi ditutup.

Jika saham yang sama dibeli kembali setelah seluruh posisi tertutup, buat position cycle baru dengan average cost baru.

Riwayat siklus sebelumnya tetap tersimpan.

Jika transaksi historis diedit, seluruh transaksi setelahnya harus dihitung ulang secara kronologis.

Tolak perubahan yang menghasilkan posisi negatif atau keadaan ledger tidak konsisten.

---

# F. BUY & SELL TRANSACTION JOURNAL

## 12. Formulir pembelian

Setiap pembelian mempunyai:

DATA:
- Tanggal.
- Kode saham.
- Harga beli.
- Jumlah lot.
- Fee broker.
- Nilai transaksi.
- Total biaya.

JURNAL:
- Mengapa membeli saham ini?
- Apa yang menarik dari fundamentalnya?
- Apa alasan teknikal entry?
- Apakah membeli di support?
- Apa katalis positifnya?
- Risiko apa yang diketahui?
- Berapa target keuntungan?
- Berapa batas kerugian?
- Berapa tingkat keyakinan?

Strategi:

- Swing Trading.
- Position Trading.
- Long-Term Investing.

Sediakan catatan bebas dan lampiran lokal opsional.

Kolom jurnal dapat dilengkapi setelah transaksi tersimpan.

## 13. Formulir penjualan

Setiap penjualan mempunyai:

DATA:
- Tanggal.
- Harga jual.
- Jumlah lot.
- Fee broker.
- Hasil bersih.
- Realized profit/loss.
- Return.
- Durasi kepemilikan.

JURNAL:
- Mengapa menjual saham?
- Apakah target profit tercapai?
- Apakah stop-loss tercapai?
- Apakah fundamental berubah?
- Apakah ada katalis negatif?
- Apakah keputusan sesuai trading plan?
- Apakah penjualan dilakukan karena emosi?

Sediakan evaluasi setelah transaksi.

## 14. Evaluasi kesalahan

Kategori pembelian:

- FOMO.
- Salah membaca support.
- Membeli terlalu cepat.
- Tidak menunggu konfirmasi.
- Salah menilai fundamental.
- Position sizing terlalu besar.
- Averaging down tanpa rencana.
- Mengabaikan risiko.
- Kesalahan lainnya.

Kategori penjualan:

- Panic selling.
- Take profit terlalu cepat.
- Terlambat cut loss.
- Menjual tanpa alasan jelas.
- Mengabaikan trading plan.
- Salah membaca pergerakan pasar.
- Kesalahan lainnya.

Pengguna dapat memilih beberapa kategori.

Simpan:

- Kesalahan.
- Penjelasan.
- Tindakan yang seharusnya dilakukan.
- Pelajaran.
- Tanggal evaluasi.

Evaluasi boleh dilakukan belakangan dan diperbarui.

Pisahkan kualitas keputusan dari hasil keuntungan/kerugian.

---

# G. WATCHLIST & INVESTMENT RULES

## 15. Watchlist

Setiap saham watchlist mempunyai:

- Kode.
- Nama.
- Sektor.
- Harga referensi manual.
- Tanggal harga.
- Target entry.
- Support.
- Resistance.
- Stop-loss.
- Potensi upside.
- Potensi downside.
- Status analisis.
- Catatan.

Status:

- Baru.
- Sedang dianalisis.
- Menunggu entry.
- Memenuhi persyaratan yang diverifikasi.
- Sudah dibeli.
- Dibatalkan.

Sediakan pencarian, filter, sorting, dan pengarsipan.

## 16. Nine Investment Rules

Buat panel **MY INVESTMENT RULES** yang selalu dapat diakses di halaman Watchlist.

Aturan pribadi:

1. Saham harus syariah.
2. Free float kurang dari 25%.
3. Volatilitas ekstrem lebih dari dua kali dalam satu tahun.
4. Harga saat ini berada pada kisaran level harga dari lima tahun terakhir.
5. Pembelian harus direncanakan di support kuat atau bottom.
6. SSID menunjukkan penurunan selama enam bulan.
7. Utamakan saham sektor komoditas.
8. Terdapat indikasi big accumulation.
9. Perusahaan menghasilkan dividen.

Setiap saham memiliki checklist independen.

Status:

- Terpenuhi.
- Tidak terpenuhi.
- Belum diverifikasi.

Sediakan:

- Nilai data.
- Sumber verifikasi.
- Tanggal verifikasi.
- Catatan.
- Dokumen atau bukti lokal opsional.

Syariah dan free float menjadi syarat wajib.

Komoditas menjadi preferensi.

Sediakan konfigurasi untuk menentukan aturan lain sebagai wajib atau preferensi.

Jangan menganggap data yang belum diverifikasi sebagai memenuhi syarat.

Karena aplikasi offline, status syariah dan free float harus diverifikasi dan diperbarui manual oleh pengguna.

Untuk volatilitas ekstrem, sediakan parameter definisi pergerakan ekstrem. Jangan menentukan bahwa saham memenuhi aturan tanpa data yang memadai.

Untuk SSID dan big accumulation, jangan mengklaim ada akumulasi institusional hanya berdasarkan perubahan jumlah pemegang saham.

---

# H. TECHNICAL & FUNDAMENTAL ANALYSIS

## 17. Fundamental manual

Sediakan kolom untuk:

- Revenue.
- Net income.
- EPS.
- Book value per share.
- PBV.
- PER.
- ROE.
- DER.
- Dividend per share.
- Dividend yield.
- Free float.
- Catatan kualitas bisnis.

Pengguna boleh menginput rasio secara langsung atau data dasar untuk perhitungan otomatis.

Tandai apakah nilai berasal dari input langsung atau hasil perhitungan.

Simpan periode laporan dan tanggal pembaruan.

Jangan mencampurkan data dari periode berbeda tanpa penanda.

## 18. Technical analysis manual

Sediakan:

- Harga terkini.
- Support 1 dan Support 2.
- Resistance 1 dan Resistance 2.
- Area bottom menurut analisis pengguna.
- Highest price 52 minggu.
- Lowest price 52 minggu.
- Volume dan catatan volume.
- Indikasi breakout.
- Indikasi accumulation.
- Target entry.
- Stop-loss.
- Risk/reward ratio.

Semua input manual.

Jika tidak tersedia data historis OHLCV yang cukup, jangan menghitung otomatis indikator yang membutuhkan rangkaian harga.

Sediakan fitur impor OHLCV CSV opsional agar pengguna dapat memasukkan data historis tanpa API.

---

# I. THREE-LEVEL TAKE PROFIT

## 19. TP1, TP2, TP3

Setiap saham memiliki tiga target jual.

Untuk setiap target:

- Harga target.
- Persentase kenaikan dari average cost.
- Persentase posisi yang direncanakan dijual.
- Jumlah lot.
- Estimasi nilai penjualan.
- Estimasi biaya.
- Estimasi laba atau rugi bersih.
- Persentase nilai penjualan terhadap total ekuitas portofolio.
- Sisa lot setelah penjualan.
- Status target.
- Catatan.

Persentase posisi harus dihitung dari jumlah lot yang tersedia dengan aturan basis alokasi yang konsisten.

Total alokasi tidak boleh melebihi 100%.

Gunakan jumlah lot bulat untuk transaksi papan reguler.

Jika perhitungan persentase menghasilkan pecahan lot, tampilkan pembulatan yang jelas dan izinkan penyesuaian manual.

Contoh:

Kepemilikan 200 lot.

TP1:
- Harga Rp1.100.
- Jual 25% atau 50 lot.

TP2:
- Harga Rp1.200.
- Jual 35% atau 70 lot.

TP3:
- Harga Rp1.350.
- Jual 40% atau 80 lot.

Target harga yang tercapai tidak otomatis berarti saham sudah dijual.

Status penjualan selesai hanya setelah ada transaksi jual yang dicatat.

## 20. Penyesuaian setelah average down

Jika pengguna melakukan pembelian tambahan, tampilkan:

- Average price baru.
- Persentase keuntungan baru pada setiap TP.
- Total lot baru.
- Bobot portofolio baru.
- Perubahan estimasi profit.
- Apakah rencana TP membutuhkan penyesuaian.

Jangan mengubah rencana lama secara diam-diam.

Simpan riwayat perubahan trading plan.

## 21. Stop-loss

Sediakan:

- Harga stop-loss.
- Persentase kerugian.
- Estimasi kerugian rupiah.
- Risk/reward ratio terhadap tiap TP.
- Alasan invalidasi.
- Catatan disiplin risiko.

Stop-loss hanya merupakan rencana, bukan order otomatis.

---

# J. CATALYST & NEWS JOURNAL

## 22. Katalis positif dan negatif

Setiap saham mempunyai dua kelompok:

**Positive Catalyst**

**Negative Catalyst**

Pengguna mencatat berita secara manual.

Setiap catatan berisi:

- Tanggal kejadian.
- Tanggal pencatatan.
- Judul berita.
- Jenis positif/negatif.
- Kategori berita.
- Sumber.
- URL sumber opsional.
- Ringkasan.
- Analisis pribadi.
- Estimasi dampak.
- Status.
- Evaluasi setelah kejadian.

Kategori:

- Laporan keuangan.
- Ekspansi.
- Kontrak baru.
- Komoditas.
- Regulasi.
- Dividen.
- Corporate action.
- Manajemen.
- Makroekonomi.
- Lainnya.

Status:

- Masih relevan.
- Terealisasi.
- Kedaluwarsa.
- Dibatalkan.

Sediakan pengurutan, filter, edit, dan arsip.

Catatan katalis tetap tersimpan setelah seluruh saham dijual.

Dukung hubungan katalis dengan transaksi beli atau jual.

Jangan membuat berita atau analisis otomatis yang tidak didukung data.

---

# K. DIVIDEND TRACKER

## 23. Dividen

Sediakan:

- Kode saham.
- Cum date.
- Ex date.
- Recording date.
- Payment date.
- Dividend per share.
- Jumlah lembar berhak.
- Dividen bruto.
- Potongan relevan.
- Dividen bersih.
- Status pembayaran.

Hitung dividen berdasarkan jumlah saham yang memenuhi ketentuan kepemilikan.

Jumlah lembar yang berhak dapat dikoreksi manual apabila diperlukan, dengan catatan alasan.

Pisahkan dividen diumumkan, diharapkan, dan diterima.

Hanya dividen diterima yang memengaruhi kas.

Buat laporan dividen bulanan dan tahunan.

---

# L. PERFORMANCE & EVALUATION

## 24. Performance dashboard

Tampilkan:

- Realized P/L.
- Unrealized P/L.
- Total return.
- Total dividen.
- Win rate.
- Average win.
- Average loss.
- Profit factor.
- Average holding period.
- Best trade.
- Worst trade.
- Jumlah posisi ditutup.
- Jumlah saham aktif.
- Alokasi per sektor.
- Konsentrasi per saham.

Sediakan definisi perhitungan untuk setiap metrik.

Win rate harus menggunakan unit trade atau siklus posisi yang jelas, agar penjualan bertahap tidak menyebabkan penghitungan ganda.

Untuk return portofolio, bedakan return sederhana dengan metrik yang mempertimbangkan setoran dan penarikan dana.

Jika data tidak cukup, tampilkan status data belum cukup.

## 25. Evaluasi strategi dan disiplin

Tampilkan:

- Kesalahan pembelian paling sering.
- Kesalahan penjualan paling sering.
- Persentase keputusan sesuai trading plan.
- Performa transaksi disiplin.
- Performa transaksi tidak disiplin.
- Perbandingan strategi swing dan long-term.
- Pola average down.
- Dampak konsentrasi posisi.
- Evaluasi bulanan dan tahunan.

Semua evaluasi kuantitatif harus berasal dari data yang benar-benar tersedia.

Jangan membuat kesimpulan kausal tanpa bukti.

---

# M. REPORTS & HISTORY

## 26. Laporan

Buat laporan:

- Transaksi bulanan.
- Transaksi tahunan.
- Realized P/L.
- Unrealized P/L.
- Dividen.
- Portofolio per periode.
- Jurnal kesalahan.
- Katalis.
- Kinerja per saham.
- Kinerja per strategi.

Sediakan filter waktu.

Dukung ekspor CSV dan XLSX.

Gunakan angka rupiah penuh.

Sediakan riwayat harga manual dan snapshot portofolio.

Jangan membuat harga historis fiktif untuk melengkapi grafik.

---

# N. BACKUP, RESTORE & PRIVACY

## 27. Perlindungan data

Database utama harus berada di direktori data aplikasi Windows yang sesuai dan bukan di folder temporary.

Sediakan:

- Backup manual.
- Backup otomatis lokal jika memungkinkan.
- Restore.
- Ekspor.
- Impor.
- Validasi backup.
- Konfirmasi tindakan destruktif.

Backup SQLite harus konsisten, menggunakan mekanisme backup database yang aman.

Sebelum restore, buat backup kondisi database saat ini.

Data tidak boleh hilang setelah update aplikasi.

Jangan menyertakan data pengguna di GitHub repository.

Sediakan petunjuk memindahkan data ke komputer lain.

---

# O. TESTING

## 28. Pengujian wajib

Lakukan pengujian unit dan integrasi untuk:

1. Pembelian pertama.
2. Pembelian saham yang sama berkali-kali.
3. Average down.
4. Average up.
5. Pembelian dengan fee berbeda.
6. Penjualan sebagian.
7. Pembelian tambahan setelah penjualan sebagian.
8. Penjualan seluruh posisi.
9. Pembukaan siklus baru.
10. Edit transaksi historis.
11. Hapus transaksi historis.
12. Validasi saldo kas.
13. Larangan overselling.
14. Perhitungan realized P/L.
15. Perhitungan unrealized P/L.
16. Alokasi TP1–TP3.
17. Pembulatan lot.
18. Penyesuaian TP setelah average down.
19. Stop-loss.
20. Investment Rules.
21. Status syariah belum diverifikasi.
22. Katalis positif dan negatif.
23. Jurnal pembelian dan penjualan.
24. Evaluasi kesalahan.
25. Dividen.
26. Corporate action dasar.
27. Laporan performa.
28. Persistensi database.
29. Backup dan restore.
30. Aplikasi tanpa internet.

Tambahkan tes untuk kasus batas, transaksi tidak berurutan, input kosong, data tidak valid, dan posisi yang sudah ditutup.

Pastikan semua tampilan membaca sumber data yang konsisten.

---

# P. BUILD WINDOWS DAN KRITERIA SELESAI

## 29. Build aplikasi

Target akhir adalah installer Windows yang benar-benar dapat digunakan.

Jika build Windows dapat dilakukan di lingkungan yang tersedia, lakukan build dan uji hasilnya.

Jika tidak, buat workflow GitHub Actions menggunakan Windows runner agar installer dapat dibangun.

Jangan menyatakan installer berhasil dibuat apabila belum ada hasil build yang terverifikasi.

Sediakan instruksi mengunduh artifact, memasang aplikasi, dan menjalankan aplikasi untuk pertama kali.

## 30. Definition of Done

Aplikasi dianggap selesai hanya jika:

- Navigasi berfungsi.
- Formulir input berfungsi.
- Data tersimpan permanen.
- Transaksi saham dihitung dengan benar.
- Average down/up berfungsi.
- Penjualan sebagian berfungsi.
- Jurnal transaksi tersimpan.
- Watchlist dan checklist berfungsi.
- TP1–TP3 berfungsi.
- Katalis tersimpan.
- Dividen dihitung dengan benar.
- Laporan konsisten.
- Backup dan restore teruji.
- Aplikasi dapat digunakan offline.
- Tidak ada tombol dekoratif yang tampak aktif tetapi tidak berfungsi.
- Tidak ada data contoh yang dianggap sebagai data pengguna.
- Tidak ada error kritis yang diketahui.
- Build dan keterbatasannya dilaporkan dengan jujur.

---

# Q. INSTRUKSI EKSEKUSI FINAL

Kerjakan pengembangan secara menyeluruh dalam urutan berikut:

FASE 1:
- Audit repository.
- Inisialisasi aplikasi.
- Konfigurasi Tauri/React/SQLite.
- Desain dasar.
- Migrasi database.

FASE 2:
- Cash ledger.
- Portfolio.
- Buy/sell transactions.
- Multiple buy.
- Average down/up.
- Financial calculation engine.

FASE 3:
- Transaction journal.
- Watchlist.
- Investment rules.
- Fundamental dan teknikal.
- Trading plan.
- TP1–TP3.
- Stop-loss.

FASE 4:
- Catalyst journal.
- Dividends.
- Performance.
- Evaluations.
- Reports.

FASE 5:
- Backup/restore.
- Unit testing.
- Integration testing.
- Bug fixing.
- Windows packaging.
- Dokumentasi penggunaan.

Kerjakan seluruh fase yang memungkinkan dalam sesi dan lingkungan yang tersedia.

Setelah setiap fase, verifikasi integritas data dan hubungan antarfitur.

Jangan mengganti fitur yang sulit dibuat dengan tombol palsu, mockup, atau placeholder.

Jangan menghapus fitur dari spesifikasi secara diam-diam.

Jika lingkungan, kuota, atau waktu eksekusi membatasi penyelesaian, simpan perubahan kode yang valid, laporkan pekerjaan yang belum selesai, dan jangan menyatakan aplikasi sudah final.

Pada akhir pekerjaan, berikan laporan lengkap:

1. Fitur yang selesai.
2. Fitur yang belum selesai.
3. Hasil pengujian.
4. Hasil build.
5. File installer yang tersedia.
6. Cara menjalankan aplikasi.
7. Cara backup dan restore.
8. Risiko atau masalah yang masih diketahui.

**HASIL YANG SAYA INGINKAN:**

ARDIN SAHAM adalah aplikasi desktop Windows pribadi yang stabil, profesional, sepenuhnya offline, memiliki perhitungan investasi yang akurat, dan dapat digunakan setiap hari untuk mengelola portofolio saham, membuat keputusan investasi, mencatat seluruh transaksi, serta mengevaluasi kesalahan dan perkembangan kemampuan investasi saya selama bertahun-tahun.

Mulai implementasi pada repository yang dipilih sekarang.
