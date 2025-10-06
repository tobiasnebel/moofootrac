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

const EntryListPage: React.FC = () => {
  const [entries, setEntries] = useState<Entry[]>([]);
  const [page, setPage] = useState(0);
  const [isLastPage, setIsLastPage] = useState(false);
  // const { token, userName, logout } = useAuth();
  const { token } = useAuth();
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
      <div className="entry-list-header">
        <h1>Entries</h1>
        <button onClick={handleBack} className="back-button">Back to Input</button>
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