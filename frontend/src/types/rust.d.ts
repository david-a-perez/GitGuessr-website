/* This file is generated and managed by tsync */

interface CorrectAnswer {
  question_id: number
  answer: string
  created_at: Date
  updated_at: Date
}

interface CreateCorrectAnswer {
  question_id: number
  answer: string
}

interface UpdateCorrectAnswer {
  answer?: string
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
  repository: string
  filenames: string
  lines_shown: number
  allow_smaller_files: boolean
}

interface CreateGitGuessrGameFormatConfig {
  repository: string
  filenames: string
  lines_shown: number
  allow_smaller_files: boolean
}

interface UpdateGitGuessrGameFormatConfig {
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
  repository: string
  created_at: Date
  updated_at: Date
}

interface CreateLobby {
  repository: string
}

interface UpdateLobby {
  repository?: string
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
  repository: string
  filenames: string
}

interface CreateObfuscatedGameFormatConfig {
  repository: string
  filenames: string
}

interface UpdateObfuscatedGameFormatConfig {
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
  question_text: string
  created_at: Date
  updated_at: Date
}

interface CreateQuestion {
  lobby_id: string
  question_text: string
}

interface UpdateQuestion {
  lobby_id?: string
  question_text?: string
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
  user_id: number
  question_id: number
  answer: string
  created_at: Date
  updated_at: Date
}

interface CreateUserAnswer {
  user_id: number
  question_id: number
  answer: string
}

interface UpdateUserAnswer {
  answer?: string
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

interface PaginationParams {
  page: number
  page_size: number
}

interface PaginationParams {
  page: number
  page_size: number
}
