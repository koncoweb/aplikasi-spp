import React from 'react';
import { Table, Button, Space, Tag, Popconfirm } from 'antd';
import { EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { Penagihan } from '../types/penagihan';

interface PenagihanTableProps {
    data: Penagihan[];
    loading: boolean;
    onEdit: (record: Penagihan) => void;
    onDelete: (id: string) => void;
    onPrint: (record: Penagihan) => React.ReactNode;
}

const formatCurrency = (value: number | null | undefined): string => {
    if (value === null || value === undefined || isNaN(Number(value))) {
        return 'Rp 0';
    }
    return `Rp ${value.toLocaleString('id-ID')}`;
};

const PenagihanTable: React.FC<PenagihanTableProps> = ({
    data,
    loading,
    onEdit,
    onDelete,
    onPrint
}) => {
    const handleEdit = (e: React.MouseEvent, record: Penagihan) => {
        e.preventDefault();
        e.stopPropagation();
        onEdit(record);
    };

    const handleDelete = (e: React.MouseEvent | undefined, id: string) => {
        if (e) {
            e.preventDefault();
            e.stopPropagation();
        }
        onDelete(id);
    };

    const columns = [
        {
            title: 'Nama Penagihan',
            dataIndex: 'nama_penagihan',
            key: 'nama_penagihan',
            render: (text: string) => text || '-',
        },
        {
            title: 'Pembayaran',
            dataIndex: 'nama_pembayaran',
            key: 'nama_pembayaran',
            render: (text: string) => text || '-',
        },
        {
            title: 'Siswa',
            dataIndex: 'nama_siswa',
            key: 'nama_siswa',
            render: (text: string, record: Penagihan) => (
                <span>{text || '-'} {record.kelas_siswa ? `- ${record.kelas_siswa}` : ''}</span>
            ),
        },
        {
            title: 'Tanggal Tagihan',
            dataIndex: 'tanggal_tagihan',
            key: 'tanggal_tagihan',
            render: (date: Date | null) => date ? new Date(date).toLocaleDateString('id-ID') : '-',
        },
        {
            title: 'Status',
            dataIndex: 'status',
            key: 'status',
            render: (status: string) => {
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
            },
        },
        {
            title: 'Jumlah Tagihan',
            dataIndex: 'jumlah_tagihan',
            key: 'jumlah_tagihan',
            render: (value: number) => formatCurrency(value),
        },
        {
            title: 'Aksi',
            key: 'action',
            render: (_: any, record: Penagihan) => (
                <Space size="middle" onClick={e => e.stopPropagation()}>
                    <Button
                        type="primary"
                        icon={<EditOutlined />}
                        onClick={(e) => handleEdit(e, record)}
                    >
                        Edit
                    </Button>
                    {onPrint(record)}
                    <Popconfirm
                        title="Apakah Anda yakin ingin menghapus data ini?"
                        onConfirm={(e) => handleDelete(e, record.id!)}
                        okText="Ya"
                        cancelText="Tidak"
                    >
                        <Button
                            type="primary"
                            danger
                            icon={<DeleteOutlined />}
                        >
                            Hapus
                        </Button>
                    </Popconfirm>
                </Space>
            ),
        },
    ];

    return (
        <Table
            dataSource={data}
            columns={columns}
            loading={loading}
            rowKey="id"
            onRow={(record) => ({
                onClick: (e) => handleEdit(e, record),
                style: { cursor: 'pointer' }
            })}
        />
    );
};

export default PenagihanTable;
