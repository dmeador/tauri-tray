import { useState } from 'react'
import logo from './logo.svg'
import './App.css'

import React, { useEffect } from 'react'
//import  Component  from 'react'
//import soundfile from 'Fire_engine_sound_effect_siren_sounds_free_download_via_converter_no_silence.mp3'
//import "./styles.css"
//../assets/
//import Sound from 'react-sound'

function App() {
  const timeString = "en"
  //const timeString = "fa-IR"
  const [count, setCount] = useState(0)
  const [time, setTime] = useState(new Date().toLocaleTimeString(timeString))
  useEffect(() => {
    const time = () => {
      const event = new Date();
      setTime(event.toLocaleTimeString(timeString));
    };
    const intervalId = setInterval(time, 1000);

    return () => {
      clearInterval(intervalId);
    };
  }, []);
  
  function checkCount(count)  {
      if ( count > 3)
      {
        new Audio('Fire_engine_sound_effect_siren_sounds_free_download_via_converter_no_silence.mp3').play();
      }
  }


  
  return (
    <div className="App">
      <header className="App-header">
        <img src={logo} className="App-logo" alt="logo" />
        <p>Hello Vite + React!</p>
        <p>
          <button type="button" onClick={() => {setCount((count) => count + 1);  checkCount( count );} }>
            count is: {count}
           
          </button>
        </p>
        <p>
        <h1>{time}</h1>
          Edited and saved App.jsx.
        </p>
        <p>
          <a
            className="App-link"
            href="https://reactjs.org"
            target="_blank"
            rel="noopener noreferrer"
          >
            Learn React
          </a>
          {' | '}
          <a
            className="App-link"
            href="https://vitejs.dev/guide/features.html"
            target="_blank"
            rel="noopener noreferrer"
          >
            Vite Docs
          </a>
        </p>
 
      </header>
    </div>
  )
}

export default App
