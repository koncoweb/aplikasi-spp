import React, { useState, useEffect } from 'react';
import { Button, Modal, message } from 'antd';
import { PlusOutlined } from '@ant-design/icons';
import { collection, getDocs, addDoc, updateDoc, deleteDoc, doc } from 'firebase/firestore';
import { db } from '../firebase';
import { Pembayaran } from '../types/pembayaran';
import PembayaranTable from './PembayaranTable';
import PembayaranForm from './PembayaranForm';

const PembayaranPage: React.FC = () => {
    const [pembayaran, setPembayaran] = useState<Pembayaran[]>([]);
    const [loading, setLoading] = useState(false);
    const [modalVisible, setModalVisible] = useState(false);
    const [selectedPembayaran, setSelectedPembayaran] = useState<Pembayaran | undefined>();

    const fetchPembayaran = async () => {
        setLoading(true);
        try {
            const querySnapshot = await getDocs(collection(db, 'pembayaran'));
            const pembayaranData = querySnapshot.docs.map(doc => ({
                id: doc.id,
                ...doc.data()
            })) as Pembayaran[];
            setPembayaran(pembayaranData);
        } catch (error) {
            console.error('Error fetching pembayaran:', error);
            message.error('Gagal mengambil data pembayaran');
        }
        setLoading(false);
    };

    useEffect(() => {
        fetchPembayaran();
    }, []);

    const handleSubmit = async (values: Partial<Pembayaran>) => {
        setLoading(true);
        try {
            const now = new Date();
            const dataToSave = {
                ...values,
                nominal: Number(values.nominal), // Konversi ke number
                updatedAt: now,
                updatedBy: 'current_user' // Replace with actual user ID
            };

            if (selectedPembayaran?.id) {
                await updateDoc(doc(db, 'pembayaran', selectedPembayaran.id), dataToSave);
                message.success('Data pembayaran berhasil diperbarui');
            } else {
                await addDoc(collection(db, 'pembayaran'), {
                    ...dataToSave,
                    createdAt: now,
                    createdBy: 'current_user', // Replace with actual user ID
                });
                message.success('Pembayaran berhasil ditambahkan');
            }
            setModalVisible(false);
            fetchPembayaran();
        } catch (error) {
            console.error('Error saving pembayaran:', error);
            message.error('Gagal menyimpan data pembayaran');
        }
        setLoading(false);
    };

    const handleEdit = (record: Pembayaran) => {
        setSelectedPembayaran(record);
        setModalVisible(true);
    };

    const handleDelete = async (id: string) => {
        try {
            await deleteDoc(doc(db, 'pembayaran', id));
            message.success('Pembayaran berhasil dihapus');
            fetchPembayaran();
        } catch (error) {
            console.error('Error deleting pembayaran:', error);
            message.error('Gagal menghapus pembayaran');
        }
    };

    const handleCancel = () => {
        setModalVisible(false);
        setSelectedPembayaran(undefined);
    };

    return (
        <div style={{ padding: '24px' }}>
            <div style={{ marginBottom: '16px', display: 'flex', justifyContent: 'space-between', alignItems: 'center' }}>
                <h2>Data Pembayaran</h2>
                <Button
                    type="primary"
                    icon={<PlusOutlined />}
                    onClick={() => {
                        setSelectedPembayaran(undefined);
                        setModalVisible(true);
                    }}
                >
                    Tambah Pembayaran
                </Button>
            </div>

            <PembayaranTable
                data={pembayaran}
                loading={loading}
                onEdit={handleEdit}
                onDelete={handleDelete}
            />

            <Modal
                title={selectedPembayaran ? 'Edit Pembayaran' : 'Tambah Pembayaran'}
                open={modalVisible}
                onCancel={handleCancel}
                footer={null}
                width={600}
            >
                <PembayaranForm
                    initialValues={selectedPembayaran}
                    onSubmit={handleSubmit}
                    onCancel={handleCancel}
                    loading={loading}
                />
            </Modal>
        </div>
    );
};

export default PembayaranPage;
