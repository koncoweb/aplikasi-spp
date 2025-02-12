import React, { useEffect, useState } from 'react';
import { Form, Select, DatePicker, InputNumber, Input, Button, Space, FormInstance } from 'antd';
import { Penagihan } from '../types/penagihan';
import { Pembayaran } from '../types/pembayaran';
import { Siswa } from '../types/student';
import { collection, getDocs } from 'firebase/firestore';
import { db } from '../firebase';

interface PenagihanFormProps {
    initialValues?: Penagihan;
    onSubmit: (values: Partial<Penagihan>) => void;
    onCancel: () => void;
    loading: boolean;
    isBatch?: boolean;
    form?: FormInstance;
}

const PenagihanForm: React.FC<PenagihanFormProps> = ({
    initialValues,
    onSubmit,
    onCancel,
    loading,
    isBatch = false,
    form: externalForm
}) => {
    const [internalForm] = Form.useForm();
    const form = externalForm || internalForm;
    const [pembayaranList, setPembayaranList] = useState<Pembayaran[]>([]);
    const [siswaList, setSiswaList] = useState<Siswa[]>([]);

    useEffect(() => {
        fetchPembayaran();
        fetchSiswa();
        form.setFieldsValue(initialValues);
    }, [initialValues, form]);

    const fetchPembayaran = async () => {
        try {
            const querySnapshot = await getDocs(collection(db, 'pembayaran'));
            const pembayaranData = querySnapshot.docs.map(doc => ({
                id: doc.id,
                ...doc.data()
            })) as Pembayaran[];
            setPembayaranList(pembayaranData);
        } catch (error) {
            console.error('Error fetching pembayaran:', error);
        }
    };

    const fetchSiswa = async () => {
        try {
            const querySnapshot = await getDocs(collection(db, 'siswa'));
            const siswaData = querySnapshot.docs.map(doc => ({
                ...doc.data(),
                nis: doc.id
            })) as Siswa[];
            setSiswaList(siswaData);
        } catch (error) {
            console.error('Error fetching siswa:', error);
        }
    };

    const handlePembayaranChange = (value: string) => {
        const pembayaran = pembayaranList.find(p => p.id === value);
        if (pembayaran) {
            form.setFieldsValue({ 
                jumlah_tagihan: pembayaran.nominal,
                nama_pembayaran: pembayaran.nama,
                nama_penagihan: `${pembayaran.nama} ${form.getFieldValue('nama_siswa') || ''}`.trim()
            });
        }
    };

    const handleSiswaChange = (value: string) => {
        const siswa = siswaList.find(s => s.nis === value);
        if (siswa) {
            form.setFieldsValue({
                nama_siswa: siswa.nama_lengkap,
                kelas_siswa: siswa.kelas,
                nama_penagihan: `${form.getFieldValue('nama_pembayaran') || ''} ${siswa.nama_lengkap}`.trim()
            });
        }
    };

    return (
        <Form
            form={form}
            layout="vertical"
            onFinish={(values) => {
                const pembayaran = pembayaranList.find(p => p.id === values.pembayaran_id);
                onSubmit({
                    ...values,
                    nama_pembayaran: pembayaran?.nama || '',
                    nama_penagihan: values.nama_penagihan
                });
            }}
            initialValues={initialValues}
        >
            {isBatch ? (
                <>
                    <Form.Item
                        name="pembayaran_id"
                        label="Pembayaran"
                        rules={[{ required: true, message: 'Pilih pembayaran!' }]}
                    >
                        <Select
                            showSearch
                            placeholder="Pilih pembayaran"
                            optionFilterProp="children"
                            onChange={handlePembayaranChange}
                        >
                            {pembayaranList.map(pembayaran => (
                                <Select.Option key={pembayaran.id} value={pembayaran.id}>
                                    {pembayaran.nama} - {pembayaran.jenis}
                                </Select.Option>
                            ))}
                        </Select>
                    </Form.Item>
                    <Form.Item
                        name="nama_pembayaran"
                        label="Nama Pembayaran"
                    >
                        <Input disabled />
                    </Form.Item>
                </>
            ) : (
                <>
                    <Form.Item
                        name="pembayaran_id"
                        label="Pembayaran"
                        rules={[{ required: true, message: 'Pilih pembayaran!' }]}
                    >
                        <Select
                            showSearch
                            placeholder="Pilih pembayaran"
                            optionFilterProp="children"
                            onChange={handlePembayaranChange}
                        >
                            {pembayaranList.map(pembayaran => (
                                <Select.Option key={pembayaran.id} value={pembayaran.id}>
                                    {pembayaran.nama} - {pembayaran.jenis}
                                </Select.Option>
                            ))}
                        </Select>
                    </Form.Item>

                    <Form.Item
                        name="siswa_nis"
                        label="Siswa"
                        rules={[{ required: true, message: 'Pilih siswa!' }]}
                    >
                        <Select
                            showSearch
                            placeholder="Pilih siswa"
                            optionFilterProp="children"
                            onChange={handleSiswaChange}
                        >
                            {siswaList.map(siswa => (
                                <Select.Option key={siswa.nis} value={siswa.nis}>
                                    {siswa.nama_lengkap} - {siswa.kelas}
                                </Select.Option>
                            ))}
                        </Select>
                    </Form.Item>
                </>
            )}

            <Form.Item
                name="tanggal_tagihan"
                label="Tanggal Tagihan"
                rules={[{ required: true, message: 'Masukkan tanggal tagihan!' }]}
            >
                <DatePicker style={{ width: '100%' }} />
            </Form.Item>

            <Form.Item
                name="jumlah_tagihan"
                label="Jumlah Tagihan"
                rules={[{ required: true, message: 'Masukkan jumlah tagihan!' }]}
            >
                <InputNumber
                    style={{ width: '100%' }}
                    formatter={value => `Rp ${value}`.replace(/\B(?=(\d{3})+(?!\d))/g, ',')}
                    parser={value => value!.replace(/\Rp\s?|(,*)/g, '')}
                />
            </Form.Item>

            <Form.Item
                name="nama_siswa"
                label="Nama Siswa"
            >
                <Input disabled />
            </Form.Item>

            <Form.Item
                name="kelas_siswa"
                label="Kelas"
            >
                <Input disabled />
            </Form.Item>

            <Form.Item
                name="nama_penagihan"
                label="Nama Penagihan"
                rules={[{ required: true, message: 'Masukkan nama penagihan!' }]}
            >
                <Input disabled />
            </Form.Item>

            <Form.Item
                name="keterangan"
                label="Keterangan"
            >
                <Input.TextArea />
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

export default PenagihanForm;