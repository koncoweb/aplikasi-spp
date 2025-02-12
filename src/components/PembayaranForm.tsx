import React from 'react';
import { Form, Input, Select, InputNumber, Button, Space } from 'antd';
import { Pembayaran } from '../types/pembayaran';

interface PembayaranFormProps {
    initialValues?: Pembayaran;
    onSubmit: (values: Partial<Pembayaran>) => void;
    onCancel: () => void;
    loading: boolean;
}

const PembayaranForm: React.FC<PembayaranFormProps> = ({
    initialValues,
    onSubmit,
    onCancel,
    loading
}) => {
    const [form] = Form.useForm();

    const jenisPembayaran = [
        'SPP',
        'Uang Buku & LKS',
        'Sumbangan Pembangunan',
        'Piknik',
        'Lain-lain'
    ];

    return (
        <Form
            form={form}
            layout="vertical"
            onFinish={onSubmit}
            initialValues={initialValues}
        >
            <Form.Item
                name="nama"
                label="Nama Pembayaran"
                rules={[{ required: true, message: 'Nama pembayaran harus diisi!' }]}
            >
                <Input placeholder="Contoh: SPP Bulan Maret 2025" />
            </Form.Item>

            <Form.Item
                name="jenis"
                label="Jenis Pembayaran"
                rules={[{ required: true, message: 'Jenis pembayaran harus dipilih!' }]}
            >
                <Select placeholder="Pilih jenis pembayaran">
                    {jenisPembayaran.map(jenis => (
                        <Select.Option key={jenis} value={jenis}>
                            {jenis}
                        </Select.Option>
                    ))}
                </Select>
            </Form.Item>

            <Form.Item
                name="nominal"
                label="Nominal"
                rules={[{ required: true, message: 'Nominal harus diisi!' }]}
            >
                <InputNumber
                    style={{ width: '100%' }}
                    formatter={value => `Rp ${value}`.replace(/\B(?=(\d{3})+(?!\d))/g, ',')}
                    parser={value => value!.replace(/\Rp\s?|(,*)/g, '')}
                    placeholder="Masukkan nominal pembayaran"
                />
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

export default PembayaranForm;