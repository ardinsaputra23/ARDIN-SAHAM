ARDIN SAHAM — Windows x64 portabel

CARA MEMAKAI
1. Unduh artifact ARDIN-SAHAM-Windows-x64-portable dari workflow yang berhasil.
2. Klik kanan ZIP → Extract All / Ekstrak Semua ke folder pribadi yang dapat ditulis.
3. Buka Ardin Saham.exe dari folder hasil ekstraksi. Jangan menjalankannya langsung dari dalam ZIP.
4. Tidak perlu memasang ARDIN SAHAM, Node.js, Rust, atau menjalankan server.

KEBUTUHAN WINDOWS
Windows 10/11 x64 dengan Microsoft Edge WebView2 Runtime yang sudah tersedia.
Paket ini tidak menyertakan WebView2. Jika komponen tersebut belum tersedia, aplikasi
belum bisa digunakan; pemasangan komponen Windows tersebut merupakan prasyarat
terpisah. Tidak ada pengunduhan runtime otomatis oleh aplikasi portabel.

PENGGUNAAN OFFLINE
Seluruh pencatatan, perhitungan, jurnal dan database bekerja lokal. Harga saham
diisi manual. Tidak membutuhkan akun, API saham, atau koneksi internet saat digunakan.
Unduh ZIP hanya memerlukan internet pada tahap memperoleh aplikasi.

DATA DAN UPDATE
Data disimpan di %APPDATA%\id.ardin.saham\ardin-saham.db pada akun Windows saat ini.
Data tidak disimpan di dalam ZIP atau folder aplikasi. Menghapus/mengganti folder
aplikasi tidak menghapus database. Jangan menghapus direktori data tersebut.
Sebelum update, buat backup dari Settings & Backup, tutup aplikasi, lalu ekstrak
versi baru ke folder berbeda dan jalankan. Jangan menjalankan dua versi bersamaan.
Untuk pindah komputer atau akun Windows, gunakan backup/restore dari aplikasi;
menyalin folder program saja tidak membawa data.

BACKUP
Settings & Backup → Backup database: isi path lengkap file baru, misalnya
C:\Users\Nama\Documents\ardin-backup.db. Folder induk harus sudah tersedia.
Simpan salinan backup terpisah dari komputer utama. Untuk restore, gunakan menu
Restore dan tentukan path baru untuk backup pengaman.

CATATAN VERSI
Status TP menunjukkan perbandingan harga manual, bukan konfirmasi penjualan.
Fitur impor CSV/XLSX, indikator historis, audit, dan tema dari pengembangan yang
dilaporkan sebelumnya belum ditemukan pada sumber repository yang tersedia.
Aplikasi belum ditandatangani digital. Periksa sumber unduhan dan BUILD-INFO.json.
SHA256SUMS.txt mencatat checksum berkas untuk mendeteksi perubahan/kerusakan.
