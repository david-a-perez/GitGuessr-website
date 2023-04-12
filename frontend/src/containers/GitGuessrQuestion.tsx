import { useEffect, useState } from 'react'
import Countdown from 'react-countdown'
import { Navigate, useNavigate, useParams } from 'react-router-dom'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useGitGuessrPathAPI } from '../apis/git_guessr_paths'
import { useGitGuessrQuestionAPI } from '../apis/git_guessr_question'
import { useGitGuessrUserAnswerAPI } from '../apis/git_guessr_user_answer'
import { useAuth } from '../hooks/useAuth'
import { useAsyncEffect } from 'use-async-effect'
import { Button, Breadcrumb } from 'react-bootstrap'
import { isNull } from 'util'

export const GitGuessrQuestion = () => {
    const auth = useAuth()
    const { lobby_id, question_num } = useParams()
    const navigate = useNavigate()
    const [path, setPath] = useState<string[]>([])
    const [selectedAnswer, setSelectedAnswer] = useState<string | null>(null)
    const [pathContents, setPathContents] = useState<Directory | null>(null)
    const [processing, setProcessing] = useState<boolean>(false)
    const [question, setQuestion] = useState<FullGitGuessrQuestion | null>(null)
    const [nextQuestion, setNextQuestion] = useState<FullGitGuessrQuestion | null>(null)

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
        const nextQuestion = await QuestionAPI.getByLobbyAndQuestionNum(lobby_id, Number(question_num) + 1)
        if (!isMounted()) return
        setNextQuestion(nextQuestion)
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
                        <div className="card border-dark mb-3 h-100 w-100">
                            <div className="card-header bg-transparent border-dark">
                                {!question && "No question"}
                                Which file contains the code snippet?
                            </div>
                            <div className="card-body">
                                <Breadcrumb>
                                    <Breadcrumb.Item onClick={() => setPath([])}>Root</Breadcrumb.Item>
                                    {path.map(folder =>
                                        <Breadcrumb.Item key={folder}>{folder}</Breadcrumb.Item>
                                    )}
                                </Breadcrumb>
                                {pathContents?.entries.map(dir =>
                                    <>
                                        {dir.is_directory &&
                                            <Button
                                                onClick={() => setPath(path.concat([dir.filename]))}
                                                variant="success"
                                                key={dir.filename}>{dir.filename}</Button>
                                        }
                                        {!dir.is_directory &&
                                            <Button
                                                onClick={()=>
                                                    setSelectedAnswer(path.concat([dir.filename]).join('/'))
                                                }
                                                variant="light"
                                                key={dir.filename}>{dir.filename}</Button>
                                        }
                                    </>
                                )}
                            </div>
                            <div style={{ alignContent: 'left', paddingBottom: '10px' }}>
                                <Button
                                    variant='danger'
                                    size='sm'
                                    onClick={() => setPath(path.slice(0, -1))}
                                >
                                    back
                                </Button>
                            </div>
                            <div className="card-footer">
                                <Button
                                    variant="success"
                                    onClick={()=>selectedAnswer? submitUserAnswer(selectedAnswer) : null}
                                >
                                    Submit
                                </Button>
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
                            navigate(`/git_guessr_question/${lobby_id}/${nextQuestion?.question.question_num}`)
                        }} />
                </div>
            }
        </div>
    )
}


