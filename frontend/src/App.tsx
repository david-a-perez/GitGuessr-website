import { useAuth, useAuthCheck } from './hooks/useAuth'
import { AccountPage } from './containers/AccountPage'
import { LoginPage } from './containers/LoginPage'
import { ActivationPage } from './containers/ActivationPage'
import { RegistrationPage } from './containers/RegistrationPage'
import { RecoveryPage } from './containers/RecoveryPage'
import { ResetPage } from './containers/ResetPage'
import { SelectRepository } from './containers/SelectRepository'
import './App.css'
import 'bootstrap/dist/css/bootstrap.css'
import { Home } from './containers/Home'
import { Todos } from './containers/Todo'
import { CreateLobby } from './containers/CreateLobby2'
import { Route, useNavigate, Routes } from 'react-router-dom'
import { JoinLobby } from './containers/JoinLobby'
import { WaitingRoom } from './containers/WaitingRoom'
import { ObfuscatedQuestion } from './containers/ObfuscatedQuestion'
import { GitGuessrQuestion } from './containers/GitGuessrQuestion'
import { GameOverPage } from './containers/GameOverPage'

const App = () => {
  useAuthCheck()
  const auth = useAuth()

  const navigate = useNavigate()
  /* CRA: app hooks */

  // @ts-ignore
  return (
    <div className="App">
      <nav className="navbar navbar-expand-lg navbar-dark bg-dark">
        <div className="container-fluid">
          <span className="navbar-brand">
            <img
              src="https://user-images.githubusercontent.com/46609460/231286372-f3968e6c-b5c3-4e11-a1aa-22f76541830c.png"
              alt=""
              width="50"
              height="40"
            />
          </span>
          <div className="collapse navbar-collapse" id="navbarNav">
            <ul className="navbar-nav">
              <li className="nav-item">
                <a className="nav-link active" href='/'>Home</a>
              </li>
              <li>
                {auth.isAuthenticated &&
                  <a className="nav-link active" href="/create_lobby">Create Lobby</a>
                }
              </li>
              <li>
                {auth.isAuthenticated &&
                  <a className="nav-link active" href="/join_lobby">Join Lobby</a>
                }
              </li>
              <li className="nav-item">
                {auth.isAuthenticated && <a className="nav-link active" href="/account">Account</a>}
                {!auth.isAuthenticated && <a className="nav-link active" href="/login">Login/Sign-Up</a>}
              </li>
            </ul>
          </div>
        </div>
      </nav>
      <div style={{ margin: '0 auto' }}>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/todos" element={<Todos />} />
          <Route path="/create_lobby" element={<CreateLobby />} />
          {/*<Route path="/select_repo" element={<SelectRepository />} />*/}
          <Route path="/join_lobby" element={<JoinLobby />} />
          <Route path="/lobby/:lobby_id" element={<WaitingRoom />} />
          <Route path="/game_over" element={<GameOverPage />} />
          <Route path="/obfuscated_question/:lobby_id/:question_num" element={<ObfuscatedQuestion />} />
          <Route path="/git_guessr_question/:lobby_id/:question_num" element={<GitGuessrQuestion />} />
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
