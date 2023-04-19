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

  let score = 0

  const git_gussr_questions = lobby?.full_git_guessr_questions.map(question => {
    const correct_split = question.correct_answer.answer.split("/")
    const user_split = question.user_answer?.answer.split("/")
    let is_correct = true
    let correct_count = 0

    const correct_answers = correct_split.map((correct_filename, i) => {
      if (is_correct && user_split && correct_filename == user_split[i]) {
        correct_count += 1
        return <span style={{ fontWeight: "bold", color: 'green' }}>/{correct_filename}</span>
      } else {
        is_correct = false
        return <span style={{ fontWeight: "bold", color: 'red' }}>/{correct_filename}</span>
      }
    })

    score += correct_count / correct_split.length * 100

    return <div style={{padding: "10px", fontSize: "20px"}}>
      <div>Your Answer: {question.user_answer && "/"}{question.user_answer?.answer}</div>
      <div>
        Correct Answer: {correct_answers}
      </div>
      <div>
        Score: {Math.floor(correct_count / correct_split.length * 100)} / 100
      </div>
    </div>
  })

  return (
    <div>
      <br />
      <h1>GAME OVER</h1>
      <h4>Thanks for playing!</h4>
      <br />

      {
        git_gussr_questions
      }

      {
        !!lobby?.full_git_guessr_questions.length &&
        <h3 style={{paddingTop: "20px"}}>
          Score: {Math.floor(score)}
          /
          {lobby?.full_git_guessr_questions.length*100}
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
