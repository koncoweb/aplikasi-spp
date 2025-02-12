import React, { useState, useEffect } from 'react';
import { Modal, Descriptions, Tag, Button, Space, Input, DatePicker, InputNumber, Select } from 'antd';
import { EditOutlined, SaveOutlined } from '@ant-design/icons';
import { PDFDownloadLink } from '@react-pdf/renderer';
import { Penagihan } from '../types/penagihan';
import PenagihanPDF from './PenagihanPDF';
import dayjs from 'dayjs';

interface PenagihanDetailsProps {
    data: Penagihan;
    visible: boolean;
    onClose: () => void;
    onEdit: (record: Penagihan) => void;
}

const formatCurrency = (value: number): string => {
    return `Rp ${value.toLocaleString('id-ID')}`;
};

const PenagihanDetails: React.FC<PenagihanDetailsProps> = ({
    data,
    visible,
    onClose,
    onEdit
}) => {
    const [isEditing, setIsEditing] = useState(false);
    const [editedData, setEditedData] = useState<Penagihan>(data);

    const getStatusTag = (status: string) => {
        let color = 'red';
        let text = 'Belum Bayar';
        
        if (status === 'sudah_bayar') {
            color = 'green';
            text = 'Sudah Bayar';
        } else if (status === 'telat') {
            color = 'orange';
            text = 'Telat';
        }

        return <Tag color={color}>{text}</Tag>;
    };

    const handleSave = () => {
        const updatedData = {
            ...editedData,
            id: data.id
        };
        onEdit(updatedData);
        setIsEditing(false);
    };

    useEffect(() => {
        setEditedData(data);
    }, [data]);

    const renderValue = (label: string, value: any, editConfig?: any) => {
        if (!isEditing) {
            return value;
        }

        switch (editConfig?.type) {
            case 'input':
                return (
                    <Input
                        value={editedData[editConfig.field as keyof Penagihan] as string}
                        onChange={(e) => setEditedData({
                            ...editedData,
                            [editConfig.field]: e.target.value
                        })}
                    />
                );
            case 'number':
                return (
                    <InputNumber
                        value={editedData[editConfig.field as keyof Penagihan] as number}
                        onChange={(value) => setEditedData({
                            ...editedData,
                            [editConfig.field]: value
                        })}
                        formatter={value => `Rp ${value}`.replace(/\B(?=(\d{3})+(?!\d))/g, ',')}
                        parser={value => value!.replace(/\Rp\s?|(,*)/g, '')}
                        style={{ width: '100%' }}
                    />
                );
            case 'date':
                return (
                    <DatePicker
                        value={dayjs(editedData[editConfig.field as keyof Penagihan] as Date)}
                        onChange={(date) => setEditedData({
                            ...editedData,
                            [editConfig.field]: date?.toDate()
                        })}
                        style={{ width: '100%' }}
                    />
                );
            case 'select':
                return (
                    <Select
                        value={editedData.status}
                        onChange={(value) => setEditedData({
                            ...editedData,
                            status: value
                        })}
                        style={{ width: '100%' }}
                    >
                        <Select.Option value="belum_bayar">Belum Bayar</Select.Option>
                        <Select.Option value="sudah_bayar">Sudah Bayar</Select.Option>
                        <Select.Option value="telat">Telat</Select.Option>
                    </Select>
                );
            default:
                return value;
        }
    };

    return (
        <Modal
            title="Detail Penagihan"
            open={visible}
            onCancel={onClose}
            width={800}
            footer={[
                <Space key="footer">
                    {isEditing ? (
                        <>
                            <Button 
                                type="primary"
                                icon={<SaveOutlined />}
                                onClick={handleSave}
                            >
                                Simpan
                            </Button>
                            <Button onClick={() => {
                                setIsEditing(false);
                                setEditedData(data);
                            }}>
                                Batal
                            </Button>
                        </>
                    ) : (
                        <>
                            <Button 
                                type="primary" 
                                icon={<EditOutlined />}
                                onClick={() => setIsEditing(true)}
                            >
                                Edit
                            </Button>
                            <PDFDownloadLink
                                document={<PenagihanPDF data={data} />}
                                fileName={`${data.nama_penagihan || 'penagihan'}-${data.nama_siswa || ''}.pdf`}
                            >
                                {({ loading }) => (
                                    <Button loading={loading}>
                                        Download PDF
                                    </Button>
                                )}
                            </PDFDownloadLink>
                        </>
                    )}
                    <Button onClick={onClose}>
                        Tutup
                    </Button>
                </Space>
            ]}
        >
            <Descriptions bordered column={2}>
                <Descriptions.Item label="Nama Penagihan" span={2}>
                    {renderValue("Nama Penagihan", data.nama_penagihan, {
                        type: 'input',
                        field: 'nama_penagihan'
                    })}
                </Descriptions.Item>
                <Descriptions.Item label="Nama Siswa">
                    {data.nama_siswa}
                </Descriptions.Item>
                <Descriptions.Item label="Kelas">
                    {data.kelas_siswa}
                </Descriptions.Item>
                <Descriptions.Item label="Jenis Pembayaran">
                    {data.nama_pembayaran}
                </Descriptions.Item>
                <Descriptions.Item label="Status">
                    {renderValue("Status", getStatusTag(data.status), {
                        type: 'select',
                        field: 'status'
                    })}
                </Descriptions.Item>
                <Descriptions.Item label="Tanggal Tagihan">
                    {renderValue("Tanggal Tagihan", 
                        data.tanggal_tagihan ? new Date(data.tanggal_tagihan).toLocaleDateString('id-ID') : '-',
                        {
                            type: 'date',
                            field: 'tanggal_tagihan'
                        }
                    )}
                </Descriptions.Item>
                <Descriptions.Item label="Jumlah Tagihan">
                    {renderValue("Jumlah Tagihan", 
                        formatCurrency(data.jumlah_tagihan),
                        {
                            type: 'number',
                            field: 'jumlah_tagihan'
                        }
                    )}
                </Descriptions.Item>
                <Descriptions.Item label="Keterangan" span={2}>
                    {renderValue("Keterangan", data.keterangan || '-', {
                        type: 'input',
                        field: 'keterangan'
                    })}
                </Descriptions.Item>
                <Descriptions.Item label="Dibuat Pada">
                    {data.createdAt ? new Date(data.createdAt).toLocaleString('id-ID') : '-'}
                </Descriptions.Item>
                <Descriptions.Item label="Diperbarui Pada">
                    {data.updatedAt ? new Date(data.updatedAt).toLocaleString('id-ID') : '-'}
                </Descriptions.Item>
            </Descriptions>
        </Modal>
    );
};

export default PenagihanDetails; 