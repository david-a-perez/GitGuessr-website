import { useEffect, useState } from 'react'
import { Navigate, useParams } from 'react-router-dom'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useNavigate } from 'react-router-dom'
import { useAuth } from '../hooks/useAuth'
import { Button } from 'react-bootstrap'
import Countdown from 'react-countdown'

export const WaitingRoom = () => {
  const { lobby_id } = useParams()
  const navigate = useNavigate()
  const auth = useAuth()
  const pageSize = 5
  const [page, setPage] = useState<number>(0)
  const [numPages, setPages] = useState<number>(1)
  const [lobby, setLobby] = useState<Lobby | null>(null)
  const [lobbyParticipants, setLobbyParticipants] = useState<PaginationResult<LobbyParticipant>>()
  const LobbyAPI = useLobbyAPI(auth)
  const LobbyParticipantAPI = useLobbyParticipantAPI(auth)

  // fetch on page change
  useEffect(() => {
    if (!auth.isAuthenticated || !lobby_id) {
      return
    }

    LobbyAPI.get(lobby_id).then((lobby) => {
      setLobby(lobby.lobby)
    })

    LobbyParticipantAPI.index(page, pageSize, {
      lobby_id
    }).then(lobbyParticipants => {
      setLobbyParticipants(lobbyParticipants)
    })

    const interval = setInterval(() => LobbyParticipantAPI.index(page, pageSize, {
      lobby_id
    }).then(lobbyParticipants => {
      setLobbyParticipants(lobbyParticipants)
    }), 1000)

    const interval2 = setInterval(() => LobbyAPI.get(lobby_id).then((lobby) => {
      setLobby(lobby.lobby)
    }), 1000)

    return () => {
      clearInterval(interval)
      clearInterval(interval2)
    }
  }, [auth.isAuthenticated, page])

  // update total number of pages
  useEffect(() => {
    if (lobbyParticipants) setPages(lobbyParticipants?.num_pages)
  }, [lobbyParticipants])

  return (
    <div style={{ display: 'flex', flexFlow: 'column', textAlign: 'left', paddingLeft: '10%', paddingRight: '10%' }}>
      <div className="mb-4 mt-4 text-center">
        <h1>Lobby: {lobby_id}</h1>
      </div>
      <div className="text-center">
        <h4>Repo: {lobby?.repository_id}</h4>
        {lobby?.git_guessr_game_format_config_id && <h4>Game Mode: GitGuessr</h4>}
        {lobby?.obfuscated_game_format_config_id && <h4>Game Mode: Obfuscated</h4>}
      </div>
      {(!lobbyParticipants || lobbyParticipants.total_items === 0) && "No lobby participants"}
      <table className='table table-striped'>
        <thead>
          <tr>
            <th scope='col'>User ID</th>
          </tr>
        </thead>
        <tbody>
          {lobbyParticipants?.items.map((lobbyParticipant) =>
            <tr key={lobbyParticipant.id}>
              <td>{lobbyParticipant.user_id}</td>
            </tr>
          )}
        </tbody>
      </table>
      <div className="Form">
        <div style={{ display: 'flex' }}>
          <Button
            variant="secondary"
            disabled={page === 0}
            onClick={() => setPage(page - 1)}
          >{`<<`}</Button>
          <span style={{ flex: 1, textAlign: 'center' }}>
            Page {page + 1} of {numPages}
          </span>
          <Button
            variant="secondary"
            disabled={page === numPages - 1}
            onClick={() => setPage(page + 1)}
          >{`>>`}</Button>
        </div>
      </div>
      {lobby_id &&
        <div className='text-center'>
          <Button
            variant='success'
            disabled={!lobbyParticipants || lobbyParticipants.total_items === 0 || !!lobby?.start_time}
            onClick={async () => {
              setLobby(await LobbyAPI.start(lobby_id, { start_time: new Date(Date.now() + 5000) }))
            }}>
            Start
          </Button>
        </div>}
      {lobby?.start_time &&
        <Countdown date={lobby?.start_time}>
          <>
            {(lobby?.git_guessr_game_format_config_id) && (<Navigate to={`/git_guessr_question/${lobby_id}/1`} />)}
            {(lobby?.obfuscated_game_format_config_id) && (<Navigate to={`/obfuscated_question/${lobby_id}/1`} />)}
          </>
        </Countdown>
      }
    </div >
  )
}
