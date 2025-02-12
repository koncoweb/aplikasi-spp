import React, { useState, useEffect } from 'react';
import { Button, Modal, message, Space } from 'antd';
import { PlusOutlined, TeamOutlined } from '@ant-design/icons';
import { collection, getDocs, addDoc, updateDoc, deleteDoc, doc, writeBatch, Timestamp } from 'firebase/firestore';
import { PDFDownloadLink } from '@react-pdf/renderer';
import { db } from '../firebase';
import { Penagihan } from '../types/penagihan';
import { KelasType } from '../types/kelas';
import PenagihanTable from './PenagihanTable';
import PenagihanForm from './PenagihanForm';
import PenagihanPDF from './PenagihanPDF';
import BulkPenagihanForm from './BulkPenagihanForm';

const PenagihanPage: React.FC = () => {
    const [penagihan, setPenagihan] = useState<Penagihan[]>([]);
    const [loading, setLoading] = useState(false);
    const [modalVisible, setModalVisible] = useState(false);
    const [bulkModalVisible, setBulkModalVisible] = useState(false);
    const [selectedPenagihan, setSelectedPenagihan] = useState<Penagihan | undefined>();
    const [kelasList, setKelasList] = useState<KelasType[]>([]);

    const fetchPenagihan = async () => {
        setLoading(true);
        try {
            const querySnapshot = await getDocs(collection(db, 'penagihan'));
            const penagihanData = querySnapshot.docs.map(doc => ({
                id: doc.id,
                ...doc.data(),
                tanggal_tagihan: doc.data().tanggal_tagihan?.toDate(),
                tanggal_bayar: doc.data().tanggal_bayar?.toDate(),
                createdAt: doc.data().createdAt?.toDate(),
                updatedAt: doc.data().updatedAt?.toDate(),
            })) as Penagihan[];
            setPenagihan(penagihanData);
        } catch (error) {
            console.error('Error fetching penagihan:', error);
            message.error('Gagal mengambil data penagihan');
        }
        setLoading(false);
    };

    const fetchKelas = async () => {
        try {
            const querySnapshot = await getDocs(collection(db, 'kelas'));
            const kelasData = querySnapshot.docs.map(doc => ({
                id: doc.id,
                ...doc.data()
            })) as KelasType[];
            setKelasList(kelasData);
        } catch (error) {
            console.error('Error fetching kelas:', error);
            message.error('Gagal mengambil data kelas');
        }
    };

    useEffect(() => {
        fetchPenagihan();
        fetchKelas();
    }, []);

    const handleSubmit = async (values: Partial<Penagihan>) => {
        setLoading(true);
        try {
            const now = new Date();

            const penagihanData = {
                ...values,
                tanggal_tagihan: values.tanggal_tagihan || now,
                tanggal_bayar: values.tanggal_bayar || null,
                status: 'belum_bayar',
                createdAt: now,
                updatedAt: now,
                createdBy: 'current_user',
                updatedBy: 'current_user',
                pembayaran_id: values.pembayaran_id || '',
                siswa_nis: values.siswa_nis || '',
                jumlah_tagihan: Number(values.jumlah_tagihan) || 0,
                nama_penagihan: values.nama_penagihan || '',
                nama_pembayaran: values.nama_pembayaran || '',
                nama_siswa: values.nama_siswa || '',
                kelas_siswa: values.kelas_siswa || '',
                keterangan: values.keterangan || null
            };

            if (selectedPenagihan?.id) {
                await updateDoc(doc(db, 'penagihan', selectedPenagihan.id), penagihanData);
                message.success('Penagihan berhasil diperbarui');
            } else {
                const docRef = await addDoc(collection(db, 'penagihan'), penagihanData);
                message.success('Penagihan berhasil ditambahkan');
                handlePrintPDF({ ...penagihanData, id: docRef.id } as Penagihan);
            }
            setModalVisible(false);
            fetchPenagihan();
        } catch (error) {
            console.error('Error saving penagihan:', error);
            if (error instanceof Error) {
                message.error(`Gagal menyimpan data penagihan: ${error.message}`);
            } else {
                message.error('Gagal menyimpan data penagihan');
            }
        }
        setLoading(false);
    };

    const handlePrintPDF = (data: Penagihan) => {
        return (
            <PDFDownloadLink
                document={<PenagihanPDF data={data} />}
                fileName={`${data.nama_penagihan || 'penagihan'}-${data.nama_siswa || ''}.pdf`}
                className="ant-btn ant-btn-default"
            >
                Download PDF
            </PDFDownloadLink>
        );
    };

    const handleEdit = (record: Penagihan) => {
        setSelectedPenagihan(record);
        setModalVisible(true);
    };

    const handleDelete = async (id: string) => {
        try {
            await deleteDoc(doc(db, 'penagihan', id));
            message.success('Penagihan berhasil dihapus');
            fetchPenagihan();
        } catch (error) {
            console.error('Error deleting penagihan:', error);
            message.error('Gagal menghapus penagihan');
        }
    };

    const handleCancel = () => {
        setModalVisible(false);
        setSelectedPenagihan(undefined);
    };

    const handleBulkCreate = async (values: { 
        tingkat?: string; 
        kelas_id?: string; 
        nama_pembayaran: string;
        jumlah_tagihan: number;
        tanggal_tagihan: any; // Moment or Date object from DatePicker
        keterangan?: string;
    }) => {
        setLoading(true);
        try {
            const now = new Date();
            // Convert the DatePicker value to a Date object
            const tagihan_date = values.tanggal_tagihan ? new Date(values.tanggal_tagihan) : now;

            const siswaSnapshot = await getDocs(collection(db, 'siswa'));
            interface SiswaWithData {
                nis: string;
                nama_lengkap: string;
                kelas: string;
                [key: string]: any;
            }
            const siswaList = siswaSnapshot.docs.map(doc => ({
                ...doc.data(),
                nis: doc.id
            })) as SiswaWithData[];

            // Filter siswa based on tingkat or kelas
            const filteredSiswa = siswaList.filter(siswa => {
                if (values.kelas_id) {
                    return siswa.kelas === values.kelas_id;
                }
                if (values.tingkat) {
                    const kelas = kelasList.find(k => k.namaKelas === siswa.kelas);
                    return kelas?.tingkat === values.tingkat;
                }
                return false;
            });

            if (filteredSiswa.length === 0) {
                throw new Error('Tidak ada siswa yang ditemukan untuk kriteria yang dipilih');
            }

            // Create penagihan for each siswa
            const batch = writeBatch(db);
            filteredSiswa.forEach(siswa => {
                const penagihanRef = doc(collection(db, 'penagihan'));
                const nama_penagihan = `${values.nama_pembayaran} ${siswa.nama_lengkap} ${siswa.kelas}`.trim();
                
                batch.set(penagihanRef, {
                    pembayaran_id: '',
                    siswa_nis: siswa.nis,
                    nama_pembayaran: values.nama_pembayaran,
                    nama_siswa: siswa.nama_lengkap,
                    kelas_siswa: siswa.kelas,
                    tanggal_tagihan: Timestamp.fromDate(tagihan_date),
                    jumlah_tagihan: values.jumlah_tagihan,
                    status: 'belum_bayar',
                    createdAt: Timestamp.fromDate(now),
                    updatedAt: Timestamp.fromDate(now),
                    createdBy: 'current_user',
                    updatedBy: 'current_user',
                    nama_penagihan,
                    keterangan: values.keterangan || null
                });
            });

            await batch.commit();
            message.success(`Berhasil membuat ${filteredSiswa.length} penagihan`);
            setBulkModalVisible(false);
            fetchPenagihan();
        } catch (error) {
            console.error('Error creating bulk penagihan:', error);
            message.error(error instanceof Error ? error.message : 'Gagal membuat penagihan massal');
        }
        setLoading(false);
    };

    return (
        <div style={{ padding: '24px' }}>
            <div style={{ marginBottom: '16px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <h2>Data Penagihan</h2>
                <Space>
                    <Button
                        type="primary"
                        icon={<TeamOutlined />}
                        onClick={() => setBulkModalVisible(true)}
                    >
                        Penagihan Massal
                    </Button>
                    <Button
                        type="primary"
                        icon={<PlusOutlined />}
                        onClick={() => {
                            setSelectedPenagihan(undefined);
                            setModalVisible(true);
                        }}
                    >
                        Tambah Penagihan
                    </Button>
                </Space>
            </div>

            <PenagihanTable
                data={penagihan}
                loading={loading}
                onEdit={handleEdit}
                onDelete={handleDelete}
                onPrint={(record) => (
                    <Button>
                        <PDFDownloadLink
                            document={<PenagihanPDF data={record} />}
                            fileName={`${record.nama_penagihan || 'penagihan'}-${record.nama_siswa || ''}.pdf`}
                        >
                            Download PDF
                        </PDFDownloadLink>
                    </Button>
                )}
            />

            <Modal
                title={selectedPenagihan ? 'Edit Penagihan' : 'Tambah Penagihan'}
                open={modalVisible}
                onCancel={handleCancel}
                footer={null}
                width={800}
            >
                <PenagihanForm
                    initialValues={selectedPenagihan}
                    onSubmit={handleSubmit}
                    onCancel={handleCancel}
                    loading={loading}
                />
            </Modal>

            <Modal
                title="Buat Penagihan Massal"
                open={bulkModalVisible}
                onCancel={() => setBulkModalVisible(false)}
                footer={null}
                width={600}
            >
                <BulkPenagihanForm
                    onSubmit={handleBulkCreate}
                    onCancel={() => setBulkModalVisible(false)}
                    loading={loading}
                    kelasList={kelasList}
                />
            </Modal>
        </div>
    );
};

export default PenagihanPage;
