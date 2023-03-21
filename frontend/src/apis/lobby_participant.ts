import { useCallback } from "react"
import { Auth } from "../hooks/useAuth"

export const useLobbyParticipantAPI = (auth: Auth) => ({
  index: useCallback(async (page: number, size: number, filters: LobbyParticipantFilters) => {
    const params = new URLSearchParams({ page: page.toString(), page_size: size.toString() })

    Object.entries(filters).forEach(value => {
      if (value[1]) params.append(value[0], value[1])
    })

    return await (await fetch(`/api/lobby_participant?${params}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json()
  }, [auth]),
  get: useCallback(async (id: string) =>
    await (await fetch(`/api/lobby_participant/${id}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth.accessToken]),
  create: useCallback(async (createLobbyParticipant: CreateLobbyParticipant) =>
    await (await fetch('/api/lobby_participant', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${auth.accessToken}`,
      },
      body: JSON.stringify(createLobbyParticipant),
    })).json(), [auth]),
  delete: useCallback(async (id: string) =>
    await (await fetch(`/api/lobby_participant/${id}`, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth.accessToken]),
  update: useCallback(async (id: string, updateLobbyParticipant: UpdateLobbyParticipant) =>
    await (await fetch(`/api/lobby_participant/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${auth.accessToken}`,
      },
      body: JSON.stringify(updateLobbyParticipant),
    })).json(), [auth.accessToken]),
})