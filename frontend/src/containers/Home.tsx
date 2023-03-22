import { Button } from 'react-bootstrap'
import { useAuth } from '../hooks/useAuth'
import { useNavigate } from 'react-router-dom'

export const Home = () => {
  const auth = useAuth()
  const navigate = useNavigate()

  if(!auth.isAuthenticated)
  {
    return (
      <div>
        <br />
        <h1 className='font-monospace'>Welcome to GITGUESSR</h1>
        <br />
        <br />
        <h2>Login or Sign Up to Play!</h2>
      </div>
    )
  }

  return (
    <div>
      <h1 className='mb-5 mt-4'>Welcome to GitGuessr!</h1>
      <div className='menu' style={{textAlign: 'left'}}>
        <div className='outer-container'>
          <div className='row'>
            <div className='col'>
              <div className='inner-container'>
                <div className='row'>
                  <div className='column'>
                    <Button className='mb-3 btn-lg' variant='light' onClick={() => navigate('/repos')}>Single Player</Button>
                  </div>
                  <div className='column'></div>
                </div>
                <div className='row'>
                  <div className='column'>
                    <Button className='mb-3 btn-lg' variant='light' onClick={() => navigate('/create_lobby')}>Create Lobby</Button>
                  </div>
                  <div className='column'></div>
                </div>
                <div className='row'>
                  <div className='column'>
                    <Button className='mb-3 btn-lg' variant='light' onClick={() => navigate('/join_lobby')}>Join Lobby</Button>
                  </div>
                  <div className='column'></div>
                </div>
              </div>
            </div>
            <div className='col'>
              <img src='https://user-images.githubusercontent.com/46609460/220524085-2e913612-03a5-431e-a326-013cd66d10bf.png' alt='logo' width='450' height='300'/>
            </div>
          </div>
        </div>
        
      </div>
    </div>
  )
}
