import { useCallback } from "react"
import { Auth } from "../hooks/useAuth"

export const useObfuscatedGameFormatConfigAPI = (auth: Auth) => ({
    index: useCallback(async (repository_id: string) =>
        await (await fetch(`/api/obfuscated_game_format_config?repository_id=${repository_id}`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${auth.accessToken}`,
            },
        })).json(), [auth]),
    get: useCallback(async (id: number) =>
        await (await fetch(`/api/obfuscated_game_format_config/${id}`, {
            method: 'GET',
            headers: {
                Authorization: `Bearer ${auth.accessToken}`,
            },
        })).json(), [auth]),
})