-- This file should undo anything in `up.sql`
DROP TABLE git_guessr_user_answer;
DROP TABLE git_guessr_correct_answer;
DROP TABLE git_guessr_question;
DROP TABLE obfuscated_user_answer;
DROP TABLE obfuscated_correct_answer;
DROP TABLE obfuscated_answer_choice;
DROP TABLE obfuscated_question;
DROP TABLE lobby_participant;
DROP TABLE lobby;
DROP TABLE git_guessr_game_format_config;
DROP TABLE obfuscated_game_format_config;
DROP TABLE repository;

DROP EXTENSION pgcrypto;
DROP FUNCTION generate_uid;
