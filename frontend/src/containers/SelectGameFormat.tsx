import React from 'react'
import { useEffect, useState } from 'react'
import { useGitGuessrGameFormatConfigAPI } from '../apis/git_guessr_game_format_config'
import { useObfuscatedGameFormatConfigAPI } from '../apis/obfuscated_game_format_config'
import { useAuth } from '../hooks/useAuth'
import { Button } from 'react-bootstrap'
import reactLogo from '../images/logo.svg'

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
    <div style={{ display: "flex", flexFlow: "column", textAlign: "left" }}>
      <div className="mb-4 mt-4 text-center">
        <h1>Select a Game To Play</h1>
      </div>
      <div className="container">
        <div className="row row-cols-1 row-cols-md-3 g-4">
          <div className="col">
            <div className="card h-80">
              <img src={reactLogo} className="card-img-top" alt="..." />
              <div className="card-body">
                <h5 className="card-title">Git Guessr</h5>
                <p className="card-text">This is placeholder text for the git guessr game.</p>
                <Button variant="success" onClick={() => setGameFormat({
                git_guessr_game_format_config_id: gitGuessrGameFormatConfigId
                })}>Select</Button>
              </div>
            </div>
          </div>
          <div className="col">
            <div className="card h-80">
              <img src={reactLogo} className="card-img-top" alt="..." />
              <div className="card-body">
                <h5 className="card-title">Obfuscated</h5>
                <p className="card-text">This is placeholder text for obfuscated game.</p>
                <Button variant="success" onClick={() => setGameFormat({
                obfuscated_game_format_config_id: obfuscatedGameFormatConfigId
                })}>Select</Button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <br />
      <div className="text-center">
        <Button variant="danger"
          onClick={() => {
            setRepository(null)
            setGameFormat(null)
          }}>
          Back
        </Button>
      </div>
    </div>
  )
}
