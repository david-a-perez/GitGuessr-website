import React from 'react'
import { useEffect, useState } from 'react'
import { useGitGuessrGameFormatConfigAPI } from '../apis/git_guessr_game_format_config'
import { useObfuscatedGameFormatConfigAPI } from '../apis/obfuscated_game_format_config'
import { useAuth } from '../hooks/useAuth'
import { Button } from 'react-bootstrap'

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
            <div className="card">
              <img
                src='https://user-images.githubusercontent.com/46609460/231556799-5e282848-2488-4472-b44e-b10adece3e47.png'
                className="card-img-top"
                alt="..."
                width="100%"
                height="100%"
              />
              <div className="card-body">
                <h5 className="card-title">Git Guessr</h5>
                <p className="card-text">
                  Match a snippet of code to the correct file name.
                  Move quickly, because the clock is ticking.
                </p>
                <Button variant="success" onClick={() => setGameFormat({
                  git_guessr_game_format_config_id: gitGuessrGameFormatConfigId
                })}>Select</Button>
              </div>
            </div>
          </div>
          <div className="col">
            <div className="card">
              <img
                src="https://user-images.githubusercontent.com/46609460/231286336-006c81ba-e855-47e8-9536-6558d412b309.png"
                className="card-img-top"
                alt="..."
                width="100%"
                height="100%"
              />
              <div className="card-body">
                <h5 className="card-title">Obfuscated</h5>
                <p className="card-text">
                  Guess the name of a random function.
                  Keywords may not be as helpful as they appear.
                </p>
                <Button variant="success" onClick={() => setGameFormat({
                  obfuscated_game_format_config_id: obfuscatedGameFormatConfigId
                })}>Select</Button>
              </div>
            </div>
          </div>
        </div>
      </div>
      <div className="text-center" style={{ paddingTop: '10px' }}>
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
