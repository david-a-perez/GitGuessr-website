import React from 'react'
import { useEffect, useState } from 'react'
import { Button } from 'react-bootstrap'
import { useLobbyAPI } from '../apis/lobby'
import { useRepositoryAPI } from '../apis/repository'
import { useAuth } from '../hooks/useAuth'

export const SelectRepository = ({ setRepository }: { setRepository: React.Dispatch<React.SetStateAction<Repository | null>> }) => {
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
    <div style={{ display: "flex", flexFlow: "column", textAlign: "left" }}>
      <div className='mb-4 mt-4 text-center'>
        <h1>Select a Repository</h1>
      </div>
      {(!repositories || repositories.total_items === 0) && "No repositories"}
      <div className='container'>
        <div className='row row-cols-1 row-cols-md-3 g-4'>
          {repositories?.items.map((repository) =>
            <div key={repository.name} className="col">
              <div className="card">
                <div className="card-body">
                  <h4 className="card-title">{repository.name}</h4>
                  <p className="card-text">{repository.description}</p>
                  <a href={repository.url} className="card-link">Repo Link</a>
                  <br />
                  <br />
                  <Button variant='success' onClick={() => setRepository(repository)}>Select</Button>
                </div>
              </div>
            </div>
          )}
        </div>
      </div>
    </div>
  )
}
