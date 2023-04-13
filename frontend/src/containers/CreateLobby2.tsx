import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useAuth } from '../hooks/useAuth'
import { SelectGameFormat } from './SelectGameFormat'
import { SelectRepository } from './SelectRepository'
import { Button } from 'react-bootstrap'

export const CreateLobby = () => {
  const auth = useAuth()
  const navigate = useNavigate()
  const [repository, setRepository] = useState<Repository | null>(null);
  const [gameFormat, setGameFormat] = useState<{ git_guessr_game_format_config_id?: number, obfuscated_game_format_config_id?: number } | null>(null);
  const [processing, setProcessing] = useState<boolean>(false)
  const LobbyAPI = useLobbyAPI(auth)
  const LobbyParticipantAPI = useLobbyParticipantAPI(auth)


  return (<>
    {
      !repository && (
        <SelectRepository
          setRepository={setRepository} />
      )
    }
    {
      repository && !gameFormat && (
        <SelectGameFormat
          repository={repository}
          setRepository={setRepository}
          setGameFormat={setGameFormat} />
      )
    }
    {
      repository && gameFormat && (
        <>
          <br />
          <br />
          <div className="card w-50 mx-auto bg-light">
            <div className="card-body">
              <h4 className="card-title">Custom Lobby</h4>
              <p className="card-text">
                Repository: {repository.name}<br />
              </p>
              <Button
                variant="danger"
                onClick={() => {
                  setGameFormat(null)
                }}
              >Back</Button>
              <Button
                variant="success"
                disabled={processing || !auth.isAuthenticated}
                onClick={async () => {
                  setProcessing(true)
                  const lobby = await LobbyAPI.create({
                    repository_id: repository.name,
                    ...gameFormat
                  })
                  LobbyParticipantAPI.create({
                    user_id: 0,
                    lobby_id: lobby.id,
                  })
                  navigate(`/lobby/${lobby.id}`)
                  setProcessing(false)
                }}>Create Lobby</Button>
            </div>
          </div>
        </>
      )
    }
  </>)
}
