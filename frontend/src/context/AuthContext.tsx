import React, { createContext, useState, useContext, type ReactNode } from 'react';

interface AuthContextType {
  token: string | null;
  userName: string | null;
  isLoggedIn: boolean;
  login: (token: string, userName: string) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [token, setToken] = useState<string | null>(localStorage.getItem('token'));
  const [userName, setUserName] = useState<string | null>(localStorage.getItem('userName'));

  const login = (newToken: string, newUserName: string) => {
    setToken(newToken);
    setUserName(newUserName);
    localStorage.setItem('token', newToken);
    localStorage.setItem('userName', newUserName);
  };

  const logout = () => {
    setToken(null);
    setUserName(null);
    localStorage.removeItem('token');
    localStorage.removeItem('userName');
  };

  const isLoggedIn = !!token;

  return (
    <AuthContext.Provider value={{ token, userName, isLoggedIn, login, logout }}>
      {children}
    </AuthContext.Provider>
  );
};

export const useAuth = () => {
  const context = useContext(AuthContext);
  if (context === undefined) {
    throw new Error('useAuth must be used within an AuthProvider');
  }
  return context;
};