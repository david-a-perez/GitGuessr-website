import { useEffect, useState } from 'react'
import Countdown from 'react-countdown'
import { Navigate, useNavigate, useParams } from 'react-router-dom'
import { useAnswerChoiceAPI } from '../apis/obfuscated_answer_choice'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useQuestionAPI } from '../apis/obfuscated_question'
import { useUserAnswerAPI } from '../apis/obfuscated_user_answer'
import { useAuth } from '../hooks/useAuth'
import { useAsyncEffect } from 'use-async-effect'

export const ObfuscatedQuestion = () => {
  const auth = useAuth()
  const { lobby_id, question_num } = useParams()
  const navigate = useNavigate()
  const [checked, setChecked] = useState<number | null>(null)
  const [processing, setProcessing] = useState<boolean>(false)
  const [question, setQuestion] = useState<FullObfuscatedQuestion | null>(null)
  const [nextQuestion, setNextQuestion] = useState<FullObfuscatedQuestion | null>(null)

  const [lobbyParticipant, setLobbyParticipant] = useState<LobbyParticipant | null>(null)

  const QuestionAPI = useQuestionAPI(auth)
  const AnswerChoiceAPI = useAnswerChoiceAPI(auth)
  const UserAnswerAPI = useUserAnswerAPI(auth)
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
    const question = await QuestionAPI.getByLobbyAndQuestionNum(lobby_id, Number(question_num))
    if (!isMounted()) return
    setQuestion(question)
    setProcessing(false)
    const nextQuestion = await QuestionAPI.getByLobbyAndQuestionNum(lobby_id, Number(question_num) + 1)
    if (!isMounted()) return
    setNextQuestion(nextQuestion)
  }, [auth.isAuthenticated, lobby_id, question_num])

  const submitUserAnswer = (answerChoice: ObfuscatedAnswerChoice) => {
    setProcessing(true)

    if (!lobbyParticipant || !lobby_id || !question) {
      return
    }

    UserAnswerAPI.create({
      lobby_participant_id: lobbyParticipant.id,
      user_id: lobbyParticipant.user_id,
      lobby_id,
      question_id: question.question.id,
      answer_choice_id: answerChoice.id,
    })
  }

  return (
    <div>
      <br/>
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
                QuestionAPI.getByLobbyAndQuestionNum(lobby_id, Number(question_num)).then((question) => {
                  setQuestion(question)
                  setProcessing(false)
                })
              }} />
          </div>
          }
        </div>
      </div>
      <br/>
      <div className="container-fluid">
        <div className="row">
          <div className="col-xl">
            <div className="card border-dark mb-3 h-100 w-100">
              <div className="card-header bg-transparent border-dark">CODE SNIPPET</div>
              <div className="card-body text-success">
                <pre>
                  <code>
                    This is an example of some code I could put 
                    for(let i=0; i!=10; i++)
                        func(i);
                  </code>
                </pre>
              </div>
            </div>
          </div>
          <div className="col-xl">
            <div className="card border-dark mb-3 h-100 w-100">
              <div className="card-header bg-transparent border-dark">
                {!question && "No question"}
                {question?.question.question_text}
              </div>
              <div className="card-body">
                {question?.answer_choices.map(answerChoice =>
                <div key={answerChoice.id} className="Form">
                  <button type="button" className={question.correct_answer?.answer_choice_id == answerChoice.id ? "btn btn-success" : question.correct_answer?.answer_choice_id != answerChoice.id && question.user_answer?.answer_choice_id == answerChoice.id ? "btn btn-danger" : checked==answerChoice.id ? "btn btn-secondary" : "btn btn-outline-secondary"} 
                  onClick={() => {
                    submitUserAnswer(answerChoice);
                    setChecked(answerChoice.id);}
                  }
                  >
                    {answerChoice.answer} {answerChoice.id === question.correct_answer?.answer_choice_id && "Correct Answer"}
                  </button>
                </div>
                )}
              </div>     
            </div>
          </div>
        </div>
      </div>
      {lobby_id && nextQuestion?.question.start_time && question?.correct_answer &&
        <div>
          <br />
          <p>
            Next Question starts in: 
          </p>
          <Countdown date={nextQuestion?.question.start_time}
            onComplete={() => {
              console.log("2x Complete" + nextQuestion?.question.question_num)
              setQuestion(null)
              setNextQuestion(null)
              navigate(`/obfuscated_question/${lobby_id}/${nextQuestion?.question.question_num}`)
            }} />
        </div>
      }
    </div>
  )
}


