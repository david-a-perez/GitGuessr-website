import { useCallback } from "react"
import { Auth } from "../hooks/useAuth"

export const useGitGuessrQuestionAPI = (auth: Auth) => ({
  index: useCallback(async (page: number, size: number) =>
    await (await fetch(`/api/git_guessr_question?page=${page}&page_size=${size}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
  get: useCallback(async (id: string) =>
    await (await fetch(`/api/git_guessr_question/${id}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
  getByLobbyAndQuestionNum: useCallback(async (lobby_id: string, question_num: number) =>
    await (await fetch(`/api/git_guessr_question/${lobby_id}/${question_num}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
})