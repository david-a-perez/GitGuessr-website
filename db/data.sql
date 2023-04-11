INSERT INTO repository(name, filename, url, description)
VALUES ('GitOxide', '../GitGuessr-data/gitoxide', 'https://github.com/Byron/gitoxide', 'An idiomatic, lean, fast & safe pure Rust implementation of Git'),
    ('Tree-sitter', '../GitGuessr-data/tree-sitter', 'https://github.com/tree-sitter/tree-sitter', 'An incremental parsing system for programming tools') ON CONFLICT (name) DO
UPDATE
SET filename = EXCLUDED.filename;
INSERT INTO obfuscated_game_format_config(repository_id, filenames)
VALUES ('GitOxide', '.rs$'),
    ('Tree-sitter', '.rs$') ON CONFLICT (repository_id) DO
UPDATE
SET filenames = EXCLUDED.filenames;
INSERT INTO git_guessr_game_format_config(
        repository_id,
        filenames,
        lines_shown,
        allow_smaller_files
    )
VALUES ('GitOxide', '.rs$', 30, false),
    ('Tree-sitter', '.rs$', 30, false) ON CONFLICT (repository_id) DO
UPDATE
SET filenames = EXCLUDED.filenames,
    lines_shown = EXCLUDED.lines_shown,
    allow_smaller_files = EXCLUDED.allow_smaller_files;