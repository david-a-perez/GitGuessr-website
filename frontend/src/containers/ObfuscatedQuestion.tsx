import { useEffect, useState } from 'react'
import Countdown from 'react-countdown'
import { Navigate, useNavigate, useParams } from 'react-router-dom'
import { useObfuscatedAnswerChoiceAPI } from '../apis/obfuscated_answer_choice'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useObfuscatedQuestionAPI } from '../apis/obfuscated_question'
import { useObfuscatedUserAnswerAPI } from '../apis/obfuscated_user_answer'
import { useAuth } from '../hooks/useAuth'
import { useAsyncEffect } from 'use-async-effect'

export const ObfuscatedQuestion = () => {
  const auth = useAuth()
  const { lobby_id, question_num } = useParams()
  const navigate = useNavigate()
  const [checked, setChecked] = useState<number | null>(null)
  const [processing, setProcessing] = useState<boolean>(false)
  const [question, setQuestion] = useState<FullObfuscatedQuestion | null>(null)
  const [disableButtons, setDisableButtons] = useState<boolean>(false)

  const [lobbyParticipant, setLobbyParticipant] = useState<LobbyParticipant | null>(null)

  const ObfuscatedQuestionAPI = useObfuscatedQuestionAPI(auth)
  const ObfuscatedUserAnswerAPI = useObfuscatedUserAnswerAPI(auth)
  const LobbyParticipantAPI = useLobbyParticipantAPI(auth)

  useAsyncEffect(async isMounted => {
    setProcessing(true)

    if (!auth.isAuthenticated || !lobby_id || !question_num) {
      return
    }

    const lobbyParticipant = await LobbyParticipantAPI.index(0, 1, {
      lobby_id: lobby_id,
      user_id: auth.session?.userId,
    })

    if (isMounted() && lobbyParticipant.items.length > 0) {
      setLobbyParticipant(lobbyParticipant.items[0])
    }

  }, [auth.isAuthenticated, lobby_id])


  useAsyncEffect(async isMounted => {
    setProcessing(true)

    if (!auth.isAuthenticated || !lobby_id || !question_num) {
      return
    }
    const question = await ObfuscatedQuestionAPI.getByLobbyAndQuestionNum(lobby_id, Number(question_num))
    if (!isMounted()) return
    setQuestion(question)
    setProcessing(false)
  }, [auth.isAuthenticated, lobby_id, question_num])

  const submitUserAnswer = (answerChoice: ObfuscatedAnswerChoice) => {
    setProcessing(true)

    if (!lobbyParticipant || !lobby_id || !question) {
      return
    }

    ObfuscatedUserAnswerAPI.create({
      lobby_participant_id: lobbyParticipant.id,
      user_id: lobbyParticipant.user_id,
      lobby_id,
      question_id: question.question.id,
      answer_choice_id: answerChoice.id,
    })
  }

  let game_over_time;

  if (question?.question.end_time) {
    game_over_time = new Date(question?.question.end_time)
    game_over_time.setSeconds(game_over_time.getSeconds() + 5)
  }

  return (
    <div>
      <br />
      <div className="container-fluid">
        <div>
          <h3>OBFUSCATED</h3>
        </div>
        <div className="row">
          <div className="col-xl">
            <h5>QUESTION: {question_num}</h5>
          </div>
          {lobby_id && question?.question.end_time && !question.correct_answer &&
            <div className='col-xl'>
              <h2>Time Remaining: </h2>
              <Countdown
                date={question?.question.end_time}
                onComplete={() => {
                  console.log("Completed countdown")
                  setProcessing(true)
                  ObfuscatedQuestionAPI.getByLobbyAndQuestionNum(lobby_id, Number(question_num)).then((question) => {
                    setQuestion(question)
                    setProcessing(false)
                  })
                }} />
            </div>
          }
          {lobby_id && question?.correct_answer && question?.next_question_start_time &&
            <div>
              <br />
              <p>
                Next Question starts in:
              </p>
              <Countdown date={question?.next_question_start_time}
                onComplete={() => {
                  setQuestion(null)
                  setDisableButtons(false)
                  navigate(`/obfuscated_question/${lobby_id}/${question.question.question_num + 1}`)
                }} />
            </div>
          }
          {lobby_id && question?.correct_answer && !question?.next_question_start_time &&
            <div>
              <br />
              <p>
                Exiting game in:
              </p>
              <Countdown date={game_over_time}
                onComplete={() => {
                  setQuestion(null)
                  navigate(`/game_over/${lobby_id}`)
                }} />
            </div>
          }
        </div>
      </div>
      <br />
      <div className="container-fluid">
        <div className="row">
          <div className="col-xl">
            <div className="card border-dark mb-3 h-100 w-100" style={{ maxHeight: '80vh', overflowY: 'scroll' }}>
              <div className="card-header bg-transparent border-dark">CODE SNIPPET</div>
              <div className="card-body text-success">
                <pre style={{ textAlign: 'left' }}>
                  <code style={{ whiteSpace: 'pre'}}>
                    {question?.question.question_text}
                  </code>
                </pre>
              </div>
            </div>
          </div>
          <div className="col-xl" style={{ height: '60vh' }}>
            <div className="card border-dark mb-3 h-100 w-100">
              <div className="card-header bg-transparent border-dark">
                {!question && "No question"}
                What is the identity of the ANSWER keyword?
              </div>
              <div className="card-body">
                {question?.answer_choices.map(answerChoice =>
                  <div key={answerChoice.id} style={{ paddingTop: '20px' }}>
                    <button type="button"
                      disabled={disableButtons}
                      className={question.correct_answer?.answer_choice_id == answerChoice.id ? "btn btn-success" : question.correct_answer?.answer_choice_id != answerChoice.id && question.user_answer?.answer_choice_id == answerChoice.id ? "btn btn-danger" : checked == answerChoice.id ? "btn btn-secondary" : "btn btn-outline-secondary"}
                      onClick={() => {
                        submitUserAnswer(answerChoice);
                        setChecked(answerChoice.id);
                        setDisableButtons(true);
                      }
                      }
                      style={{ width: '100%' }}
                    >
                      {answerChoice.answer}
                    </button>
                  </div>
                )}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}


