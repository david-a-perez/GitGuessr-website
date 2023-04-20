INSERT INTO repository(name, filename, url, description)
VALUES ('GitOxide', '../GitGuessr-data/gitoxide', 'https://github.com/Byron/gitoxide', 'An idiomatic, lean, fast & safe pure Rust implementation of Git'),
    ('Tree-sitter', '../GitGuessr-data/tree-sitter', 'https://github.com/tree-sitter/tree-sitter', 'An incremental parsing system for programming tools'),
    ('GitGuessr', '.', 'https://github.com/david-a-perez/GitGuessr-website', 'Tool to get familiar with a codebase through fun games'),
    ('Containerized Workshops', '../GitGuessr-data/containerized-workshops', 'https://github.com/david-a-perez/containerized-workshops', 'Tool for workshop delivery through cloud containers'),
    ('Linux', '../GitGuessr-data/linux', 'https://www.kernel.org/', 'The Linux kernel source code') ON CONFLICT (name) DO
UPDATE
SET filename = EXCLUDED.filename;
INSERT INTO obfuscated_game_format_config(repository_id, language, filenames)
VALUES ('GitOxide', 'rust', '\.rs$'),
    ('Tree-sitter', 'rust', '\.rs$'),
    ('GitGuessr', 'rust', '\.rs$'),
    ('Containerized Workshops', 'python', 'ContainerizedWorkshops/[a-z]+\.py$'),
    ('Linux', 'c', '\.c$') ON CONFLICT (repository_id) DO
UPDATE
SET filenames = EXCLUDED.filenames;
INSERT INTO git_guessr_game_format_config(
        repository_id,
        filenames,
        lines_shown,
        allow_smaller_files
    )
VALUES ('GitOxide', '\.rs$', 30, false),
    ('Tree-sitter', '\.rs$', 30, false),
    ('GitGuessr', '\.rs$', 30, false),
    ('Containerized Workshops', '\[^_].py$', 30, false),
    ('Linux', '\.c$', 30, false) ON CONFLICT (repository_id) DO
UPDATE
SET filenames = EXCLUDED.filenames,
    lines_shown = EXCLUDED.lines_shown,
    allow_smaller_files = EXCLUDED.allow_smaller_files;
