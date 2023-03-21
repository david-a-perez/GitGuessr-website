import React from 'react'
import { useEffect, useState } from 'react'
import { useGitGuessrGameFormatConfigAPI } from '../apis/git_guessr_game_format_config'
import { useObfuscatedGameFormatConfigAPI } from '../apis/obfuscated_game_format_config'
import { useAuth } from '../hooks/useAuth'

export const SelectGameFormat = ({ repository, setRepository, setGameFormat }: {
  repository: Repository, setRepository: React.Dispatch<React.SetStateAction<Repository | null>>, setGameFormat: React.Dispatch<React.SetStateAction<{
    git_guessr_game_format_config_id?: number | undefined;
    obfuscated_game_format_config_id?: number | undefined;
  } | null>>
}) => {
  const auth = useAuth()

  const [gitGuessrGameFormatConfigId, setGitGuessrGameFormatConfigId] = useState<number | undefined>()
  const [obfuscatedGameFormatConfigId, setObfuscatedGameFormatConfigId] = useState<number | undefined>()

  const GitGuessrGameFormatConfigAPI = useGitGuessrGameFormatConfigAPI(auth)
  const ObfuscatedGameFormatConfigAPI = useObfuscatedGameFormatConfigAPI(auth)


  // fetch on page change
  useEffect(() => {
    if (!auth.isAuthenticated) {
      return
    }

    GitGuessrGameFormatConfigAPI.index(repository.name).then((gitGuessrGameFormatConfig: GitGuessrGameFormatConfig | null) => {
      setGitGuessrGameFormatConfigId(gitGuessrGameFormatConfig?.id)
    })
    ObfuscatedGameFormatConfigAPI.index(repository.name).then((obfuscatedGameFormatConfig: ObfuscatedGameFormatConfig | null) => {
      setObfuscatedGameFormatConfigId(obfuscatedGameFormatConfig?.id)
    })
  }, [auth.isAuthenticated])


  return (
    <div style={{ display: 'flex', flexFlow: 'column', textAlign: 'left' }}>
      <h1>Game Formats</h1>
      <div className="Form">
        <button disabled={!gitGuessrGameFormatConfigId} onClick={() => setGameFormat({
          git_guessr_game_format_config_id: gitGuessrGameFormatConfigId
        })}>
          Git Guessr
        </button>
        <button style={{ flex: 1 }} disabled={!obfuscatedGameFormatConfigId} onClick={() => setGameFormat({
          obfuscated_game_format_config_id: obfuscatedGameFormatConfigId
        })}>
          Obfuscated
        </button>
      </div>
      <button
        onClick={() => {
          setRepository(null)
          setGameFormat(null)
        }}
      >Back</button>
    </div>
  )
}
