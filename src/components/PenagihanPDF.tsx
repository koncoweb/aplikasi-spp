import React, { useState, useEffect } from 'react';
import { Page, Text, View, Document, StyleSheet, Image } from '@react-pdf/renderer';
import { Penagihan } from '../types/penagihan';
import { getDoc, doc } from 'firebase/firestore';
import { db } from '../firebase';
import logo from '../assets/logo.png';

const styles = StyleSheet.create({
    page: {
        padding: 30,
        fontSize: 12,
    },
    header: {
        flexDirection: 'row',
        marginBottom: 20,
        borderBottom: '2 solid #000',
        paddingBottom: 10,
    },
    logo: {
        width: 60,
        height: 60,
        marginRight: 15,
    },
    schoolInfo: {
        flex: 1,
    },
    schoolName: {
        fontSize: 16,
        fontWeight: 'bold',
        marginBottom: 5,
    },
    schoolAddress: {
        fontSize: 10,
        color: '#666',
    },
    title: {
        fontSize: 14,
        fontWeight: 'bold',
        textAlign: 'center',
        marginBottom: 20,
    },
    section: {
        marginBottom: 10,
    },
    row: {
        flexDirection: 'row',
        marginBottom: 5,
    },
    label: {
        width: 150,
    },
    value: {
        flex: 1,
    },
    separator: {
        marginHorizontal: 5,
    },
    footer: {
        marginTop: 30,
        borderTop: '1 solid #666',
        paddingTop: 10,
        fontSize: 10,
        color: '#666',
    },
    signature: {
        marginTop: 50,
        flexDirection: 'row',
        justifyContent: 'space-between',
    },
    signatureBox: {
        width: 200,
        textAlign: 'center',
    },
    signatureLine: {
        borderTop: '1 solid #000',
        marginTop: 50,
        marginHorizontal: 20,
    },
    amount: {
        marginTop: 20,
        padding: 10,
        backgroundColor: '#f0f0f0',
        borderRadius: 5,
    },
    amountText: {
        fontWeight: 'bold',
    }
});

const formatCurrency = (value: number): string => {
    return `Rp ${value.toLocaleString('id-ID')}`;
};

const formatDate = (date: Date): string => {
    return new Date(date).toLocaleDateString('id-ID', {
        day: 'numeric',
        month: 'long',
        year: 'numeric'
    });
};

const PenagihanPDF: React.FC<{ data: Penagihan }> = ({ data }) => {
    const [schoolSettings, setSchoolSettings] = useState<SchoolSettings | null>(null);

    useEffect(() => {
        const fetchSchoolSettings = async () => {
            const doc = await getDoc(doc(db, 'settings', 'school'));
            if (doc.exists()) {
                setSchoolSettings(doc.data() as SchoolSettings);
            }
        };
        fetchSchoolSettings();
    }, []);

    return (
        <Document>
            <Page size="A4" style={styles.page}>
                <View style={styles.header}>
                    <Image style={styles.logo} src={logo} />
                    <View style={styles.schoolInfo}>
                        <Text style={styles.schoolName}>{schoolSettings?.nama || 'NAMA SEKOLAH'}</Text>
                        <Text style={styles.schoolAddress}>
                            {schoolSettings?.alamat || ''}{'\n'}
                            Telepon: {schoolSettings?.telepon || ''}{'\n'}
                            Email: {schoolSettings?.email || ''}
                        </Text>
                    </View>
                </View>

                <Text style={styles.title}>BUKTI PENAGIHAN PEMBAYARAN</Text>

                <View style={styles.section}>
                    <View style={styles.row}>
                        <Text style={styles.label}>No. Penagihan</Text>
                        <Text style={styles.separator}>:</Text>
                        <Text style={styles.value}>{data.id}</Text>
                    </View>
                    <View style={styles.row}>
                        <Text style={styles.label}>Tanggal</Text>
                        <Text style={styles.separator}>:</Text>
                        <Text style={styles.value}>{formatDate(data.tanggal_tagihan)}</Text>
                    </View>
                </View>

                <View style={styles.section}>
                    <View style={styles.row}>
                        <Text style={styles.label}>Nama Siswa</Text>
                        <Text style={styles.separator}>:</Text>
                        <Text style={styles.value}>{data.nama_siswa}</Text>
                    </View>
                    <View style={styles.row}>
                        <Text style={styles.label}>Kelas</Text>
                        <Text style={styles.separator}>:</Text>
                        <Text style={styles.value}>{data.kelas_siswa}</Text>
                    </View>
                </View>

                <View style={styles.section}>
                    <View style={styles.row}>
                        <Text style={styles.label}>Jenis Pembayaran</Text>
                        <Text style={styles.separator}>:</Text>
                        <Text style={styles.value}>{data.nama_pembayaran}</Text>
                    </View>
                    <View style={styles.row}>
                        <Text style={styles.label}>Keterangan</Text>
                        <Text style={styles.separator}>:</Text>
                        <Text style={styles.value}>{data.keterangan || '-'}</Text>
                    </View>
                </View>

                <View style={styles.amount}>
                    <View style={styles.row}>
                        <Text style={styles.label}>Jumlah Tagihan</Text>
                        <Text style={styles.separator}>:</Text>
                        <Text style={[styles.value, styles.amountText]}>{formatCurrency(data.jumlah_tagihan)}</Text>
                    </View>
                </View>

                <View style={styles.signature}>
                    <View style={styles.signatureBox}>
                        <Text>Petugas,</Text>
                        <View style={styles.signatureLine} />
                        <Text>{data.createdBy}</Text>
                    </View>
                    <View style={styles.signatureBox}>
                        <Text>Wali Murid,</Text>
                        <View style={styles.signatureLine} />
                        <Text>(...........................)</Text>
                    </View>
                </View>

                <View style={styles.footer}>
                    <Text>* Bukti penagihan ini adalah dokumen yang sah</Text>
                    <Text>* Harap simpan bukti ini sebagai arsip</Text>
                    <Text>* Pembayaran dapat dilakukan melalui rekening sekolah atau langsung ke bagian keuangan</Text>
                </View>
            </Page>
        </Document>
    );
};

export default PenagihanPDF; 