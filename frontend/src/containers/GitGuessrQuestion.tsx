import { useState } from 'react'
import Countdown from 'react-countdown'
import { useNavigate, useParams } from 'react-router-dom'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useGitGuessrPathAPI } from '../apis/git_guessr_paths'
import { useGitGuessrQuestionAPI } from '../apis/git_guessr_question'
import { useGitGuessrUserAnswerAPI } from '../apis/git_guessr_user_answer'
import { useAuth } from '../hooks/useAuth'
import { useAsyncEffect } from 'use-async-effect'
import { Button, Breadcrumb } from 'react-bootstrap'

export const GitGuessrQuestion = () => {
  const auth = useAuth()
  const { lobby_id, question_num } = useParams()
  const navigate = useNavigate()
  const [path, setPath] = useState<string[]>([])
  const [selectedAnswer, setSelectedAnswer] = useState<string | null>(null)
  const [pathContents, setPathContents] = useState<Directory | null>(null)
  const [processing, setProcessing] = useState<boolean>(false)
  const [question, setQuestion] = useState<FullGitGuessrQuestion | null>(null)
  const [disableButton, setdisableButton] = useState<boolean>(false)

  const [lobbyParticipant, setLobbyParticipant] = useState<LobbyParticipant | null>(null)

  const QuestionAPI = useGitGuessrQuestionAPI(auth)
  const UserAnswerAPI = useGitGuessrUserAnswerAPI(auth)
  const LobbyParticipantAPI = useLobbyParticipantAPI(auth)
  const PathAPI = useGitGuessrPathAPI(auth)

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
  }, [auth.isAuthenticated, lobby_id, question_num])

  const submitUserAnswer = (path: string) => {
    setProcessing(true)

    if (!lobbyParticipant || !lobby_id || !question) {
      return
    }

    UserAnswerAPI.create({
      lobby_participant_id: lobbyParticipant.id,
      user_id: lobbyParticipant.user_id,
      lobby_id,
      question_id: question.question.id,
      answer: path,
    })

  }

  useAsyncEffect(async isMounted => {
    setProcessing(true)

    if (!auth.isAuthenticated || !lobby_id || !question_num) {
      return
    }

    const pathContents = await PathAPI.getByLobbyAndPath(
      lobby_id,
      path.join('/')
    )

    if (!isMounted()) {
      return
    }

    setPathContents(pathContents)

  }, [auth.isAuthenticated, lobby_id, question_num, path])

  let game_over_time;

  if (question?.question.end_time) {
    game_over_time = new Date(question?.question.end_time)
    game_over_time.setSeconds(game_over_time.getSeconds() + 10)
  }

  return (
    <div>
      <br />
      <div className="container-fluid">
        <div>
          <h3>GitGuessr</h3>
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
          {lobby_id && question?.correct_answer && question?.next_question_start_time &&
            <div>
              <br />
              <p>
                Next Question starts in:
              </p>
              <Countdown date={question?.next_question_start_time}
                onComplete={() => {
                  setPath([])
                  setSelectedAnswer(null)
                  setQuestion(null)
                  setdisableButton(false)
                  navigate(`/git_guessr_question/${lobby_id}/${question.question.question_num + 1}`)
                }} />
            </div>
          }
          {lobby_id && question?.correct_answer && !question?.next_question_start_time &&
            <div>
              <br />
              <p>
                Results in:
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
                  <code>
                    {question?.question.question_text}
                  </code>
                </pre>
              </div>
            </div>
          </div>
          <div className="col-xl">
            <div className="card border-dark mb-3 h-70 w-100">
              <div className="card-header bg-transparent border-dark">
                {!question && "No question"}
                Which file contains the code snippet?
              </div>
              <div className="card-body">
                <Breadcrumb>
                  <Breadcrumb.Item onClick={() => setPath([])}>Root</Breadcrumb.Item>
                  {path.map((folder, index) =>
                    <Breadcrumb.Item key={folder} onClick={() => setPath(path.slice(0, index + 1))}>{folder}</Breadcrumb.Item>
                  )}
                </Breadcrumb>
                <div className='mb-2'>
                  {pathContents?.entries.map(dir =>
                    <span style={{ margin: '5px', display: 'inline-block' }}>
                      {dir.is_directory &&
                        <Button
                          disabled={disableButton}
                          onClick={() => setPath(path.concat([dir.filename]))}
                          variant="success"
                          key={dir.filename}>{dir.filename}</Button>
                      }
                      {!dir.is_directory &&
                        <Button
                          disabled={disableButton}
                          onClick={() =>
                            setSelectedAnswer(path.concat([dir.filename]).join('/'))
                          }
                          variant="light"
                          key={dir.filename}>{dir.filename}
                        </Button>
                      }
                    </span>
                  )}
                </div>
                <div style={{ textAlign: 'left', paddingBottom: '10px', paddingTop: '20px' }}>
                  <h4>Selected Path: {selectedAnswer}</h4>
                  {question?.correct_answer && <h4>Correct Path:  {question?.correct_answer.answer}</h4>}
                </div>
              </div>
              <div className="card-footer">
                <Button
                  disabled={disableButton}
                  variant="success"
                  onClick={() => {
                    selectedAnswer ? submitUserAnswer(selectedAnswer) : null;
                    setdisableButton(selectedAnswer ? true : false);
                  }
                  }
                >
                  Submit
                </Button>
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  )
}


