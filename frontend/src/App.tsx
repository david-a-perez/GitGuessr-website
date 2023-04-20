import React from 'react'
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
import { CreateLobby } from './containers/CreateLobby'
import { Route, Routes, Link } from 'react-router-dom'
import { JoinLobby } from './containers/JoinLobby'
import { WaitingRoom } from './containers/WaitingRoom'
import { ObfuscatedQuestion } from './containers/ObfuscatedQuestion'
import { GitGuessrQuestion } from './containers/GitGuessrQuestion'
import { GameOverPage } from './containers/GameOverPage'
import { Nav, Navbar, Container } from 'react-bootstrap';

const App = () => {
  useAuthCheck()
  const auth = useAuth()
  /* CRA: app hooks */

  // @ts-ignore
  return (
    <div className="App">
      <Navbar variant="dark" bg="dark" expand="lg">
        <Container>
          <Navbar.Brand>
            <img
              src="https://user-images.githubusercontent.com/46609460/231286372-f3968e6c-b5c3-4e11-a1aa-22f76541830c.png"
              alt=""
              width="50"
              height="40"
            />
          </Navbar.Brand>
          <Navbar.Toggle aria-controls="basic-navbar-nav" />
          <Navbar.Collapse id="basic-navbar-nav">
            <Nav className="me-auto">
              <Link className="nav-link active" to='/'>Home</Link>
              {auth.isAuthenticated &&
                <Link className="nav-link active" to="/create_lobby">Create Lobby</Link>
              }
              {auth.isAuthenticated &&
                <Link className="nav-link active" to="/join_lobby">Join Lobby</Link>
              }
              {auth.isAuthenticated && <Link className="nav-link active" to="/account">Account</Link>}
              {!auth.isAuthenticated && <Link className="nav-link active" to="/login">Login/Sign-Up</Link>}
            </Nav>
          </Navbar.Collapse>
        </Container>
      </Navbar>
      <div style={{ margin: '0 auto' }}>
        <Routes>
          <Route path="/" element={<Home />} />
          <Route path="/create_lobby" element={<CreateLobby />} />
          {/*<Route path="/select_repo" element={<SelectRepository />} />*/}
          <Route path="/join_lobby" element={<JoinLobby />} />
          <Route path="/lobby/:lobby_id" element={<WaitingRoom />} />
          <Route path="/game_over/:lobby_id" element={<GameOverPage />} />
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
