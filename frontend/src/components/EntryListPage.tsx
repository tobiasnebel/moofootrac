import React, { useState, useEffect } from 'react';
import axios from 'axios';
import { useAuth } from '../context/AuthContext';
import { toast } from 'react-hot-toast';
import './EntryListPage.css';
import { useNavigate } from 'react-router-dom';

interface Entry {
  id: number;
  mood: string;
  food1: string;
  food1Time: string;
  food2: string;
  food2Time: string;
  timestamp: string;
  userId: string;
}

interface ApiResponse {
  page: number;
  page_size: number;
  data: Entry[];
}

const BackIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <polyline points="15 18 9 12 15 6" />
  </svg>
);

const LogoutIcon = () => (
  <svg
    xmlns="http://www.w3.org/2000/svg"
    width="24"
    height="24"
    viewBox="0 0 24 24"
    fill="none"
    stroke="currentColor"
    strokeWidth="2"
    strokeLinecap="round"
    strokeLinejoin="round"
  >
    <path d="M9 21H5a2 2 0 0 1-2-2V5a2 2 0 0 1 2-2h4" />
    <polyline points="16 17 21 12 16 7" />
    <line x1="21" y1="12" x2="9" y2="12" />
  </svg>
);

const EntryListPage: React.FC = () => {
  const [entries, setEntries] = useState<Entry[]>([]);
  const [page, setPage] = useState(0);
  const [isLastPage, setIsLastPage] = useState(false);
  // const { token, userName, logout } = useAuth();
  // const { token } = useAuth();
  const { token, logout } = useAuth();
  const navigate = useNavigate();

  useEffect(() => {
    const fetchEntries = async () => {
      if (!token) {
        toast.error('Authentication token not found.');
        return;
      }
      try {
        const response = await axios.get<ApiResponse>(`/api/moofoolog?page=${page}`, {
          headers: {
            'MooFoo-Token': token,
          },
        });
        setEntries(response.data.data);
        setIsLastPage(response.data.data.length < response.data.page_size);
      } catch (error) {
        toast.error('Failed to fetch entries.');
        console.error(error);
      }
    };

    fetchEntries();
  }, [token, page]);

  const handleDelete = async (id: number) => {
    if (!token) {
      toast.error('Authentication token not found.');
      return;
    }
    const originalEntries = [...entries];
    setEntries(entries.filter(entry => entry.id !== id));
    toast.loading('Deleting entry...', { id: 'delete-toast' });

    try {
      await axios.delete(`/api/moofoolog/${id}`, {
        headers: {
          'MooFoo-Token': token,
        },
      });
      toast.success('Entry deleted successfully.', { id: 'delete-toast' });
    } catch (error) {
      toast.error('Failed to delete entry.', { id: 'delete-toast' });
      setEntries(originalEntries);
      console.error(error);
    }
  };

  const handleBack = () => {
    navigate('/');
  };

  return (
    <div className="entry-list-page">
      {/* <div className="entry-list-header">
        <h1>Entries</h1>
        <button onClick={handleBack} className="back-button">Back to Input</button>
      </div> */}
      
      <div className="header">
        <button onClick={() => navigate('/form')} className="menu-button">
            <BackIcon />
        </button>
        <button onClick={logout} className="logout-button">
          <LogoutIcon />
        </button>
      </div>
      
      <div className="entry-list-header">
        <h1>Entries</h1>
      </div>
      
      <div className="entries-container">
        {entries.map((entry) => (
          <div key={entry.id} className="entry-kachel">
            <p><strong>Mood:</strong> {entry.mood}</p>
            <p><strong>Food 1:</strong> {entry.food1} ({entry.food1Time})</p>
            <p><strong>Food 2:</strong> {entry.food2} ({entry.food2Time})</p>
            <p><strong>Timestamp:</strong> {new Date(entry.timestamp).toLocaleString()}</p>
            <button onClick={() => handleDelete(entry.id)} className="delete-button">Delete</button>
          </div>
        ))}
      </div>
      <div className="pagination">
        <button onClick={() => setPage(p => Math.max(0, p - 1))} disabled={page === 0}>Previous</button>
        <span>Page {page + 1}</span>
        <button onClick={() => setPage(p => p + 1)} disabled={isLastPage}>Next</button>
      </div>
    </div>
  );
};

export default EntryListPage;