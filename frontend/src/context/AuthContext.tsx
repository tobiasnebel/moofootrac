import React, { createContext, useState, useContext, type ReactNode } from 'react';

interface AuthContextType {
  token: string | null;
  isLoggedIn: boolean;
  login: (token: string) => void;
  logout: () => void;
}

const AuthContext = createContext<AuthContextType | undefined>(undefined);

export const AuthProvider: React.FC<{ children: ReactNode }> = ({ children }) => {
  const [token, setToken] = useState<string | null>(null);

  const login = (newToken: string) => {
    setToken(newToken);
    // In a real app, you'd also save this to localStorage
  };

  const logout = () => {
    setToken(null);
    // And clear it from localStorage
  };

  const isLoggedIn = !!token;

  return (
    <AuthContext.Provider value={{ token, isLoggedIn, login, logout }}>
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