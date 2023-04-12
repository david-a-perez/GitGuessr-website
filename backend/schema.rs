// @generated automatically by Diesel CLI.

diesel::table! {
    git_guessr_correct_answer (id) {
        id -> Int4,
        answer -> Text,
        question_id -> Int4,
        lobby_id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    git_guessr_game_format_config (id) {
        id -> Int4,
        repository_id -> Text,
        filenames -> Text,
        lines_shown -> Int4,
        allow_smaller_files -> Bool,
    }
}

diesel::table! {
    git_guessr_question (id) {
        id -> Int4,
        lobby_id -> Text,
        question_num -> Int4,
        question_text -> Text,
        start_time -> Nullable<Timestamptz>,
        end_time -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    git_guessr_user_answer (id) {
        id -> Int4,
        answer -> Text,
        question_id -> Int4,
        lobby_participant_id -> Int4,
        user_id -> Int4,
        lobby_id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    lobby (id) {
        id -> Text,
        git_guessr_game_format_config_id -> Nullable<Int4>,
        obfuscated_game_format_config_id -> Nullable<Int4>,
        repository_id -> Text,
        start_time -> Nullable<Timestamptz>,
        end_time -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    lobby_participant (id) {
        id -> Int4,
        user_id -> Int4,
        lobby_id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    obfuscated_answer_choice (id) {
        id -> Int4,
        answer -> Text,
        question_id -> Int4,
        lobby_id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    obfuscated_correct_answer (id) {
        id -> Int4,
        answer_choice_id -> Int4,
        question_id -> Int4,
        lobby_id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    obfuscated_game_format_config (id) {
        id -> Int4,
        repository_id -> Text,
        language -> Text,
        filenames -> Text,
    }
}

diesel::table! {
    obfuscated_question (id) {
        id -> Int4,
        lobby_id -> Text,
        question_num -> Int4,
        question_text -> Text,
        big_answer_choices -> Bool,
        start_time -> Nullable<Timestamptz>,
        end_time -> Nullable<Timestamptz>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    obfuscated_user_answer (id) {
        id -> Int4,
        answer_choice_id -> Int4,
        question_id -> Int4,
        lobby_participant_id -> Int4,
        user_id -> Int4,
        lobby_id -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    repository (name) {
        name -> Text,
        filename -> Text,
        url -> Text,
        description -> Text,
    }
}

diesel::table! {
    role_permissions (role, permission) {
        role -> Text,
        permission -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    todos (id) {
        id -> Int4,
        text -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    user_permissions (user_id, permission) {
        user_id -> Int4,
        permission -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_roles (user_id, role) {
        user_id -> Int4,
        role -> Text,
        created_at -> Timestamptz,
    }
}

diesel::table! {
    user_sessions (id) {
        id -> Int4,
        user_id -> Int4,
        refresh_token -> Text,
        device -> Nullable<Text>,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    users (id) {
        id -> Int4,
        email -> Text,
        hash_password -> Text,
        activated -> Bool,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::joinable!(git_guessr_correct_answer -> git_guessr_question (question_id));
diesel::joinable!(git_guessr_correct_answer -> lobby (lobby_id));
diesel::joinable!(git_guessr_game_format_config -> repository (repository_id));
diesel::joinable!(git_guessr_question -> lobby (lobby_id));
diesel::joinable!(git_guessr_user_answer -> git_guessr_question (question_id));
diesel::joinable!(git_guessr_user_answer -> lobby (lobby_id));
diesel::joinable!(git_guessr_user_answer -> lobby_participant (lobby_participant_id));
diesel::joinable!(git_guessr_user_answer -> users (user_id));
diesel::joinable!(lobby -> git_guessr_game_format_config (git_guessr_game_format_config_id));
diesel::joinable!(lobby -> obfuscated_game_format_config (obfuscated_game_format_config_id));
diesel::joinable!(lobby -> repository (repository_id));
diesel::joinable!(lobby_participant -> lobby (lobby_id));
diesel::joinable!(lobby_participant -> users (user_id));
diesel::joinable!(obfuscated_answer_choice -> lobby (lobby_id));
diesel::joinable!(obfuscated_answer_choice -> obfuscated_question (question_id));
diesel::joinable!(obfuscated_correct_answer -> lobby (lobby_id));
diesel::joinable!(obfuscated_correct_answer -> obfuscated_answer_choice (answer_choice_id));
diesel::joinable!(obfuscated_correct_answer -> obfuscated_question (question_id));
diesel::joinable!(obfuscated_game_format_config -> repository (repository_id));
diesel::joinable!(obfuscated_question -> lobby (lobby_id));
diesel::joinable!(obfuscated_user_answer -> lobby (lobby_id));
diesel::joinable!(obfuscated_user_answer -> lobby_participant (lobby_participant_id));
diesel::joinable!(obfuscated_user_answer -> obfuscated_answer_choice (answer_choice_id));
diesel::joinable!(obfuscated_user_answer -> obfuscated_question (question_id));
diesel::joinable!(obfuscated_user_answer -> users (user_id));
diesel::joinable!(user_permissions -> users (user_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    git_guessr_correct_answer,
    git_guessr_game_format_config,
    git_guessr_question,
    git_guessr_user_answer,
    lobby,
    lobby_participant,
    obfuscated_answer_choice,
    obfuscated_correct_answer,
    obfuscated_game_format_config,
    obfuscated_question,
    obfuscated_user_answer,
    repository,
    role_permissions,
    todos,
    user_permissions,
    user_roles,
    user_sessions,
    users,
);
