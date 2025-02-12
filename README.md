# SPP Sekolah Management System

Aplikasi manajemen SPP Sekolah berbasis web menggunakan React, TypeScript, dan Firebase.

## Persiapan Awal

### 1. Konfigurasi Firebase

1. Buat project baru di [Firebase Console](https://console.firebase.google.com/)
2. Aktifkan Authentication dengan Email/Password
3. Aktifkan Firestore Database
4. Salin kredensial Firebase dari Project Settings
5. Buat file `.env` di root folder dan isi dengan kredensial Firebase anda sendiri :

VITE_FIREBASE_API_KEY=your_api_key
VITE_FIREBASE_AUTH_DOMAIN=your_auth_domain
VITE_FIREBASE_PROJECT_ID=your_project_id
VITE_FIREBASE_STORAGE_BUCKET=your_storage_bucket
VITE_FIREBASE_MESSAGING_SENDER_ID=your_messaging_sender_id
VITE_FIREBASE_APP_ID=your_app_id

### 2. Struktur Database Firestore

Buat collections berikut di Firestore:

1. Collection: `settings`
   - Document ID: `school`
     ```json
     {
       "nama": "Nama Sekolah",
       "alamat": "Alamat Sekolah",
       "telepon": "Nomor Telepon",
       "email": "email@sekolah.com"
     }
     ```
   - Document ID: `firebase`
     ```json
     {
       "apiKey": "firebase_api_key",
       "authDomain": "firebase_auth_domain",
       "projectId": "firebase_project_id",
       "storageBucket": "firebase_storage_bucket",
       "messagingSenderId": "firebase_messaging_sender_id",
       "appId": "firebase_app_id"
     }
     ```

2. Collection: `kelas`
   ```json
   {
     "namaKelas": "Nama Kelas",
     "waliKelas": "Nama Wali Kelas",
     "tingkat": "Tingkat/Level",
     "tahunAjaran": "2023/2024"
   }
   ```

3. Collection: `siswa`
   ```json
   {
     "nama": "Nama Siswa",
     "nis": "Nomor Induk Siswa",
     "kelasId": "ID Kelas",
     "namaKelas": "Nama Kelas",
     "alamat": "Alamat Siswa",
     "telepon": "Nomor Telepon"
   }
   ```

4. Collection: `penagihan`
   ```json
   {
     "siswaId": "ID Siswa",
     "nama_siswa": "Nama Siswa",
     "kelasId": "ID Kelas",
     "nama_kelas": "Nama Kelas",
     "tanggal_tagihan": "Timestamp",
     "jumlah_tagihan": 0,
     "status": "belum_lunas",
     "keterangan": "Keterangan Tagihan"
   }
   ```

5. Collection: `pembayaran`
   ```json
   {
     "siswaId": "ID Siswa",
     "nama_siswa": "Nama Siswa",
     "kelasId": "ID Kelas",
     "nama_kelas": "Nama Kelas",
     "tanggal_pembayaran": "Timestamp",
     "jumlah_pembayaran": 0,
     "keterangan": "Keterangan Pembayaran"
   }
   ```

## Menjalankan Aplikasi

1. Install dependencies:

```bash
npm install
```

2. Jalankan aplikasi:
```bash
npm run dev
```

## Cara Penggunaan Aplikasi

### 1. Login
- Gunakan email dan password yang telah didaftarkan di Firebase Authentication

### 2. Pengaturan Awal
- Buka menu "Pengaturan"
- Isi informasi sekolah (nama, alamat, dll)
- Upload logo sekolah jika diperlukan

### 3. Manajemen Kelas
- Buka menu "Kelas"
- Tambah/edit/hapus data kelas
- Isi informasi wali kelas dan tahun ajaran

### 4. Manajemen Siswa
- Buka menu "Siswa"
- Tambah/edit/hapus data siswa
- Pilih kelas untuk setiap siswa

### 5. Penagihan Pembayaran
- Buka menu "Penagihan"
- Buat penagihan individual atau massal
- Generate dan download PDF penagihan

### 6. Pembayaran
- Buka menu "Pembayaran"
- Catat pembayaran siswa
- Lihat history pembayaran

## Fitur Utama

1. Manajemen Data Kelas
2. Manajemen Data Siswa
3. Penagihan Pembayaran
4. Cetak PDF Penagihan
5. Pencatatan Pembayaran
6. Pengaturan Informasi Sekolah