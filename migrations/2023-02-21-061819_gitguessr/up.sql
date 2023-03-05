-- Your SQL goes here
CREATE TABLE repository (
    name TEXT PRIMARY KEY,
    filename TEXT NOT NULL
);
CREATE TABLE git_guessr_game_format_config (
    repository TEXT PRIMARY KEY REFERENCES repository(name),
    filenames TEXT NOT NULL,
    lines_shown INTEGER NOT NULL,
    allow_smaller_files BOOLEAN NOT NULL
);
CREATE TABLE obfuscated_game_format_config (
    repository TEXT PRIMARY KEY REFERENCES repository(name),
    filenames TEXT NOT NULL
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
    -- id SERIAL PRIMARY KEY,
    repository TEXT NOT NULL REFERENCES repository(name),
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT manage_updated_at('lobby');
CREATE TABLE question (
    id SERIAL PRIMARY KEY,
    lobby_id TEXT NOT NULL REFERENCES lobby(id),
    question_text TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT manage_updated_at('question');
CREATE TABLE correct_answer (
    question_id SERIAL PRIMARY KEY REFERENCES question(id),
    answer TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
);
SELECT manage_updated_at('correct_answer');
CREATE TABLE user_answer (
    user_id SERIAL NOT NULL REFERENCES users(id),
    question_id SERIAL NOT NULL REFERENCES question(id),
    answer TEXT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, question_id)
);
SELECT manage_updated_at('user_answer');