import React from 'react';
import { BrowserRouter as Router, Route, Routes, Navigate } from 'react-router-dom';
import { AuthProvider, useAuth } from './context/AuthContext';
import Login from './components/Login';
import Dashboard from './components/Dashboard';
import KelasPage from './components/Kelas';
import SiswaPage from './components/Siswa';
import PembayaranPage from './components/Pembayaran';
import PenagihanPage from './components/Penagihan';
import Settings from './components/Settings';
import './App.css';

interface PrivateRouteProps {
  children: React.ReactNode;
}

function PrivateRoute({ children }: PrivateRouteProps): JSX.Element {
  const { user, loading } = useAuth();
  
  if (loading) {
    return <div>Loading...</div>;
  }

  return user ? <>{children}</> : <Navigate to="/login" />;
}

function App(): JSX.Element {
  return (
    <AuthProvider>
      <Router>
        <div className="App">
          <Routes>
            <Route path="/login" element={<Login />} />
            <Route
              path="/dashboard"
              element={
                <PrivateRoute>
                  <Dashboard />
                </PrivateRoute>
              }
            >
              <Route path="kelas" element={<KelasPage />} />
              <Route path="siswa" element={<SiswaPage />} />
              <Route path="pembayaran" element={<PembayaranPage />} />
              <Route path="penagihan" element={<PenagihanPage />} />
              <Route path="settings" element={<Settings />} />
            </Route>
            <Route
              path="/"
              element={<Navigate to="/dashboard" replace />}
            />
          </Routes>
        </div>
      </Router>
    </AuthProvider>
  );
}

export default App; 