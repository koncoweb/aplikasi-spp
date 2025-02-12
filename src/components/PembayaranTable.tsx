import React from 'react';
import { Table, Space, Button, Popconfirm } from 'antd';
import { EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { Pembayaran } from '../types/pembayaran';

interface PembayaranTableProps {
    data: Pembayaran[];
    loading: boolean;
    onEdit: (record: Pembayaran) => void;
    onDelete: (id: string) => void;
}

const formatCurrency = (value: any): string => {
    if (value === null || value === undefined || isNaN(Number(value))) {
        return 'Rp 0';
    }
    return `Rp ${Number(value).toLocaleString('id-ID')}`;
};

const PembayaranTable: React.FC<PembayaranTableProps> = ({
    data,
    loading,
    onEdit,
    onDelete
}) => {
    const columns = [
        {
            title: 'Nama Pembayaran',
            dataIndex: 'nama',
            key: 'nama',
            render: (text: string) => text || '-',
        },
        {
            title: 'Jenis',
            dataIndex: 'jenis',
            key: 'jenis',
            render: (text: string) => text || '-',
        },
        {
            title: 'Nominal',
            dataIndex: 'nominal',
            key: 'nominal',
            render: (value: any) => formatCurrency(value),
        },
        {
            title: 'Aksi',
            key: 'action',
            render: (_: any, record: Pembayaran) => (
                <Space size="middle">
                    <Button
                        type="primary"
                        icon={<EditOutlined />}
                        onClick={() => onEdit(record)}
                    >
                        Edit
                    </Button>
                    <Popconfirm
                        title="Apakah Anda yakin ingin menghapus data ini?"
                        onConfirm={() => record.id && onDelete(record.id)}
                        okText="Ya"
                        cancelText="Tidak"
                    >
                        <Button
                            type="primary"
                            danger
                            icon={<DeleteOutlined />}
                            disabled={!record.id}
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
        />
    );
};

export default PembayaranTable;
