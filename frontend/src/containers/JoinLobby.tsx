import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useAuth } from '../hooks/useAuth'

export const JoinLobby = () => {
  const auth = useAuth()
  const navigate = useNavigate()
  const pageSize = 5
  const [page, setPage] = useState<number>(0)
  const [numPages, setPages] = useState<number>(1)
  const [processing, setProcessing] = useState<boolean>(false)
  const [lobbies, setLobbies] = useState<PaginationResult<Lobby>>()
  const [selectedLobby, setSelectedLobby] = useState<Lobby | null>(null)
  const LobbyAPI = useLobbyAPI(auth)
  const LobbyParticipantAPI = useLobbyParticipantAPI(auth)


  const createLobbyParticipant = async () => {
    setProcessing(true)
    try {
      if (selectedLobby) {
        if (!(await LobbyParticipantAPI.index(0, 1, {
          lobby_id: selectedLobby.id,
          user_id: auth.session?.userId,
        })).total_items) {
          await LobbyParticipantAPI.create({
            lobby_id: selectedLobby.id,
            user_id: 0
          })
        }

        navigate(`/lobby/${selectedLobby.id}`)
      }
    } catch (e) {
      // TODO: display "Already joined lobby"?
    } finally {
      setProcessing(false)
    }
  }

  // fetch on page change
  useEffect(() => {
    setProcessing(true)

    console.log(auth.isAuthenticated)

    if (!auth.isAuthenticated) {
      return
    }

    LobbyAPI.index(page, pageSize, {
      repository_id: undefined
    }).then((lobbies) => {
      setLobbies(lobbies)
      setProcessing(false)
    })
  }, [auth.isAuthenticated, page])

  // update total number of pages
  useEffect(() => {
    if (lobbies) setPages(lobbies?.num_pages)
  }, [lobbies])

  return (
    <div style={{ display: 'flex', flexFlow: 'column', textAlign: 'left' }}>
      <h1>Lobbies</h1>
      {(!lobbies || lobbies.total_items === 0) && "No lobbies"}
      {lobbies?.items.map((lobby) =>
        lobby.id === selectedLobby?.id ? (
          <div className="Form">
            <div style={{ flex: 1 }}>
              {lobby.id} {lobby.repository_id} {lobby.git_guessr_game_format_config_id && "Git Guessr"} {lobby.obfuscated_game_format_config_id && "Obfuscated"}
            </div>
          </div>
        ) : (
          <div className="Form">
            <div style={{ flex: 1 }} onClick={() => setSelectedLobby(lobby)}>
              {lobby.id} {lobby.repository_id} {lobby.git_guessr_game_format_config_id && "Git Guessr"} {lobby.obfuscated_game_format_config_id && "Obfuscated"}
            </div>
          </div>
        )
      )}
      {selectedLobby && (
        <div className="Form">
          <div style={{ display: 'flex' }}>
            <button
              disabled={processing}
              style={{ height: '40px' }}
              onClick={() => createLobbyParticipant()}
            >
              Join Lobby
            </button>
          </div>
        </div>
      )}
      <div className="Form">
        <div style={{ display: 'flex' }}>
          <button disabled={processing || page === 0} onClick={() => setPage(page - 1)}>{`<<`}</button>
          <span style={{ flex: 1, textAlign: 'center' }}>
            Page {page + 1} of {numPages}
          </span>
          <button
            disabled={processing || page === numPages - 1}
            onClick={() => setPage(page + 1)}
          >{`>>`}</button>
        </div>
      </div>
    </div>
  )
}
