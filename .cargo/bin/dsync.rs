use std::collections::HashMap;
use std::path::PathBuf;

use dsync::{GenerationConfig, TableOptions};

pub fn main() {
    let dir = env!("CARGO_MANIFEST_DIR");

    println!("Running dsync (generating model code from `backend/schema.rs`)");

    let schema_file = PathBuf::from_iter([dir, "backend/schema.rs"]);
    let models_dir = PathBuf::from_iter([dir, "backend/models"]);

    ensure_schema_is_generated(&schema_file);

    dsync::generate_files(
        schema_file,
        models_dir,
        GenerationConfig {
            connection_type: "create_rust_app::Connection".to_string(),
            default_table_options: TableOptions::default().tsync().autogenerated_columns(vec![
                "id",
                "created_at",
                "updated_at",
            ]),
            table_options: HashMap::from([
                // plugin_storage
                ("attachment_blobs", TableOptions::default().ignore()),
                ("attachments", TableOptions::default().ignore()),
                // plugin_auth
                ("role_permissions", TableOptions::default().ignore()),
                ("user_permissions", TableOptions::default().ignore()),
                ("user_roles", TableOptions::default().ignore()),
                ("user_sessions", TableOptions::default().ignore()),
                ("users", TableOptions::default().ignore()),
            ]),
        },
    );
}

pub fn ensure_schema_is_generated(schema_file: &PathBuf) {
    // check that the diesel schema file is not empty
    let content = std::fs::read_to_string(schema_file).unwrap_or_default();

    if content.trim().is_empty() {
        println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        println!(" The diesel schema file is not generated yet.");
        println!(" If this is a new project, did you forget\n the `diesel database setup` step?");
        println!("=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=-=");
        panic!("`backend/schema.rs` is empty or not present");
    }
}
