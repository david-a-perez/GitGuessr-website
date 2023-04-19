import { useEffect, useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useAuth } from '../hooks/useAuth'
import { Button } from 'react-bootstrap'

export const JoinLobby = () => {
  const auth = useAuth()
  const navigate = useNavigate()
  const pageSize = 5
  const [page, setPage] = useState<number>(0)
  const [numPages, setPages] = useState<number>(1)
  const [processing, setProcessing] = useState<boolean>(false)
  const [lobbies, setLobbies] = useState<PaginationResult<Lobby>>()
  const [selectedLobby, setSelectedLobby] = useState<string>('')
  const LobbyAPI = useLobbyAPI(auth)
  const LobbyParticipantAPI = useLobbyParticipantAPI(auth)


  const createLobbyParticipant = async () => {
    setProcessing(true)
    try {
      if (selectedLobby) {
        if (!(await LobbyParticipantAPI.index(0, 1, {
          lobby_id: selectedLobby,
          user_id: auth.session?.userId,
        })).total_items) {
          await LobbyParticipantAPI.create({
            lobby_id: selectedLobby,
            user_id: 0
          })
        }

        navigate(`/lobby/${selectedLobby}`)
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
    <div style={{ display: 'flex', flexFlow: 'column', textAlign: 'left', paddingLeft: '10%', paddingRight: '10%' }}>
      <div className="mb-4 mt-4 text-center">
        <h1>Join Lobby</h1>
      </div>
      <div className='text-center'>
        <div>
        <h4>Enter a Lobby ID</h4>
        <input value={selectedLobby} onChange={(e) => setSelectedLobby(e.target.value)} />
        </div>
        <Button style={{marginTop: '10px'}} onClick={() => createLobbyParticipant()}>Join</Button>
      </div>
    </div>
  )
}
