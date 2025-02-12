import React from 'react';
import { Form, Input, Select, DatePicker, Button, Space } from 'antd';
import { Siswa } from '../types/student';
import { Kelas } from '../types/kelas';

interface SiswaFormProps {
  initialValues?: Siswa;
  onSubmit: (values: Partial<Siswa>) => void;
  onCancel: () => void;
  loading: boolean;
  kelasList: Kelas[]; // Menambahkan prop untuk daftar kelas
}

const SiswaForm: React.FC<SiswaFormProps> = ({
  initialValues,
  onSubmit,
  onCancel,
  loading,
  kelasList,
}) => {
  const [form] = Form.useForm();

  React.useEffect(() => {
    if (initialValues) {
      form.setFieldsValue({
        ...initialValues,
        tanggal_lahir: initialValues.tanggal_lahir && new Date(initialValues.tanggal_lahir),
      });
    } else {
      form.resetFields();
    }
  }, [initialValues, form]);

  return (
    <Form
      form={form}
      layout="vertical"
      onFinish={onSubmit}
      initialValues={initialValues}
    >
      <Form.Item
        name="nis"
        label="NIS"
        rules={[{ required: true, message: 'Mohon masukkan NIS' }]}
      >
        <Input disabled={!!initialValues} />
      </Form.Item>

      <Form.Item
        name="nisn"
        label="NISN"
        rules={[{ required: true, message: 'Mohon masukkan NISN' }]}
      >
        <Input />
      </Form.Item>

      <Form.Item
        name="nama_lengkap"
        label="Nama Lengkap"
        rules={[{ required: true, message: 'Mohon masukkan nama lengkap' }]}
      >
        <Input />
      </Form.Item>

      <Form.Item
        name="jenis_kelamin"
        label="Jenis Kelamin"
        rules={[{ required: true, message: 'Mohon pilih jenis kelamin' }]}
      >
        <Select>
          <Select.Option value="L">Laki-laki</Select.Option>
          <Select.Option value="P">Perempuan</Select.Option>
        </Select>
      </Form.Item>

      <Form.Item
        name="kelas"
        label="Kelas"
        rules={[{ required: true, message: 'Mohon pilih kelas' }]}
      >
        <Select
          placeholder="Pilih Kelas"
          showSearch
          optionFilterProp="children"
        >
          {kelasList.map((kelas) => (
            <Select.Option key={kelas.id} value={kelas.namaKelas}>
              {`${kelas.namaKelas} - ${kelas.tahunAjaran} (${kelas.waliKelas})`}
            </Select.Option>
          ))}
        </Select>
      </Form.Item>

      <Form.Item
        name="status"
        label="Status"
        rules={[{ required: true, message: 'Mohon pilih status' }]}
      >
        <Select>
          <Select.Option value="aktif">Aktif</Select.Option>
          <Select.Option value="alumni">Alumni</Select.Option>
          <Select.Option value="keluar">Keluar</Select.Option>
        </Select>
      </Form.Item>

      <Form.Item>
        <Space>
          <Button type="primary" htmlType="submit" loading={loading}>
            {initialValues ? 'Update' : 'Simpan'}
          </Button>
          <Button onClick={onCancel}>Batal</Button>
        </Space>
      </Form.Item>
    </Form>
  );
};

export default SiswaForm;
