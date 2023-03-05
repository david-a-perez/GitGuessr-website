-- This file should undo anything in `up.sql`
DROP TABLE user_answer;
DROP TABLE correct_answer;
DROP TABLE question;
DROP TABLE lobby;
DROP TABLE git_guessr_game_format_config;
DROP TABLE obfuscated_game_format_config;
DROP TABLE repository;

DROP EXTENSION pgcrypto;
DROP FUNCTION generate_uid;
