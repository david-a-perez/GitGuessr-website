import { useState } from 'react'
import { useNavigate } from 'react-router-dom'
import { useLobbyAPI } from '../apis/lobby'
import { useLobbyParticipantAPI } from '../apis/lobby_participant'
import { useAuth } from '../hooks/useAuth'
import { SelectGameFormat } from './SelectGameFormat'
import { SelectRepository } from './SelectRepository'

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
          <button
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
            }}>Create Lobby</button>
          <button
            onClick={() => {
              setGameFormat(null)
            }}
          >Back</button>
        </>
      )
    }
  </>)
}
