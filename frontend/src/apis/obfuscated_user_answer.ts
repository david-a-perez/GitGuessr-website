import { useCallback } from "react"
import { Auth } from "../hooks/useAuth"

export const useUserAnswerAPI = (auth: Auth) => ({
  index: useCallback(async (page: number, size: number) =>
    await (await fetch(`/api/user_answer?page=${page}&page_size=${size}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
  get: useCallback(async (id: string) =>
    await (await fetch(`/api/user_answer/${id}`, {
      method: 'GET',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth]),
  create: useCallback(async (createUserAnswer: CreateUserAnswer) =>
    await (await fetch('/api/user_answer', {
      method: 'POST',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${auth.accessToken}`,
      },
      body: JSON.stringify(createUserAnswer),
    })).json(), [auth]),
  delete: useCallback(async (id: string) =>
    await (await fetch(`/api/user_answer/${id}`, {
      method: 'DELETE',
      headers: {
        Authorization: `Bearer ${auth.accessToken}`,
      },
    })).json(), [auth.accessToken]),
  update: useCallback(async (id: string, updateUserAnswer: UpdateUserAnswer) =>
    await (await fetch(`/api/user_answer/${id}`, {
      method: 'PUT',
      headers: {
        'Content-Type': 'application/json',
        Authorization: `Bearer ${auth.accessToken}`,
      },
      body: JSON.stringify(updateUserAnswer),
    })).json(), [auth.accessToken]),
})