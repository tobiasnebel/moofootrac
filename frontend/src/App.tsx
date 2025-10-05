import { BrowserRouter as Router, Routes, Route, useNavigate } from 'react-router-dom';
import LoginPage from './components/LoginPage';
import InputFormPage from './components/InputFormPage';
import './App.css';

const AppContent = () => {
  const navigate = useNavigate();

  const handleLogin = () => {
    // Mock login logic
    console.log('User logged in');
    navigate('/form');
  };

  return (
    <Routes>
      <Route path="/" element={<LoginPage onLogin={handleLogin} />} />
      <Route path="/form" element={<InputFormPage />} />
    </Routes>
  );
};

const App = () => {
  return (
    <Router>
      <div className="App">
        <AppContent />
      </div>
    </Router>
  );
};

export default App;