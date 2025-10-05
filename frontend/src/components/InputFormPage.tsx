import React, { useState } from 'react';
import axios from 'axios';
import toast from 'react-hot-toast';
import { useAuth } from '../context/AuthContext';
import './InputFormPage.css';

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

const InputFormPage: React.FC = () => {
  const { token, logout } = useAuth();
  const [mood, setMood] = useState('');
  const [food1, setFood1] = useState('');
  const [time1, setTime1] = useState('');
  const [food2, setFood2] = useState('');
  const [time2, setTime2] = useState('');

  const now = new Date();
  const date = now.toLocaleDateString('de-DE', { day: '2-digit', month: 'long', year: 'numeric' });
  const time = now.toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit' });

  const resetForm = () => {
    setMood('');
    setFood1('');
    setTime1('');
    setFood2('');
    setTime2('');
  };

  const handleSubmit = async (event: React.FormEvent) => {
    event.preventDefault();
    if (!mood) {
      toast.error("Please select your mood.");
      return;
    }

    const loadingToast = toast.loading('Submitting...');

    const apiPayload: {
      mood: string;
      food1?: string;
      food1Time?: string;
      food2?: string;
      food2Time?: string;
    } = {
      mood: mood,
    };

    if (food1) {
      apiPayload.food1 = food1;
      apiPayload.food1Time = time1;
    }

    if (food2) {
      apiPayload.food2 = food2;
      apiPayload.food2Time = time2;
    }

    try {
      await axios.post('/api/moofoolog', apiPayload, {
        headers: {
          'MooFoo-Token': token,
        },
      });
      toast.success('Submission successful!', { id: loadingToast });
      resetForm();
    } catch (error) {
      console.error('Submission error:', error);
      toast.error('Submission failed. Please try again.', { id: loadingToast });
    }
  };

  return (
    <>
      <div className="header">
        <button onClick={logout} className="logout-button">
          <LogoutIcon />
        </button>
      </div>
      <form className="form-container" onSubmit={handleSubmit}>
        <h2 className="greeting">Hi Sven.</h2>
        <p className="question">Wie geht's dir?</p>

        {/* Mood Radio Group */}
        <div className="radio-group">
          <div className="radio-option">
            <input type="radio" id="mood-mittel" name="mood" value="Mittel" checked={mood === 'Mittel'} onChange={(e) => setMood(e.target.value)} />
            <label htmlFor="mood-mittel">Mittel</label>
          </div>
          <div className="radio-option">
            <input type="radio" id="mood-mah" name="mood" value="Mäh" checked={mood === 'Mäh'} onChange={(e) => setMood(e.target.value)} />
            <label htmlFor="mood-mah">Mäh</label>
          </div>
          <div className="radio-option">
            <input type="radio" id="mood-schlecht" name="mood" value="Schlecht" checked={mood === 'Schlecht'} onChange={(e) => setMood(e.target.value)} />
            <label htmlFor="mood-schlecht">Schlecht</label>
          </div>
        </div>

        {/* Food 1 Section */}
        <div className="input-section">
          <label htmlFor="food-morning">Was hast du gegessen?</label>
          <textarea id="food-morning" name="food-morning" value={food1} onChange={(e) => setFood1(e.target.value)} />
          <div className="radio-group">
              <div className="radio-option">
                  <input type="radio" id="time-morning-1" name="time-1" value="Morgen" checked={time1 === 'Morgen'} onChange={(e) => setTime1(e.target.value)} />
                  <label htmlFor="time-morning-1">Morgen</label>
              </div>
              <div className="radio-option">
                  <input type="radio" id="time-mittag-1" name="time-1" value="Mittag" checked={time1 === 'Mittag'} onChange={(e) => setTime1(e.target.value)} />
                  <label htmlFor="time-mittag-1">Mittag</label>
              </div>
              <div className="radio-option">
                  <input type="radio" id="time-abend-1" name="time-1" value="Abend" checked={time1 === 'Abend'} onChange={(e) => setTime1(e.target.value)} />
                  <label htmlFor="time-abend-1">Abend</label>
              </div>
          </div>
        </div>

        {/* Food 2 Section */}
        <div className="input-section">
          <label htmlFor="food-evening">Und was noch?</label>
          <textarea id="food-evening" name="food-evening" value={food2} onChange={(e) => setFood2(e.target.value)} />
          <div className="radio-group">
              <div className="radio-option">
                  <input type="radio" id="time-morning-2" name="time-2" value="Morgen" checked={time2 === 'Morgen'} onChange={(e) => setTime2(e.target.value)} />
                  <label htmlFor="time-morning-2">Morgen</label>
              </div>
              <div className="radio-option">
                  <input type="radio" id="time-mittag-2" name="time-2" value="Mittag" checked={time2 === 'Mittag'} onChange={(e) => setTime2(e.target.value)} />
                  <label htmlFor="time-mittag-2">Mittag</label>
              </div>
              <div className="radio-option">
                  <input type="radio" id="time-abend-2" name="time-2" value="Abend" checked={time2 === 'Abend'} onChange={(e) => setTime2(e.target.value)} />
                  <label htmlFor="time-abend-2">Abend</label>
              </div>
          </div>
        </div>

        <button type="submit" className="submit-button">SEND</button>

        <div className="footer">
          <p>HEUTE</p>
          <p>{date}, {time} Uhr</p>
        </div>
      </form>
    </>
  );
};

export default InputFormPage;