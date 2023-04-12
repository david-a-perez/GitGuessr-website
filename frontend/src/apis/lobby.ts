import { useCallback } from "react"
import { Auth } from "../hooks/useAuth"

export const useLobbyAPI = (auth: Auth) => ({
  index: useCallback(async (page: number, size: number, filters: LobbyFilters) => {
    const params = new URLSearchParams({ page: page.toString(), page_size: size.toString() })

    Object.entries(filters).forEach(value => {
      if (value[1]) params.append(value[0], value[1])
    })

    return await (await fetch(`/api/lobby?${params}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json()
  }, [auth]),
  get: useCallback(async (id: string): Promise<FullLobby> =>
    await (await fetch(`/api/lobby/${id}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
  create: useCallback(async (createLobby: CreateLobby) =>
    await (await fetch('/api/lobby', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${auth.accessToken}`,
      },
      body: JSON.stringify(createLobby),
    })).json(), [auth]),
  delete: useCallback(async (id: string) =>
    await (await fetch(`/api/lobby/${id}`, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth.accessToken]),
  start: useCallback(async (id: string, startLobby: StartLobby) =>
    await (await fetch(`/api/lobby/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${auth.accessToken}`,
      },
      body: JSON.stringify(startLobby),
    })).json(), [auth.accessToken]),
})