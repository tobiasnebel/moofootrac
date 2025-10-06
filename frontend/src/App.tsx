import { BrowserRouter as Router, Routes, Route, Navigate } from 'react-router-dom';
import { Toaster } from 'react-hot-toast';
import { AuthProvider, useAuth } from './context/AuthContext';
import LoginPage from './components/LoginPage';
import InputFormPage from './components/InputFormPage';
import EntryListPage from './components/EntryListPage';
import './App.css';

const ProtectedRoute: React.FC<{ children: React.ReactElement }> = ({ children }) => {
  const { isLoggedIn } = useAuth();
  if (!isLoggedIn) {
    return <Navigate to="/" replace />;
  }
  return children;
};

const AppRoutes = () => {
  const { isLoggedIn } = useAuth();

  return (
    <Routes>
      <Route path="/" element={isLoggedIn ? <Navigate to="/form" /> : <LoginPage />} />
      <Route
        path="/form"
        element={
          <ProtectedRoute>
            <InputFormPage />
          </ProtectedRoute>
        }
      />
      <Route
        path="/entries"
        element={
          <ProtectedRoute>
            <EntryListPage />
          </ProtectedRoute>
        }
      />
    </Routes>
  );
};

const App = () => {
  return (
    <Router basename="/app">
      <AuthProvider>
        <div className="App">
          <Toaster position="top-center" reverseOrder={false} />
          <AppRoutes />
        </div>
      </AuthProvider>
    </Router>
  );
};

export default App;