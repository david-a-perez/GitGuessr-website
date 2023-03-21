import { useEffect, useState } from 'react'
import { useLobbyAPI } from '../apis/lobby'
import { useRepositoryAPI } from '../apis/repository'
import { useAuth } from '../hooks/useAuth'

export const CreateLobby = () => {
  const auth = useAuth()
  const pageSize = 5
  const [page, setPage] = useState<number>(0)
  const [numPages, setPages] = useState<number>(1)
  const [processing, setProcessing] = useState<boolean>(false)
  const [repositories, setRepositories] = useState<PaginationResult<Repository>>()
  const [selectedRepository, setSelectedRepository] = useState<Repository | null>(null)
  const RepositoryAPI = useRepositoryAPI(auth)
  const LobbyAPI = useLobbyAPI(auth)

  const createLobby = async () => {
    setProcessing(true)
    if (selectedRepository)
      await LobbyAPI.create({
        repository_id: selectedRepository.name
      })
    setProcessing(false)
  }

  // const updateLobby = async (lobby: Lobby) => {
  //   setProcessing(true)
  //   if (selectedRepository)
  //     await LobbyAPI.update(lobby.id, {
  //       repository_id: selectedRepository.name
  //     })
  //   setProcessing(false)
  // }

  const deleteLobby = async (lobby: Lobby) => {
    setProcessing(true)
    await LobbyAPI.delete(lobby.id)
    setProcessing(false)
  }

  // fetch on page change
  useEffect(() => {
    setProcessing(true)

    console.log(auth.isAuthenticated)
    
    if (!auth.isAuthenticated) {
      return
    }

    RepositoryAPI.index(page, pageSize).then((repositories) => {
      setRepositories(repositories)
      setProcessing(false)
    })
  }, [auth.isAuthenticated, page])

  // const [oldIsCheckingAuth, setOldIsCheckingAuth] = useState(auth.isCheckingAuth.current);

  // // redirect to login
  // useEffect(() => {
  //   console.log([auth.isAuthenticated, oldIsCheckingAuth, auth.isCheckingAuth.current])
  //   if (!auth.isAuthenticated && oldIsCheckingAuth && !auth.isCheckingAuth.current) {
  //     navigate('/login')
  //     return
  //   }
  //   setOldIsCheckingAuth(auth.isCheckingAuth.current)
  // }, [auth.isAuthenticated, auth.isCheckingAuth.current])

  // update total number of pages
  useEffect(() => {
    if (repositories) setPages(repositories?.num_pages)
  }, [repositories])

  return (
    <div style={{ display: 'flex', flexFlow: 'column', textAlign: 'left' }}>
      <h1>Repositories</h1>
      {(!repositories || repositories.total_items === 0) && "No repositories"}
      {repositories?.items.map((repository) =>
        repository.name === selectedRepository?.name ? (
          <div className="Form">
            <div style={{ flex: 1 }}>
              {repository.name}
            </div>
          </div>
        ) : (
          <div className="Form">
            <div style={{ flex: 1 }} onClick={() => setSelectedRepository(repository)}>
              {repository.name}
            </div>
          </div>
        )
      )}
      {selectedRepository && (
        <div className="Form">
          <div style={{ display: 'flex' }}>
            <button
              disabled={processing}
              style={{ height: '40px' }}
              onClick={() => createLobby()}
            >
              Create Lobby
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
