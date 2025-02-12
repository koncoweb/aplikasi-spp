import React, { useEffect, useState } from 'react';
import { Form, Select, DatePicker, InputNumber, Input, Button, Space, FormInstance } from 'antd';
import { Penagihan } from '../types/penagihan';
import { Pembayaran } from '../types/pembayaran';
import { Siswa } from '../types/student';
import { collection, getDocs } from 'firebase/firestore';
import { db } from '../firebase';
import dayjs from 'dayjs';

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
        if (initialValues) {
            // Convert date fields to dayjs objects
            const formValues = {
                ...initialValues,
                tanggal_tagihan: initialValues.tanggal_tagihan ? dayjs(initialValues.tanggal_tagihan) : undefined,
                tanggal_bayar: initialValues.tanggal_bayar ? dayjs(initialValues.tanggal_bayar) : undefined
            };
            form.setFieldsValue(formValues);
        }
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

    const handleFinish = (values: any) => {
        // Convert dayjs objects back to Date objects before submitting
        const submittedValues = {
            ...values,
            tanggal_tagihan: values.tanggal_tagihan?.toDate(),
            tanggal_bayar: values.tanggal_bayar?.toDate()
        };
        onSubmit(submittedValues);
    };

    return (
        <Form
            form={form}
            layout="vertical"
            onFinish={handleFinish}
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
