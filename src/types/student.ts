export interface Siswa {
    nis: string;
    nisn: string;
    nama_lengkap: string;
    jenis_kelamin: 'L' | 'P';
    kelas: string;
    jurusan?: string;
    status: 'aktif' | 'alumni' | 'keluar';
    tempat_lahir: string;
    tanggal_lahir: Date;
    agama: string;
    biaya_spp: number;
    created_at?: Date;
    updated_at?: Date;
    created_by?: string;
    updated_by?: string;
}

import { DocumentData, QueryDocumentSnapshot, FirestoreDataConverter, Timestamp, SnapshotOptions } from 'firebase/firestore';

const siswaConverter: FirestoreDataConverter<Siswa> = {
    toFirestore(siswa: Siswa) {
        const data: any = {
            ...siswa,
            tanggal_lahir: Timestamp.fromDate(siswa.tanggal_lahir),
        };

        if (siswa.created_at) {
            data.created_at = Timestamp.fromDate(siswa.created_at);
        }
        if (siswa.updated_at) {
            data.updated_at = Timestamp.fromDate(siswa.updated_at);
        }

        return data;
    },
    fromFirestore(
        snapshot: QueryDocumentSnapshot,
        options: SnapshotOptions
    ): Siswa {
        const data = snapshot.data(options);
        return {
            ...data,
            tanggal_lahir: data.tanggal_lahir.toDate(),
            created_at: data.created_at?.toDate(),
            updated_at: data.updated_at?.toDate()
        } as Siswa;
    }
};

export { siswaConverter };
