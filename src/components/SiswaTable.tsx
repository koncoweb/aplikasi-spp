import React from 'react';
import { Button, message, Popconfirm, Space, Table } from 'antd';
import { DeleteOutlined, EditOutlined } from '@ant-design/icons';
import { collection, deleteDoc, doc } from 'firebase/firestore';
import { db } from '../firebase';
import { Siswa } from '../types/student';

interface SiswaTableProps {
  data: Siswa[];
  loading: boolean;
  onEdit: (siswa: Siswa) => void;
  onDelete: (nis: string) => Promise<void>;
  onRefresh: () => Promise<void>;
}

const SiswaTable: React.FC<SiswaTableProps> = ({ data, loading, onEdit, onDelete, onRefresh }) => {
  const handleDelete = async (id: string) => {
    try {
      await deleteDoc(doc(db, 'siswa', id));
      message.success('Siswa berhasil dihapus');
      onRefresh();
    } catch (error) {
      message.error('Gagal menghapus siswa');
    }
  };

  const columns = [
    {
      title: 'NIS',
      dataIndex: 'nis',
      key: 'nis',
    },
    {
      title: 'NISN',
      dataIndex: 'nisn',
      key: 'nisn',
    },
    {
      title: 'Nama Lengkap',
      dataIndex: 'nama_lengkap',
      key: 'nama_lengkap',
    },
    {
      title: 'Kelas',
      dataIndex: 'kelas',
      key: 'kelas',
    },
    {
      title: 'Jurusan',
      dataIndex: 'jurusan',
      key: 'jurusan',
    },
    {
      title: 'Jenis Kelamin',
      dataIndex: 'jenis_kelamin',
      key: 'jenis_kelamin',
      render: (text: 'L' | 'P') => (text === 'L' ? 'Laki-laki' : 'Perempuan'),
    },
    {
      title: 'Status',
      dataIndex: 'status',
      key: 'status',
      render: (text: string | undefined) => {
        if (!text) return '-';
        return text.charAt(0).toUpperCase() + text.slice(1);
      },
    },
    {
      title: 'Aksi',
      key: 'action',
      render: (_: any, record: Siswa) => (
        <Space>
          <Button
            type="primary"
            icon={<EditOutlined />}
            onClick={() => onEdit(record)}
          >
            Edit
          </Button>
          <Popconfirm
            title="Apakah Anda yakin ingin menghapus data ini?"
            onConfirm={() => handleDelete(record.nis)}
            okText="Ya"
            cancelText="Tidak"
          >
            <Button type="primary" danger icon={<DeleteOutlined />}>
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
      rowKey="nis"
      scroll={{ x: true }}
    />
  );
};

export default SiswaTable;
