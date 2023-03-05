// @generated automatically by Diesel CLI.

diesel::table! {
    answer_choice (question_id, answer) {
        question_id -> Int4,
        answer -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    correct_answer (question_id) {
        question_id -> Int4,
        answer -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    git_guessr_game_format_config (repository) {
        repository -> Text,
        filenames -> Text,
        lines_shown -> Int4,
        allow_smaller_files -> Bool,
    }
}

diesel::table! {
    lobby (id) {
        id -> Text,
        repository -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    obfuscated_game_format_config (repository) {
        repository -> Text,
        filenames -> Text,
    }
}

diesel::table! {
    question (id) {
        id -> Int4,
        lobby_id -> Text,
        question_text -> Text,
        created_at -> Timestamptz,
        updated_at -> Timestamptz,
    }
}

diesel::table! {
    repository (name) {
        name -> Text,
        filename -> Text,
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
    user_answer (user_id, question_id) {
        user_id -> Int4,
        question_id -> Int4,
        answer -> Text,
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

diesel::joinable!(answer_choice -> question (question_id));
diesel::joinable!(correct_answer -> question (question_id));
diesel::joinable!(git_guessr_game_format_config -> repository (repository));
diesel::joinable!(lobby -> repository (repository));
diesel::joinable!(obfuscated_game_format_config -> repository (repository));
diesel::joinable!(question -> lobby (lobby_id));
diesel::joinable!(user_answer -> question (question_id));
diesel::joinable!(user_answer -> users (user_id));
diesel::joinable!(user_permissions -> users (user_id));
diesel::joinable!(user_roles -> users (user_id));
diesel::joinable!(user_sessions -> users (user_id));

diesel::allow_tables_to_appear_in_same_query!(
    answer_choice,
    correct_answer,
    git_guessr_game_format_config,
    lobby,
    obfuscated_game_format_config,
    question,
    repository,
    role_permissions,
    todos,
    user_answer,
    user_permissions,
    user_roles,
    user_sessions,
    users,
);
