import React from 'react'
import { useEffect, useState } from 'react'
import { useLobbyAPI } from '../apis/lobby'
import { useRepositoryAPI } from '../apis/repository'
import { useAuth } from '../hooks/useAuth'

export const SelectRepository = ({setRepository}: { setRepository: React.Dispatch<React.SetStateAction<Repository | null>>}) => {
  const auth = useAuth()
  const pageSize = 5
  const [page, setPage] = useState<number>(0)
  const [numPages, setPages] = useState<number>(1)
  const [repositories, setRepositories] = useState<PaginationResult<Repository>>()
  const RepositoryAPI = useRepositoryAPI(auth)

  // fetch on page change
  useEffect(() => {
    if (!auth.isAuthenticated) {
      return
    }

    RepositoryAPI.index(page, pageSize).then((repositories) => {
      setRepositories(repositories)
    })
  }, [auth.isAuthenticated, page])

  // update total number of pages
  useEffect(() => {
    if (repositories) setPages(repositories?.num_pages)
  }, [repositories])

  return (
    <div style={{ display: 'flex', flexFlow: 'column', textAlign: 'left' }}>
      <h1>Repositories</h1>
      {(!repositories || repositories.total_items === 0) && "No repositories"}
      {repositories?.items.map((repository) =>
          <div key={repository.name} className="Form">
            <div style={{ flex: 1 }} onClick={() => setRepository(repository)}>
              {repository.name}
            </div>
          </div>
      )}
      <div className="Form">
        <div style={{ display: 'flex' }}>
          <button disabled={page === 0} onClick={() => setPage(page - 1)}>{`<<`}</button>
          <span style={{ flex: 1, textAlign: 'center' }}>
            Page {page + 1} of {numPages}
          </span>
          <button
            disabled={page === numPages - 1}
            onClick={() => setPage(page + 1)}
          >{`>>`}</button>
        </div>
      </div>
    </div>
  )
}
