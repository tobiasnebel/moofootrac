import toast from 'react-hot-toast';
import { useAuth } from '../context/AuthContext';
import React, { useState } from 'react';
import axios from 'axios';
import './LoginPage.css';

const LoginPage: React.FC = () => {
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const { login } = useAuth();

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    const loadingToast = toast.loading('Logging in...');

    try {
      // Base64
      const credentials = btoa(`${username}:${password}`);
      // Header
      const authHeader = `Basic ${credentials}`;

      // OLD: // const response = await axios.get(`/api/login?auth=${password}`);
      // NEW
      const response = await axios.get('/api/login', {
        headers: {
          Authorization: authHeader,
        },
      });

      if (response.data && response.data.token) {
        login(response.data.token, username);
        toast.success('Login successful!', { id: loadingToast });
      } else {
        // Handle cases where the response does not contain a token
        toast.error('Login failed: No token received.', { id: loadingToast });
      }
    } catch (error) {
      console.error('Login error:', error);
      toast.error('Login failed. Please check your credentials.', { id: loadingToast });
    }
  };

  return (
    <div className="login-container">
      <h1>Login</h1>
      <form onSubmit={handleSubmit}>
        <div>
          <label htmlFor="username">Username</label>
          <input
            type="text"
            id="username"
            name="username"
            value={username}
            onChange={(e) => setUsername(e.target.value)}
          />
        </div>
        <div>
          <label htmlFor="password">Password</label>
          <input
            type="password"
            id="password"
            name="password"
            value={password}
            onChange={(e) => setPassword(e.target.value)}
          />
        </div>
        <button type="submit">Login</button>
      </form>
    </div>
  );
};

export default LoginPage;