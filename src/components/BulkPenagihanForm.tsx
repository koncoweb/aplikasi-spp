import React, { useState } from 'react';
import { Form, Select, Input, InputNumber, DatePicker, Button, Space } from 'antd';
import { KelasType } from '../types/kelas';
import PenagihanForm from './PenagihanForm';

interface BulkPenagihanFormProps {
    onSubmit: (values: any) => void;
    onCancel: () => void;
    loading: boolean;
    kelasList: KelasType[];
}

const BulkPenagihanForm: React.FC<BulkPenagihanFormProps> = ({
    onSubmit,
    onCancel,
    loading,
    kelasList
}) => {
    const [filterType, setFilterType] = useState<'tingkat' | 'kelas'>('tingkat');
    const [form] = Form.useForm();
    const [penagihanForm] = Form.useForm();

    const tingkatOptions = Array.from(new Set(kelasList.map(k => k.tingkat)))
        .map(tingkat => ({
            label: `Tingkat ${tingkat}`,
            value: tingkat
        }));

    const handleSubmit = (values: any) => {
        const filterValues = form.getFieldsValue();
        const penagihanValues = penagihanForm.getFieldsValue();
        onSubmit({
            ...filterValues,
            ...penagihanValues
        });
    };

    return (
        <Form
            form={form}
            layout="vertical"
            onFinish={handleSubmit}
        >
            <Form.Item
                name="filter_type"
                label="Filter Berdasarkan"
            >
                <Select
                    onChange={(value) => setFilterType(value)}
                    defaultValue="tingkat"
                >
                    <Select.Option value="tingkat">Tingkat</Select.Option>
                    <Select.Option value="kelas">Kelas</Select.Option>
                </Select>
            </Form.Item>

            {filterType === 'tingkat' ? (
                <Form.Item
                    name="tingkat"
                    label="Pilih Tingkat"
                    rules={[{ required: true, message: 'Pilih tingkat!' }]}
                >
                    <Select options={tingkatOptions} />
                </Form.Item>
            ) : (
                <Form.Item
                    name="kelas_id"
                    label="Pilih Kelas"
                    rules={[{ required: true, message: 'Pilih kelas!' }]}
                >
                    <Select>
                        {kelasList.map(kelas => (
                            <Select.Option key={kelas.id} value={kelas.id}>
                                {kelas.namaKelas} - Tingkat {kelas.tingkat}
                            </Select.Option>
                        ))}
                    </Select>
                </Form.Item>
            )}

            <PenagihanForm
                form={penagihanForm}
                onCancel={onCancel}
                loading={loading}
                isBatch={true}
            />

            <Form.Item>
                <Space>
                    <Button type="primary" htmlType="submit" loading={loading}>
                        Buat Penagihan Massal
                    </Button>
                    <Button onClick={onCancel}>Batal</Button>
                </Space>
            </Form.Item>
        </Form>
    );
};

export default BulkPenagihanForm; 