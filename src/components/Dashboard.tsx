import React from 'react';
import { Layout, Menu, Button } from 'antd';
import { Link, Outlet, useNavigate } from 'react-router-dom';
import { useAuth } from '../context/AuthContext';
import {
    TeamOutlined,
    BookOutlined,
    DollarOutlined,
    FileOutlined,
    LogoutOutlined,
    SettingOutlined
} from '@ant-design/icons';

const { Header, Sider, Content } = Layout;

const Dashboard: React.FC = () => {
    const { user } = useAuth();
    const navigate = useNavigate();

    const menuItems = [
        {
            key: 'kelas',
            icon: <BookOutlined />,
            label: <Link to="kelas">Kelas</Link>
        },
        {
            key: 'siswa',
            icon: <TeamOutlined />,
            label: <Link to="siswa">Siswa</Link>
        },
        {
            key: 'pembayaran',
            icon: <DollarOutlined />,
            label: <Link to="pembayaran">Pembayaran</Link>
        },
        {
            key: 'penagihan',
            icon: <FileOutlined />,
            label: <Link to="penagihan">Penagihan</Link>
        },
        {
            key: 'settings',
            icon: <SettingOutlined />,
            label: <Link to="settings">Pengaturan</Link>
        }
    ];

    return (
        <Layout style={{ minHeight: '100vh' }}>
            <Sider>
                <div style={{ padding: '16px', color: 'white' }}>
                    <h2>SPP Sekolah</h2>
                </div>
                <Menu
                    theme="dark"
                    mode="inline"
                    defaultSelectedKeys={['kelas']}
                    items={menuItems}
                />
            </Sider>
            <Layout>
                <Header style={{ padding: '0 16px', background: '#fff', display: 'flex', justifyContent: 'flex-end', alignItems: 'center' }}>
                    <span style={{ marginRight: '16px' }}>{user?.email}</span>
                    <Button
                        type="primary"
                        icon={<LogoutOutlined />}
                        onClick={() => navigate('/login')}
                    >
                        Logout
                    </Button>
                </Header>
                <Content style={{ margin: '16px' }}>
                    <Outlet />
                </Content>
            </Layout>
        </Layout>
    );
};

export default Dashboard; 