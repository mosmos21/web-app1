import React from 'react'
import axios from 'axios'

const App: React.FC = () => {
  const [text, setText] = React.useState('')

  const handleClickButton = () => {
    axios
      .post('/api/echo', { value: text })
      .then(({data}) => alert(data))
      .catch(console.error)
  }

  return (
    <div>
      <div>
        <input
          type="text"
          value={text}
          onChange={(e) => setText(e.target.value)} />
      </div>
      <div>
        <button onClick={handleClickButton}>Echo</button>
      </div>
    </div>
  )
}

export default App
