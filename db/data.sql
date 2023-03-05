INSERT INTO repository(name, filename)
VALUES ('GitOxide', 'gitoxide'),
    ('Tree-sitter', 'tree-sitter') ON CONFLICT (name) DO
UPDATE
SET filename = EXCLUDED.filename;
INSERT INTO obfuscated_game_format_config(repository, filenames)
VALUES ('GitOxide', '*.rs'),
    ('Tree-sitter', '*.rs') ON CONFLICT (repository) DO
UPDATE
SET filenames = EXCLUDED.filenames;
INSERT INTO git_guessr_game_format_config(
        repository,
        filenames,
        lines_shown,
        allow_smaller_files
    )
VALUES ('GitOxide', '*.rs', 30, false),
    ('Tree-sitter', '*.rs', 30, false) ON CONFLICT (repository) DO
UPDATE
SET filenames = EXCLUDED.filenames,
    lines_shown = EXCLUDED.lines_shown,
    allow_smaller_files = EXCLUDED.allow_smaller_files;