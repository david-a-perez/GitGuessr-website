import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import { Button } from 'react-bootstrap'

export const AccountPage = () => {
  const auth = useAuth()
  const navigate = useNavigate()

  const [processing, setProcessing] = useState<boolean>(false)
  const [originalPassword, setOriginalPassword] = useState<string>('')
  const [password, setPassword] = useState<string>('')

  const changePassword = async () => {
    setProcessing(true)
    const response = await (
      await fetch('/auth_api/auth/change', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json',
          Authorization: `Bearer ${auth.accessToken}`,
        },
        body: JSON.stringify({
          old_password: originalPassword,
          new_password: password,
        }),
      })
    ).json()
    console.log(response)
    setOriginalPassword('')
    setPassword('')
    setProcessing(false)
  }

  return (
    <div style={{ textAlign: 'left', paddingLeft: '10%', paddingRight: '10%' }}>
      <br />
      <h1>Account</h1>
      <br />
      {auth.isAuthenticated && (
        <div>
          User # {auth.session?.userId}
          <div className='mb-2 mt-2'>
            <br />
            <Button onClick={auth.logout}>Log Out</Button>
          </div>
          <div className="Form" style={{ textAlign: 'left' }}>
            <h1>Change password</h1>
            <br />
            <div style={{ display: 'flex', flexFlow: 'column' }}>
              <label>Original Password</label>
              <input
                type="password"
                value={originalPassword}
                onChange={(e) => setOriginalPassword(e.target.value)}
              />
            </div>
            <div style={{ display: 'flex', flexFlow: 'column' }}>
              <label>New Password</label>
              <input
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
              />
            </div>
            <div style={{ display: 'flex', flexFlow: 'column' }}>
              <button disabled={processing} onClick={changePassword}>
                Change Password
              </button>
            </div>
          </div>
        </div>
      )}
      {!auth.isAuthenticated && (
        <div>
          <a href="#" onClick={() => navigate('/login')}>
            Login to view your account detials
          </a>
        </div>
      )}
    </div>
  )
}
