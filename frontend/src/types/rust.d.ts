/* This file is generated and managed by tsync */

interface AnswerChoice {
  id: number
  answer: string
  question_id: number
  lobby_id: string
  created_at: Date
  updated_at: Date
}

interface CreateAnswerChoice {
  answer: string
  question_id: number
  lobby_id: string
}

interface UpdateAnswerChoice {
  answer?: string
  question_id?: number
  lobby_id?: string
  created_at?: Date
  updated_at?: Date
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface CorrectAnswer {
  id: number
  answer_choice_id: number
  question_id: number
  lobby_id: string
  created_at: Date
  updated_at: Date
}

interface CreateCorrectAnswer {
  answer_choice_id: number
  question_id: number
  lobby_id: string
}

interface UpdateCorrectAnswer {
  answer_choice_id?: number
  question_id?: number
  lobby_id?: string
  created_at?: Date
  updated_at?: Date
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface GitGuessrGameFormatConfig {
  id: number
  repository_id: string
  filenames: string
  lines_shown: number
  allow_smaller_files: boolean
}

interface CreateGitGuessrGameFormatConfig {
  repository_id: string
  filenames: string
  lines_shown: number
  allow_smaller_files: boolean
}

interface UpdateGitGuessrGameFormatConfig {
  repository_id?: string
  filenames?: string
  lines_shown?: number
  allow_smaller_files?: boolean
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface Lobby {
  id: string
  git_guessr_game_format_config_id?: number
  obfuscated_game_format_config_id?: number
  repository_id: string
  start_time?: Date
  end_time?: Date
  created_at: Date
  updated_at: Date
}

interface CreateLobby {
  git_guessr_game_format_config_id?: number
  obfuscated_game_format_config_id?: number
  repository_id: string
  start_time?: Date
  end_time?: Date
}

interface UpdateLobby {
  git_guessr_game_format_config_id?: number
  obfuscated_game_format_config_id?: number
  repository_id?: string
  start_time?: Date
  end_time?: Date
  created_at?: Date
  updated_at?: Date
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface LobbyParticipant {
  id: number
  user_id: number
  lobby_id: string
  created_at: Date
  updated_at: Date
}

interface CreateLobbyParticipant {
  user_id: number
  lobby_id: string
}

interface UpdateLobbyParticipant {
  user_id?: number
  lobby_id?: string
  created_at?: Date
  updated_at?: Date
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface ObfuscatedGameFormatConfig {
  id: number
  repository_id: string
  filenames: string
}

interface CreateObfuscatedGameFormatConfig {
  repository_id: string
  filenames: string
}

interface UpdateObfuscatedGameFormatConfig {
  repository_id?: string
  filenames?: string
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface Question {
  id: number
  lobby_id: string
  question_num: number
  question_text: string
  start_time?: Date
  end_time?: Date
  created_at: Date
  updated_at: Date
}

interface CreateQuestion {
  lobby_id: string
  question_num: number
  question_text: string
  start_time?: Date
  end_time?: Date
}

interface UpdateQuestion {
  lobby_id?: string
  question_num?: number
  question_text?: string
  start_time?: Date
  end_time?: Date
  created_at?: Date
  updated_at?: Date
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface Repository {
  name: string
  filename: string
}

interface CreateRepository {
  name: string
  filename: string
}

interface UpdateRepository {
  filename?: string
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface Todo {
  id: number
  text: string
  created_at: Date
  updated_at: Date
}

interface CreateTodo {
  text: string
}

interface UpdateTodo {
  text?: string
  created_at?: Date
  updated_at?: Date
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface UserAnswer {
  id: number
  answer_choice_id: number
  question_id: number
  lobby_participant_id: number
  user_id: number
  lobby_id: string
  created_at: Date
  updated_at: Date
}

interface CreateUserAnswer {
  answer_choice_id: number
  question_id: number
  lobby_participant_id: number
  user_id: number
  lobby_id: string
}

interface UpdateUserAnswer {
  answer_choice_id?: number
  question_id?: number
  lobby_participant_id?: number
  user_id?: number
  lobby_id?: string
  created_at?: Date
  updated_at?: Date
}

interface PaginationResult<T> {
  items: Array<T>
  total_items: number
  /** 0-based index */
  page: number
  page_size: number
  num_pages: number
}

interface PaginationParams {
  page: number
  page_size: number
}

interface PaginationParams {
  page: number
  page_size: number
}

interface PaginationParams {
  page: number
  page_size: number
}

interface LobbyFilters {
  repository_id?: string
}

interface FullLobby {
  lobby: Lobby
}

interface StartLobby {
  start_time?: Date
}

interface PaginationParams {
  page: number
  page_size: number
}

interface LobbyParticipantFilters {
  lobby_id?: string
  user_id?: number
}

interface PaginationParams {
  page: number
  page_size: number
}

interface FullQuestion {
  question: Question
  answer_choices: Array<AnswerChoice>
  correct_answer?: CorrectAnswer
  user_answer?: UserAnswer
}

interface PaginationParams {
  page: number
  page_size: number
}

interface PaginationParams {
  page: number
  page_size: number
}

interface PaginationParams {
  page: number
  page_size: number
}
