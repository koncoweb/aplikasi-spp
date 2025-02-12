export interface Pembayaran {
    id?: string;
    nama: string;
    jenis: string;
    nominal: number;
    createdAt?: Date;
    updatedAt?: Date;
    createdBy?: string;
    updatedBy?: string;
}

export interface PembayaranFormData {
    nama: string;
    jenis: Pembayaran['jenis'];
    nominal: number;
}
