import { initializeApp } from 'firebase/app';
import { getFirestore } from 'firebase/firestore';
import { getAuth } from 'firebase/auth';

const firebaseConfig = {
  apiKey: "AIzaSyDdglX-UCem01jsS90XtAFLNADNkZfITpY",
  authDomain: "proyekcontoh-8e1c4.firebaseapp.com",
  projectId: "proyekcontoh-8e1c4",
  storageBucket: "proyekcontoh-8e1c4.firebasestorage.app",
  messagingSenderId: "110782257634",
  appId: "1:110782257634:web:f714c8a964d94c087181bb"
};

const app = initializeApp(firebaseConfig);
const db = getFirestore(app);
const auth = getAuth(app);

export { db, auth };
