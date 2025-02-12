import React, { useState, useEffect } from 'react';
import { Button, Modal, message } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import { collection, getDocs, addDoc, updateDoc, deleteDoc, doc } from 'firebase/firestore';
import { db } from '../firebase';
import { KelasType, KelasFormData } from '../types/kelas';
import KelasTable from './KelasTable';
import KelasForm from './KelasForm';

const KelasPage: React.FC = () => {
    const [kelas, setKelas] = useState<KelasType[]>([]);
    const [loading, setLoading] = useState(false);
    const [modalVisible, setModalVisible] = useState(false);
    const [selectedKelas, setSelectedKelas] = useState<KelasType | undefined>();

    const fetchKelas = async () => {
        setLoading(true);
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
        setLoading(false);
    };

    useEffect(() => {
        fetchKelas();
    }, []);

    const handleSubmit = async (values: KelasFormData) => {
        setLoading(true);
        try {
            if (selectedKelas?.id) {
                await updateDoc(doc(db, 'kelas', selectedKelas.id), {
                    ...values,
                    tingkat: values.tingkat,
                    updatedAt: new Date()
                });
                message.success('Kelas berhasil diperbarui');
            } else {
                await addDoc(collection(db, 'kelas'), {
                    ...values,
                    tingkat: values.tingkat,
                    createdAt: new Date(),
                    updatedAt: new Date()
                });
                message.success('Kelas berhasil ditambahkan');
            }
            setModalVisible(false);
            fetchKelas();
        } catch (error) {
            console.error('Error saving kelas:', error);
            message.error('Gagal menyimpan kelas');
        }
        setLoading(false);
    };

    const handleDelete = async (id: string) => {
        try {
            await deleteDoc(doc(db, 'kelas', id));
            message.success('Kelas berhasil dihapus');
            fetchKelas();
        } catch (error) {
            console.error('Error deleting kelas:', error);
            message.error('Gagal menghapus kelas');
        }
    };

    const handleEdit = (record: KelasType) => {
        setSelectedKelas(record);
        setModalVisible(true);
    };

    const handleCancel = () => {
        setModalVisible(false);
        setSelectedKelas(undefined);
    };

    return (
        <div className="kelas-container">
            <h2>Kelas</h2>
            <div className="kelas-content">
                <div style={{ marginBottom: '16px' }}>
                    <Button
                        type="primary"
                        icon={<PlusOutlined />}
                        onClick={() => {
                            setSelectedKelas(undefined);
                            setModalVisible(true);
                        }}
                    >
                        Tambah Kelas
                    </Button>
                </div>

                <KelasTable
                    dataSource={kelas}
                    loading={loading}
                    onEdit={handleEdit}
                    onDelete={handleDelete}
                />

                <Modal
                    title={selectedKelas ? 'Edit Kelas' : 'Tambah Kelas'}
                    open={modalVisible}
                    onCancel={handleCancel}
                    footer={null}
                >
                    <KelasForm
                        initialValues={selectedKelas}
                        onSubmit={handleSubmit}
                        onCancel={handleCancel}
                        loading={loading}
                    />
                </Modal>
            </div>
        </div>
    );
};

export default KelasPage;
