import { useEffect, useState } from 'react'
import Countdown from 'react-countdown'
import { Navigate, useNavigate, useParams } from 'react-router-dom'
import { useAnswerChoiceAPI } from '../apis/answer_choice'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useQuestionAPI } from '../apis/question'
import { useUserAnswerAPI } from '../apis/user_answer'
import { useAuth } from '../hooks/useAuth'
import { useAsyncEffect } from 'use-async-effect'

export const Question = () => {
  const auth = useAuth()
  const { lobby_id, question_num } = useParams()
  const navigate = useNavigate()
  const [processing, setProcessing] = useState<boolean>(false)
  const [question, setQuestion] = useState<FullQuestion | null>(null)
  const [nextQuestion, setNextQuestion] = useState<FullQuestion | null>(null)

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

  const submitUserAnswer = (answerChoice: AnswerChoice) => {
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
    <div style={{ display: 'flex', flexFlow: 'column', textAlign: 'left' }}>
      <h1>Question</h1>
      {!question && "No question"}
      {question?.question.question_text}
      {question?.answer_choices.map(answerChoice =>
        <div key={answerChoice.id} className="Form">
          <div style={{ flex: 1 }} onClick={() => submitUserAnswer(answerChoice)}>
            {answerChoice.answer} {answerChoice.id === question.user_answer?.answer_choice_id && "User Answer"} {answerChoice.id === question.correct_answer?.answer_choice_id && "Correct Answer"}
          </div>
        </div>
      )}
      {lobby_id && question?.question.end_time && !question.correct_answer &&
        <>
          Question ends in
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
        </>
      }
      {lobby_id && nextQuestion?.question.start_time && question?.correct_answer &&
        <>
          Next Question starts in
          <Countdown date={nextQuestion?.question.start_time}
            onComplete={() => {
              console.log("2x Complete" + nextQuestion?.question.question_num)
              setQuestion(null)
              setNextQuestion(null)
              navigate(`/question/${lobby_id}/${nextQuestion?.question.question_num}`)
            }} /></>}
    </div>
  )
}


