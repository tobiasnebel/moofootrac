import React from 'react';
import './InputFormPage.css';

const InputFormPage: React.FC = () => {
  // Get current date and time for the footer
  const now = new Date();
  const date = now.toLocaleDateString('de-DE', { day: '2-digit', month: 'long', year: 'numeric' });
  const time = now.toLocaleTimeString('de-DE', { hour: '2-digit', minute: '2-digit' });

  return (
    <div className="form-container">
      <h2 className="greeting">Hi Sven.</h2>
      <p className="question">Wie geht's dir?</p>

      <div className="radio-group">
        <div className="radio-option">
          <input type="radio" id="mood-mittel" name="mood" value="Mittel" />
          <label htmlFor="mood-mittel">Mittel</label>
        </div>
        <div className="radio-option">
          <input type="radio" id="mood-mah" name="mood" value="Mäh" />
          <label htmlFor="mood-mah">Mäh</label>
        </div>
        <div className="radio-option">
          <input type="radio" id="mood-schlecht" name="mood" value="Schlecht" />
          <label htmlFor="mood-schlecht">Schlecht</label>
        </div>
      </div>

      <div className="input-section">
        <label htmlFor="food-morning">Was hast du gegessen?</label>
        <textarea id="food-morning" name="food-morning" />
        <div className="radio-group">
            <div className="radio-option">
                <input type="radio" id="time-morning-1" name="time-1" value="Morgen" />
                <label htmlFor="time-morning-1">Morgen</label>
            </div>
            <div className="radio-option">
                <input type="radio" id="time-mittag-1" name="time-1" value="Mittag" />
                <label htmlFor="time-mittag-1">Mittag</label>
            </div>
            <div className="radio-option">
                <input type="radio" id="time-abend-1" name="time-1" value="Abend" />
                <label htmlFor="time-abend-1">Abend</label>
            </div>
        </div>
      </div>

      <div className="input-section">
        <label htmlFor="food-evening">Und was noch?</label>
        <textarea id="food-evening" name="food-evening" />
        <div className="radio-group">
            <div className="radio-option">
                <input type="radio" id="time-morning-2" name="time-2" value="Morgen" />
                <label htmlFor="time-morning-2">Morgen</label>
            </div>
            <div className="radio-option">
                <input type="radio" id="time-mittag-2" name="time-2" value="Mittag" />
                <label htmlFor="time-mittag-2">Mittag</label>
            </div>
            <div className="radio-option">
                <input type="radio" id="time-abend-2" name="time-2" value="Abend" />
                <label htmlFor="time-abend-2">Abend</label>
            </div>
        </div>
      </div>

      <button type="submit">SEND</button>

      <div className="footer">
        <p>HEUTE</p>
        <p>{date}, {time} Uhr</p>
      </div>
    </div>
  );
};

export default InputFormPage;