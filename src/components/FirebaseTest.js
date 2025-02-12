import React, { useEffect, useState } from 'react';
import { db } from '../firebase';
import { collection, getDocs } from 'firebase/firestore';

function FirebaseTest() {
    const [connectionStatus, setConnectionStatus] = useState('Checking connection...');

    useEffect(() => {
        const testConnection = async () => {
            try {
                const querySnapshot = await getDocs(collection(db, 'test'));
                setConnectionStatus('Successfully connected to Firebase! 🎉');
            } catch (error) {
                setConnectionStatus(`Failed to connect to Firebase: ${error.message}`);
                console.error('Firebase connection error:', error);
            }
        };

        testConnection();
    }, []);

    return (
        <div style={{ padding: '20px', textAlign: 'center' }}>
            <h2>Firebase Connection Status</h2>
            <p>{connectionStatus}</p>
        </div>
    );
}

export default FirebaseTest;
