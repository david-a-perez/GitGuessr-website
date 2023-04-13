import { useNavigate } from 'react-router-dom'
import { Button } from 'react-bootstrap'

export const GameOverPage = () => {
    const navigate = useNavigate()

    return (
        <div>
            <br />
            <h1>GAME OVER</h1>
            <h4>Thanks for playing!</h4>
            <br />
            <Button variant="success" onClick={() => navigate('/')}>Home</Button>
        </div>
    )
}