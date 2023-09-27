import { useState, useEffect } from 'react'
import './App.css'

function App() {
  const [message, setMessage] = useState("")

  useEffect(() => {
    getMessage()
  }, [message])

  async function getMessage() {
    const response = await fetch('http://localhost:3030');
    const json = await response.json();
    setMessage(json.message)
}

  return (
    <>
     <h1>Rust + React</h1>

     <button onClick={() => getMessage()}>Get Message</button> 
    </>
  )
}

export default App
