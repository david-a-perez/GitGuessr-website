import { useCallback } from "react"
import { Auth } from "../hooks/useAuth"

export const useGitGuessrPathAPI = (auth: Auth) => ({
  getByLobbyAndPath: useCallback(async (lobby_id: string, path: string) =>
    await (await fetch(`/api/git_guessr_paths/${lobby_id}?path=${encodeURIComponent(path)}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
})