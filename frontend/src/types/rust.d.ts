/* This file is generated and managed by tsync */

interface GitGuessrCorrectAnswer {
  id: number;
  answer: string;
  question_id: number;
  lobby_id: string;
  created_at: Date;
  updated_at: Date;
}

interface CreateGitGuessrCorrectAnswer {
  answer: string;
  question_id: number;
  lobby_id: string;
}

interface UpdateGitGuessrCorrectAnswer {
  answer?: string;
  question_id?: number;
  lobby_id?: string;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface GitGuessrGameFormatConfig {
  id: number;
  repository_id: string;
  filenames: string;
  lines_shown: number;
  allow_smaller_files: boolean;
}

interface CreateGitGuessrGameFormatConfig {
  repository_id: string;
  filenames: string;
  lines_shown: number;
  allow_smaller_files: boolean;
}

interface UpdateGitGuessrGameFormatConfig {
  repository_id?: string;
  filenames?: string;
  lines_shown?: number;
  allow_smaller_files?: boolean;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface GitGuessrQuestion {
  id: number;
  lobby_id: string;
  question_num: number;
  question_text: string;
  start_time?: Date;
  end_time?: Date;
  created_at: Date;
  updated_at: Date;
}

interface CreateGitGuessrQuestion {
  lobby_id: string;
  question_num: number;
  question_text: string;
  start_time?: Date;
  end_time?: Date;
}

interface UpdateGitGuessrQuestion {
  lobby_id?: string;
  question_num?: number;
  question_text?: string;
  start_time?: Date;
  end_time?: Date;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface GitGuessrUserAnswer {
  id: number;
  answer: string;
  question_id: number;
  lobby_participant_id: number;
  user_id: number;
  lobby_id: string;
  created_at: Date;
  updated_at: Date;
}

interface CreateGitGuessrUserAnswer {
  answer: string;
  question_id: number;
  lobby_participant_id: number;
  user_id: number;
  lobby_id: string;
}

interface UpdateGitGuessrUserAnswer {
  answer?: string;
  question_id?: number;
  lobby_participant_id?: number;
  user_id?: number;
  lobby_id?: string;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface Lobby {
  id: string;
  git_guessr_game_format_config_id?: number;
  obfuscated_game_format_config_id?: number;
  repository_id: string;
  start_time?: Date;
  end_time?: Date;
  created_at: Date;
  updated_at: Date;
}

interface CreateLobby {
  git_guessr_game_format_config_id?: number;
  obfuscated_game_format_config_id?: number;
  repository_id: string;
  start_time?: Date;
  end_time?: Date;
}

interface UpdateLobby {
  git_guessr_game_format_config_id?: number;
  obfuscated_game_format_config_id?: number;
  repository_id?: string;
  start_time?: Date;
  end_time?: Date;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface LobbyParticipant {
  id: number;
  user_id: number;
  lobby_id: string;
  created_at: Date;
  updated_at: Date;
}

interface CreateLobbyParticipant {
  user_id: number;
  lobby_id: string;
}

interface UpdateLobbyParticipant {
  user_id?: number;
  lobby_id?: string;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface ObfuscatedAnswerChoice {
  id: number;
  answer: string;
  question_id: number;
  lobby_id: string;
  created_at: Date;
  updated_at: Date;
}

interface CreateObfuscatedAnswerChoice {
  answer: string;
  question_id: number;
  lobby_id: string;
}

interface UpdateObfuscatedAnswerChoice {
  answer?: string;
  question_id?: number;
  lobby_id?: string;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface ObfuscatedCorrectAnswer {
  id: number;
  answer_choice_id: number;
  question_id: number;
  lobby_id: string;
  created_at: Date;
  updated_at: Date;
}

interface CreateObfuscatedCorrectAnswer {
  answer_choice_id: number;
  question_id: number;
  lobby_id: string;
}

interface UpdateObfuscatedCorrectAnswer {
  answer_choice_id?: number;
  question_id?: number;
  lobby_id?: string;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface ObfuscatedGameFormatConfig {
  id: number;
  repository_id: string;
  language: string;
  filenames: string;
}

interface CreateObfuscatedGameFormatConfig {
  repository_id: string;
  language: string;
  filenames: string;
}

interface UpdateObfuscatedGameFormatConfig {
  repository_id?: string;
  language?: string;
  filenames?: string;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface ObfuscatedQuestion {
  id: number;
  lobby_id: string;
  question_num: number;
  question_text: string;
  big_answer_choices: boolean;
  start_time?: Date;
  end_time?: Date;
  created_at: Date;
  updated_at: Date;
}

interface CreateObfuscatedQuestion {
  lobby_id: string;
  question_num: number;
  question_text: string;
  big_answer_choices: boolean;
  start_time?: Date;
  end_time?: Date;
}

interface UpdateObfuscatedQuestion {
  lobby_id?: string;
  question_num?: number;
  question_text?: string;
  big_answer_choices?: boolean;
  start_time?: Date;
  end_time?: Date;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface ObfuscatedUserAnswer {
  id: number;
  answer_choice_id: number;
  question_id: number;
  lobby_participant_id: number;
  user_id: number;
  lobby_id: string;
  created_at: Date;
  updated_at: Date;
}

interface CreateObfuscatedUserAnswer {
  answer_choice_id: number;
  question_id: number;
  lobby_participant_id: number;
  user_id: number;
  lobby_id: string;
}

interface UpdateObfuscatedUserAnswer {
  answer_choice_id?: number;
  question_id?: number;
  lobby_participant_id?: number;
  user_id?: number;
  lobby_id?: string;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface Repository {
  name: string;
  filename: string;
  url: string;
  description: string;
}

interface CreateRepository {
  name: string;
  filename: string;
  url: string;
  description: string;
}

interface UpdateRepository {
  filename?: string;
  url?: string;
  description?: string;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface Todo {
  id: number;
  text: string;
  created_at: Date;
  updated_at: Date;
}

interface CreateTodo {
  text: string;
}

interface UpdateTodo {
  text?: string;
  created_at?: Date;
  updated_at?: Date;
}

interface PaginationResult<T> {
  items: Array<T>;
  total_items: number;
  /** 0-based index */
  page: number;
  page_size: number;
  num_pages: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface Entry {
  is_directory: boolean;
  filename: string;
}

interface Directory {
  entries: Array<Entry>;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface FullGitGuessrQuestion {
  question: GitGuessrQuestion;
  correct_answer?: GitGuessrCorrectAnswer;
  user_answer?: GitGuessrUserAnswer;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface LobbyFilters {
  repository_id?: string;
}

interface FullObfuscatedQuestions {
  question: ObfuscatedQuestion;
  answer_choices: Array<ObfuscatedAnswerChoice>;
  user_answer?: ObfuscatedUserAnswer;
}

interface FullGitGuessrQuestions {
  question: GitGuessrQuestion;
  user_answer?: GitGuessrUserAnswer;
}

interface FullLobby {
  lobby: Lobby;
  full_obfuscated_questions: Array<FullObfuscatedQuestions>;
  full_git_guessr_questions: Array<FullGitGuessrQuestions>;
}

interface StartLobby {
  start_time?: Date;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface LobbyParticipantFilters {
  lobby_id?: string;
  user_id?: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface FullObfuscatedQuestion {
  question: ObfuscatedQuestion;
  answer_choices: Array<ObfuscatedAnswerChoice>;
  correct_answer?: ObfuscatedCorrectAnswer;
  user_answer?: ObfuscatedUserAnswer;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}

interface PaginationParams {
  page: number;
  page_size: number;
}
