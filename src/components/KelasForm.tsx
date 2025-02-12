import React from 'react';
import { Form, Input, Button, Space, Select } from 'antd';
import { Kelas, KelasFormData } from '../types/kelas';

interface KelasFormProps {
  initialValues?: Kelas;
  onSubmit: (values: KelasFormData) => void;
  onCancel: () => void;
  loading: boolean;
}

const KelasForm: React.FC<KelasFormProps> = ({
  initialValues,
  onSubmit,
  onCancel,
  loading,
}) => {
  const [form] = Form.useForm();

  React.useEffect(() => {
    if (initialValues) {
      form.setFieldsValue(initialValues);
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
        name="namaKelas"
        label="Nama Kelas"
        rules={[{ required: true, message: 'Mohon masukkan nama kelas' }]}
      >
        <Input />
      </Form.Item>

      <Form.Item
        name="waliKelas"
        label="Wali Kelas"
        rules={[{ required: true, message: 'Mohon masukkan nama wali kelas' }]}
      >
        <Input />
      </Form.Item>

      <Form.Item
        name="tingkat"
        label="Tingkat"
        rules={[{ required: true, message: 'Masukkan tingkat kelas!' }]}
      >
        <Select placeholder="Pilih tingkat">
          <Select.Option value="1">Tingkat 1</Select.Option>
          <Select.Option value="2">Tingkat 2</Select.Option>
          <Select.Option value="3">Tingkat 3</Select.Option>
        </Select>
      </Form.Item>

      <Form.Item
        name="tahunAjaran"
        label="Tahun Ajaran"
        rules={[{ required: true, message: 'Mohon masukkan tahun ajaran' }]}
      >
        <Input />
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

export default KelasForm;
