import { useState } from 'react'
import { useNavigate, useParams } from 'react-router-dom'
import { useLobbyAPI } from '../apis/lobby'
import { useAuth } from '../hooks/useAuth'
import { useAsyncEffect } from 'use-async-effect'
import { Button } from 'react-bootstrap'

export const GameOverPage = () => {
  const navigate = useNavigate()

  const auth = useAuth()
  const { lobby_id } = useParams()
  const [lobby, setLobby] = useState<FullLobby | null>(null)

  const LobbyAPI = useLobbyAPI(auth)

  useAsyncEffect(async isMounted => {
    if (!auth.isAuthenticated || !lobby_id) {
      return
    }

    const lobby = await LobbyAPI.get(lobby_id)

    if (!isMounted()) {
      return
    }

    setLobby(lobby)
  }, [auth.isAuthenticated, lobby_id])

  return (
    <div>
      <br />
      <h1>GAME OVER</h1>
      <h4>Thanks for playing!</h4>
      <br />
      {
        !!lobby?.full_git_guessr_questions.length &&
        <h3>
          Score: {lobby?.full_git_guessr_questions.filter(question => question.correct_answer == question.user_answer).length}
          /
          {lobby?.full_git_guessr_questions.length}
        </h3>
      }
      {
        !!lobby?.full_obfuscated_questions.length &&
        <h3>

          Correct: {lobby?.full_obfuscated_questions.filter(question => question.correct_answer.answer_choice_id == question.user_answer?.answer_choice_id).length}
          /
          {lobby?.full_obfuscated_questions.length}
        </h3>
      }
      <Button variant="success" onClick={() => navigate('/')}>Home</Button>
    </div>
  )
}