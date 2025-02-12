import React, { useState, useEffect } from 'react';
import { Form, Input, Button, Card, message, Space, Tabs, Alert } from 'antd';
import { collection, doc, getDoc, setDoc } from 'firebase/firestore';
import { db } from '../firebase';
import { CopyOutlined } from '@ant-design/icons';

interface FirebaseConfig {
    apiKey: string;
    authDomain: string;
    projectId: string;
    storageBucket: string;
    messagingSenderId: string;
    appId: string;
}

interface SchoolSettings {
    nama: string;
    alamat: string;
    telepon: string;
    email: string;
}

const { TabPane } = Tabs;

const Settings: React.FC = () => {
    const [form] = Form.useForm();
    const [schoolForm] = Form.useForm();
    const [loading, setLoading] = useState(false);
    const [schoolLoading, setSchoolLoading] = useState(false);

    useEffect(() => {
        fetchSettings();
        fetchSchoolSettings();
    }, []);

    const fetchSettings = async () => {
        try {
            const settingsDoc = await getDoc(doc(db, 'settings', 'firebase'));
            if (settingsDoc.exists()) {
                form.setFieldsValue(settingsDoc.data());
            }
        } catch (error) {
            console.error('Error fetching settings:', error);
            message.error('Gagal mengambil pengaturan');
        }
    };

    const handleSubmit = async (values: FirebaseConfig) => {
        setLoading(true);
        try {
            await setDoc(doc(db, 'settings', 'firebase'), {
                ...values,
                updatedAt: new Date(),
                updatedBy: 'admin'
            });
            message.success('Pengaturan berhasil disimpan');
            generateEnvFile(values);
        } catch (error) {
            console.error('Error saving settings:', error);
            message.error('Gagal menyimpan pengaturan');
        }
        setLoading(false);
    };

    const generateEnvFile = (values: FirebaseConfig) => {
        const envContent = `REACT_APP_FIREBASE_API_KEY=${values.apiKey}
REACT_APP_FIREBASE_AUTH_DOMAIN=${values.authDomain}
REACT_APP_FIREBASE_PROJECT_ID=${values.projectId}
REACT_APP_FIREBASE_STORAGE_BUCKET=${values.storageBucket}
REACT_APP_FIREBASE_MESSAGING_SENDER_ID=${values.messagingSenderId}
REACT_APP_FIREBASE_APP_ID=${values.appId}`;

        const blob = new Blob([envContent], { type: 'text/plain' });
        const url = window.URL.createObjectURL(blob);
        const a = document.createElement('a');
        a.href = url;
        a.download = '.env';
        a.click();
        window.URL.revokeObjectURL(url);
    };

    const copyToClipboard = (text: string) => {
        navigator.clipboard.writeText(text).then(() => {
            message.success('Berhasil disalin ke clipboard');
        });
    };

    const currentEnvValues = {
        apiKey: process.env.REACT_APP_FIREBASE_API_KEY || '',
        authDomain: process.env.REACT_APP_FIREBASE_AUTH_DOMAIN || '',
        projectId: process.env.REACT_APP_FIREBASE_PROJECT_ID || '',
        storageBucket: process.env.REACT_APP_FIREBASE_STORAGE_BUCKET || '',
        messagingSenderId: process.env.REACT_APP_FIREBASE_MESSAGING_SENDER_ID || '',
        appId: process.env.REACT_APP_FIREBASE_APP_ID || '',
    };

    const fetchSchoolSettings = async () => {
        try {
            const settingsDoc = await getDoc(doc(db, 'settings', 'school'));
            if (settingsDoc.exists()) {
                const data = settingsDoc.data();
                schoolForm.setFieldsValue(data);
            }
        } catch (error) {
            console.error('Error fetching school settings:', error);
            message.error('Gagal mengambil pengaturan sekolah');
        }
    };

    const handleSchoolSubmit = async (values: SchoolSettings) => {
        setSchoolLoading(true);
        try {
            await setDoc(doc(db, 'settings', 'school'), {
                ...values,
                updatedAt: new Date(),
                updatedBy: 'admin'
            });
            message.success('Pengaturan sekolah berhasil disimpan');
        } catch (error) {
            console.error('Error saving school settings:', error);
            message.error('Gagal menyimpan pengaturan sekolah');
        }
        setSchoolLoading(false);
    };

    return (
        <div style={{ padding: '24px' }}>
            <Card style={{ maxWidth: 800, margin: '0 auto' }}>
                <Tabs defaultActiveKey="1">
                    <TabPane tab="Pengaturan Sekolah" key="1">
                        <Form
                            form={schoolForm}
                            layout="vertical"
                            onFinish={handleSchoolSubmit}
                        >
                            <Form.Item
                                name="nama"
                                label="Nama Sekolah"
                                rules={[{ required: true, message: 'Masukkan nama sekolah!' }]}
                            >
                                <Input />
                            </Form.Item>

                            <Form.Item
                                name="alamat"
                                label="Alamat"
                                rules={[{ required: true, message: 'Masukkan alamat sekolah!' }]}
                            >
                                <Input.TextArea rows={3} />
                            </Form.Item>

                            <Form.Item
                                name="telepon"
                                label="Telepon"
                                rules={[{ required: true, message: 'Masukkan nomor telepon!' }]}
                            >
                                <Input />
                            </Form.Item>

                            <Form.Item
                                name="email"
                                label="Email"
                                rules={[
                                    { required: true, message: 'Masukkan email!' },
                                    { type: 'email', message: 'Email tidak valid!' }
                                ]}
                            >
                                <Input />
                            </Form.Item>

                            <Form.Item>
                                <Space>
                                    <Button type="primary" htmlType="submit" loading={schoolLoading}>
                                        Simpan Pengaturan Sekolah
                                    </Button>
                                    <Button onClick={() => schoolForm.resetFields()}>
                                        Reset
                                    </Button>
                                </Space>
                            </Form.Item>
                        </Form>
                    </TabPane>

                    <TabPane tab="Konfigurasi Firebase" key="2">
                        <Form
                            form={form}
                            layout="vertical"
                            onFinish={handleSubmit}
                        >
                            <Form.Item
                                name="apiKey"
                                label="API Key"
                                rules={[{ required: true, message: 'Masukkan API Key!' }]}
                            >
                                <Input.Password />
                            </Form.Item>

                            <Form.Item
                                name="authDomain"
                                label="Auth Domain"
                                rules={[{ required: true, message: 'Masukkan Auth Domain!' }]}
                            >
                                <Input />
                            </Form.Item>

                            <Form.Item
                                name="projectId"
                                label="Project ID"
                                rules={[{ required: true, message: 'Masukkan Project ID!' }]}
                            >
                                <Input />
                            </Form.Item>

                            <Form.Item
                                name="storageBucket"
                                label="Storage Bucket"
                                rules={[{ required: true, message: 'Masukkan Storage Bucket!' }]}
                            >
                                <Input />
                            </Form.Item>

                            <Form.Item
                                name="messagingSenderId"
                                label="Messaging Sender ID"
                                rules={[{ required: true, message: 'Masukkan Messaging Sender ID!' }]}
                            >
                                <Input />
                            </Form.Item>

                            <Form.Item
                                name="appId"
                                label="App ID"
                                rules={[{ required: true, message: 'Masukkan App ID!' }]}
                            >
                                <Input.Password />
                            </Form.Item>

                            <Form.Item>
                                <Space>
                                    <Button type="primary" htmlType="submit" loading={loading}>
                                        Simpan & Generate .env
                                    </Button>
                                    <Button onClick={() => form.resetFields()}>
                                        Reset
                                    </Button>
                                </Space>
                            </Form.Item>
                        </Form>
                    </TabPane>

                    <TabPane tab="Environment Variables" key="3">
                        <Alert
                            message="Current Environment Variables"
                            description="These are the current environment variables being used by the application."
                            type="info"
                            showIcon
                            style={{ marginBottom: 16 }}
                        />
                        {Object.entries(currentEnvValues).map(([key, value]) => (
                            <div key={key} style={{ marginBottom: 16 }}>
                                <Input
                                    addonBefore={`REACT_APP_FIREBASE_${key.toUpperCase()}`}
                                    value={value}
                                    readOnly
                                    addonAfter={
                                        <CopyOutlined 
                                            onClick={() => copyToClipboard(`REACT_APP_FIREBASE_${key.toUpperCase()}=${value}`)}
                                            style={{ cursor: 'pointer' }}
                                        />
                                    }
                                />
                            </div>
                        ))}
                    </TabPane>
                </Tabs>
            </Card>
        </div>
    );
};

export default Settings; 