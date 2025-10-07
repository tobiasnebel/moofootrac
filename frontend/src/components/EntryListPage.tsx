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

const DeleteIcon = () => (
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
    <polyline points="3 6 5 6 21 6" />
    <path d="M19 6l-1 14a2 2 0 0 1-2 2H8a2 2 0 0 1-2-2L5 6" />
    <path d="M10 11v6" />
    <path d="M14 11v6" />
    <path d="M9 6V4a1 1 0 0 1 1-1h4a1 1 0 0 1 1 1v2" />
  </svg>
);

const ExportIcon = () => (
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
    <path d="M21 15v4a2 2 0 0 1-2 2H5a2 2 0 0 1-2-2v-4" />
    <polyline points="7 10 12 15 17 10" />
    <line x1="12" y1="15" x2="12" y2="3" />
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

  const handleExport = async () => {
    if (!token) {
      toast.error('Authentication token not found.');
      return;
    }
    toast.loading('Exporting data...', { id: 'export-toast' });

    try {
      const response = await axios.get('/api/moofoolog/export', {
        headers: {
          'MooFoo-Token': token,
        },
        responseType: 'blob',
      });

      const url = window.URL.createObjectURL(new Blob([response.data]));
      const link = document.createElement('a');
      link.href = url;
      const contentDisposition = response.headers['content-disposition'];
      let filename = 'moofoolog_export.xlsx';
      if (contentDisposition) {
        const filenameMatch = contentDisposition.match(/filename="(.+)"/);
        if (filenameMatch && filenameMatch.length > 1) {
          filename = filenameMatch[1];
        }
      }
      link.setAttribute('download', filename);
      document.body.appendChild(link);
      link.click();

      if(link.parentNode) {
        link.parentNode.removeChild(link);
      }
      window.URL.revokeObjectURL(url);

      toast.success('Export successful!', { id: 'export-toast' });
    } catch (error) {
        if (axios.isAxiosError(error) && error.response) {
            const reader = new FileReader();
            reader.onload = () => {
                try {
                    const errorData = JSON.parse(reader.result as string);
                    toast.error(`Export failed: ${errorData.error}`, { id: 'export-toast' });
                } catch (e) {
                    toast.error('Export failed with an unknown error.', { id: 'export-toast' });
                }
            };
            reader.readAsText(error.response.data);
          } else {
            toast.error('Export failed.', { id: 'export-toast' });
            console.error(error);
          }
    }
  };

  return (
    <div className="entry-list-page">
      
      <div className="header">
        <button onClick={() => navigate('/form')} className="menu-button">
            <BackIcon />
        </button>
        <div className="header-actions">
          <button onClick={handleExport} className="menu-button">
            <ExportIcon />
          </button>
          <button onClick={logout} className="logout-button">
            <LogoutIcon />
          </button>
        </div>
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
            {/* <button onClick={() => handleDelete(entry.id)} className="delete-button">Delete</button> */}
            <button onClick={() => handleDelete(entry.id)} className="delete-button">
              <DeleteIcon />
            </button>
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