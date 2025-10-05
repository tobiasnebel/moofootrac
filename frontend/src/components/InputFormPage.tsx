import React, { useState } from 'react';
import './InputFormPage.css';

const InputFormPage: React.FC = () => {
  const [mood, setMood] = useState('');
  const [food1, setFood1] = useState('');
  const [time1, setTime1] = useState('');
  const [food2, setFood2] = useState('');
  const [time2, setTime2] = useState('');

  // Get current date and time for the footer
  const now = new Date();
  const date = now.toLocaleDateString('de-DE', { day: '2-digit', month: 'long', year: 'numeric' });
  const time = now.toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit' });

  const handleSubmit = (event: React.FormEvent) => {
    event.preventDefault();

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

    console.log(apiPayload);
    // Here we would send the data to the API
  };

  return (
    <form className="form-container" onSubmit={handleSubmit}>
      <h2 className="greeting">Hi Sven.</h2>
      <p className="question">Wie geht's dir?</p>

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
  );
};

export default InputFormPage;