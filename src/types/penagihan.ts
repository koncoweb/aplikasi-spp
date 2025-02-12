import { Pembayaran } from './pembayaran';
import { Siswa } from './student';

export interface Penagihan {
    id?: string;
    pembayaran_id: string;
    siswa_nis: string;
    status: 'belum_bayar' | 'sudah_bayar' | 'telat';
    tanggal_tagihan: Date;
    tanggal_bayar?: Date;
    jumlah_tagihan: number;
    keterangan?: string;
    // Denormalized fields untuk mempermudah tampilan
    nama_pembayaran?: string;
    nama_siswa?: string;
    kelas_siswa?: string;
    createdAt?: Date;
    updatedAt?: Date;
    createdBy?: string;
    updatedBy?: string;
    nama_penagihan: string;
}

export interface PenagihanFormData {
    pembayaran_id: string;
    siswa_nis: string;
    tanggal_tagihan: Date;
    jumlah_tagihan: number;
    keterangan?: string;
}