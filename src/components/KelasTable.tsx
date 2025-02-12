import React from 'react';
import { Table, Space, Button, Popconfirm } from 'antd';
import { EditOutlined, DeleteOutlined } from '@ant-design/icons';
import { Kelas } from '../types/kelas';

interface KelasTableProps {
  dataSource: Kelas[];
  loading: boolean;
  onEdit: (record: Kelas) => void;
  onDelete: (id: string) => void;
}

const KelasTable: React.FC<KelasTableProps> = ({
  dataSource,
  loading,
  onEdit,
  onDelete,
}) => {
  const columns = [
    {
      title: 'Nama Kelas',
      dataIndex: 'namaKelas',
      key: 'namaKelas',
    },
    {
      title: 'Tingkat',
      dataIndex: 'tingkat',
      key: 'tingkat',
      render: (tingkat: string) => `Tingkat ${tingkat}`,
    },
    {
      title: 'Wali Kelas',
      dataIndex: 'waliKelas',
      key: 'waliKelas',
    },
    {
      title: 'Tahun Ajaran',
      dataIndex: 'tahunAjaran',
      key: 'tahunAjaran',
    },
    {
      title: 'Aksi',
      key: 'action',
      render: (_: any, record: Kelas) => (
        <Space size="middle">
          <Button
            type="primary"
            icon={<EditOutlined />}
            onClick={() => onEdit(record)}
          >
            Edit
          </Button>
          <Popconfirm
            title="Apakah Anda yakin ingin menghapus kelas ini?"
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
      dataSource={dataSource}
      columns={columns}
      loading={loading}
      rowKey="id"
    />
  );
};

export default KelasTable;
