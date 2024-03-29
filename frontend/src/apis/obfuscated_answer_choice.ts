import { useCallback } from "react"
import { Auth } from "../hooks/useAuth"

export const useObfuscatedAnswerChoiceAPI = (auth: Auth) => ({
  index: useCallback(async (page: number, size: number) =>
    await (await fetch(`/api/obfuscated_answer_choice?page=${page}&page_size=${size}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
  get: useCallback(async (id: string) =>
    await (await fetch(`/api/obfuscated_answer_choice/${id}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
})