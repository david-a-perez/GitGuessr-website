-- Your SQL goes here
CREATE TABLE repository (
    name TEXT PRIMARY KEY,
    filename TEXT NOT NULL
);
CREATE TABLE git_guessr_game_format_config (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    repository_id TEXT NOT NULL REFERENCES repository(name) ON DELETE CASCADE,
    filenames TEXT NOT NULL,
    lines_shown INTEGER NOT NULL,
    allow_smaller_files BOOLEAN NOT NULL,
    UNIQUE (repository_id, id),
    UNIQUE (repository_id)
);
CREATE TABLE obfuscated_game_format_config (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    repository_id TEXT NOT NULL REFERENCES repository(name) ON DELETE CASCADE,
    filenames TEXT NOT NULL,
    UNIQUE (repository_id, id),
    UNIQUE (repository_id)
);
CREATE EXTENSION pgcrypto;

CREATE FUNCTION generate_uid(size INT) RETURNS TEXT AS $$
DECLARE
  characters TEXT := 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
  bytes BYTEA := gen_random_bytes(size);
  l INT := length(characters);
  i INT := 0;
  output TEXT := '';
BEGIN
  WHILE i < size LOOP
    output := output || substr(characters, get_byte(bytes, i) % l + 1, 1);
    i := i + 1;
  END LOOP;
  RETURN output;
END;
$$ LANGUAGE plpgsql VOLATILE;

CREATE TABLE lobby (
    id TEXT PRIMARY KEY DEFAULT generate_uid(6),
    git_guessr_game_format_config_id INTEGER,
    obfuscated_game_format_config_id INTEGER,
    repository_id TEXT NOT NULL REFERENCES repository(name) ON DELETE CASCADE,

    start_time TIMESTAMPTZ, --TODO: do we care when the game is started or only when questions start
    end_time TIMESTAMPTZ, -- TODO: do we care when the game is ended or only when questions end
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (repository_id, git_guessr_game_format_config_id) REFERENCES git_guessr_game_format_config (repository_id, id) ON DELETE CASCADE,
    FOREIGN KEY (repository_id, obfuscated_game_format_config_id) REFERENCES obfuscated_game_format_config (repository_id, id) ON DELETE CASCADE,
    CONSTRAINT chk_only_one_is_not_null CHECK (num_nonnulls(git_guessr_game_format_config_id, obfuscated_game_format_config_id) = 1)
);
SELECT manage_updated_at('lobby');
CREATE TABLE question (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    lobby_id TEXT NOT NULL REFERENCES lobby(id) ON DELETE CASCADE,
    question_num INTEGER NOT NULL,
    question_text TEXT NOT NULL,
    start_time TIMESTAMPTZ, -- TODO: if start_time has not started, set question_text to "Pending"
    end_time TIMESTAMPTZ, -- TODO: if end_time has passed, don't allow submission
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (lobby_id, id),
    UNIQUE (lobby_id, question_num)
);
SELECT manage_updated_at('question');
CREATE TABLE answer_choice (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    answer TEXT NOT NULL,
    question_id INTEGER NOT NULL,
    lobby_id TEXT NOT NULL REFERENCES lobby(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lobby_id, question_id) REFERENCES question (lobby_id, id) ON DELETE CASCADE,
    UNIQUE (lobby_id, question_id, id),
    UNIQUE (question_id, answer)
);
SELECT manage_updated_at('answer_choice');
CREATE TABLE correct_answer (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    answer_choice_id INTEGER NOT NULL,
    question_id INTEGER NOT NULL,
    lobby_id TEXT NOT NULL REFERENCES lobby(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lobby_id, question_id) REFERENCES question (lobby_id, id) ON DELETE CASCADE,
    FOREIGN KEY (lobby_id, question_id, answer_choice_id) REFERENCES answer_choice (lobby_id, question_id, id) ON DELETE CASCADE,
    UNIQUE (lobby_id, question_id, answer_choice_id, id),
    UNIQUE (question_id)
);
SELECT manage_updated_at('correct_answer');
CREATE TABLE lobby_participant (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lobby_id TEXT NOT NULL REFERENCES lobby(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    UNIQUE (lobby_id, user_id, id),
    UNIQUE (lobby_id, user_id)
);
SELECT manage_updated_at('lobby_participant');
CREATE TABLE user_answer (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    answer_choice_id INTEGER NOT NULL,
    question_id INTEGER NOT NULL,
    lobby_participant_id INTEGER NOT NULL,
    user_id INTEGER NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    lobby_id TEXT NOT NULL REFERENCES lobby(id) ON DELETE CASCADE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (lobby_id, user_id, lobby_participant_id) REFERENCES lobby_participant (lobby_id, user_id, id) ON DELETE CASCADE,
    FOREIGN KEY (lobby_id, question_id) REFERENCES question (lobby_id, id) ON DELETE CASCADE,
    FOREIGN KEY (lobby_id, question_id, answer_choice_id) REFERENCES answer_choice (lobby_id, question_id, id) ON DELETE CASCADE,
    UNIQUE (lobby_id, user_id, lobby_participant_id, question_id, answer_choice_id, id),
    UNIQUE (user_id, question_id)
);
SELECT manage_updated_at('user_answer');
