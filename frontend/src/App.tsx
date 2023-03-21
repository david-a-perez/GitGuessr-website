import { useAuth, useAuthCheck } from './hooks/useAuth'
import { AccountPage } from './containers/AccountPage'
import { LoginPage } from './containers/LoginPage'
import { ActivationPage } from './containers/ActivationPage'
import { RegistrationPage } from './containers/RegistrationPage'
import { RecoveryPage } from './containers/RecoveryPage'
import { ResetPage } from './containers/ResetPage'
import React from 'react'
import './App.css'
import { Home } from './containers/Home'
import { Todos } from './containers/Todo'
import { CreateLobby } from './containers/CreateLobby2'
import { Route, useNavigate, Routes } from 'react-router-dom'
import { JoinLobby } from './containers/JoinLobby'
import { WaitingRoom } from './containers/WaitingRoom'
import { Question } from './containers/Question'

const App = () => {
  useAuthCheck()
  const auth = useAuth()
    
  const navigate = useNavigate()
  /* CRA: app hooks */
  
  // @ts-ignore
  return (
    <div className="App">
      <div className="App-nav-header">
        <div style={{ display: 'flex', flex: 1 }}>
          <a className="NavButton" onClick={() => navigate('/')}>Home</a>
          <a className="NavButton" onClick={() => navigate('/todos')}>Todos</a>
          {/* CRA: left-aligned nav buttons */}
          <a className="NavButton" onClick={() => navigate('/account')}>Account</a>
          {auth.isAuthenticated &&
            <>
              <a className="NavButton" onClick={() => navigate('/create_lobby')}>Create Lobby</a>
              <a className="NavButton" onClick={() => navigate('/join_lobby')}>Join Lobby</a>
            </>}

        </div>
        <div>
          {/* CRA: right-aligned nav buttons */}
          { auth.isAuthenticated && <a className="NavButton" onClick={() => auth.logout()}>Logout</a> }
          { !auth.isAuthenticated && <a className="NavButton" onClick={() => navigate('/login')}>Login/Register</a> }
        </div>
      </div>
      <div style={{ margin: '0 auto', maxWidth: '800px' }}>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/todos" element={<Todos />} />
          <Route path="/create_lobby" element={<CreateLobby />} />
          <Route path="/join_lobby" element={<JoinLobby />} />
          <Route path="/lobby/:lobby_id" element={<WaitingRoom />} />
          <Route path="/question/:lobby_id/:question_num" element={<Question />} />

          {/* CRA: routes */}
            <Route path="/login" element={<LoginPage />} />
            <Route path="/recovery" element={<RecoveryPage />} />
            <Route path="/reset" element={<ResetPage />} />
            <Route path="/activate" element={<ActivationPage />} />
            <Route path="/register" element={<RegistrationPage />} />
            <Route path="/account" element={<AccountPage />} />
    
        </Routes>
      </div>
    </div>
  )
}

export default App
