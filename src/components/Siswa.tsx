import React, { useState, useEffect } from 'react';
import { Button, Modal, message } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import { collection, getDocs, addDoc, updateDoc, deleteDoc, doc } from 'firebase/firestore';
import { db } from '../firebase';
import { Siswa } from '../types/student';
import { KelasType } from '../types/kelas';
import SiswaTable from './SiswaTable';
import SiswaForm from './SiswaForm';

const SiswaPage: React.FC = () => {
    const [siswa, setSiswa] = useState<Siswa[]>([]);
    const [kelas, setKelas] = useState<KelasType[]>([]);
    const [loading, setLoading] = useState(false);
    const [modalVisible, setModalVisible] = useState(false);
    const [selectedSiswa, setSelectedSiswa] = useState<Siswa | undefined>();

    const fetchKelas = async () => {
        try {
            const querySnapshot = await getDocs(collection(db, 'kelas'));
            const kelasData = querySnapshot.docs.map(doc => ({
                id: doc.id,
                ...doc.data()
            })) as KelasType[];
            setKelas(kelasData);
        } catch (error) {
            console.error('Error fetching kelas:', error);
            message.error('Gagal mengambil data kelas');
        }
    };

    const fetchSiswa = async () => {
        setLoading(true);
        try {
            const querySnapshot = await getDocs(collection(db, 'siswa'));
            const siswaData = querySnapshot.docs.map(doc => ({
                ...doc.data(),
                nis: doc.id
            })) as Siswa[];
            setSiswa(siswaData);
        } catch (error) {
            console.error('Error fetching siswa:', error);
            message.error('Gagal mengambil data siswa');
        }
        setLoading(false);
    };

    useEffect(() => {
        fetchSiswa();
        fetchKelas(); // Mengambil data kelas saat komponen dimount
    }, []);

    const handleSubmit = async (values: Partial<Siswa>) => {
        setLoading(true);
        try {
            const now = new Date();
            if (selectedSiswa?.nis) {
                await updateDoc(doc(db, 'siswa', selectedSiswa.nis), {
                    ...values,
                    updated_at: now,
                    updated_by: 'current_user' // Replace with actual user ID from auth context
                });
                message.success('Data siswa berhasil diperbarui');
            } else {
                const newSiswa = {
                    ...values,
                    created_at: now,
                    updated_at: now,
                    created_by: 'current_user', // Replace with actual user ID from auth context
                    updated_by: 'current_user'  // Replace with actual user ID from auth context
                };
                await addDoc(collection(db, 'siswa'), newSiswa);
                message.success('Siswa berhasil ditambahkan');
            }
            setModalVisible(false);
            fetchSiswa();
        } catch (error) {
            console.error('Error saving siswa:', error);
            message.error('Gagal menyimpan data siswa');
        }
        setLoading(false);
    };

    const handleEdit = (record: Siswa) => {
        setSelectedSiswa(record);
        setModalVisible(true);
    };

    const handleDelete = async (nis: string) => {
        try {
            await deleteDoc(doc(db, 'siswa', nis));
            message.success('Siswa berhasil dihapus');
            fetchSiswa();
        } catch (error) {
            console.error('Error deleting siswa:', error);
            message.error('Gagal menghapus siswa');
        }
    };

    const handleCancel = () => {
        setModalVisible(false);
        setSelectedSiswa(undefined);
    };

    return (
        <div style={{ padding: '24px' }}>
            <div style={{ marginBottom: '16px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <h2>Data Siswa</h2>
                <Button
                    type="primary"
                    icon={<PlusOutlined />}
                    onClick={() => {
                        setSelectedSiswa(undefined);
                        setModalVisible(true);
                    }}
                >
                    Tambah Siswa
                </Button>
            </div>

            <SiswaTable
                data={siswa}
                loading={loading}
                onEdit={handleEdit}
                onDelete={handleDelete}
                onRefresh={fetchSiswa}
            />

            <Modal
                title={selectedSiswa ? 'Edit Data Siswa' : 'Tambah Siswa Baru'}
                open={modalVisible}
                onCancel={handleCancel}
                footer={null}
                width={800}
            >
                <SiswaForm
                    initialValues={selectedSiswa}
                    onSubmit={handleSubmit}
                    onCancel={handleCancel}
                    loading={loading}
                    kelasList={kelas} // Meneruskan data kelas ke form
                />
            </Modal>
        </div>
    );
};

export default SiswaPage;
