fn main() {
    #[cfg(feature = "json_schema")]
    {
        let path = std::env::var_os("SCHEMA_OUT_PATH").unwrap_or("schema.json".into());
        generate_schema(path);
    }

    #[cfg(not(feature = "json_schema"))]
    {
        panic!("Feature 'json_schema' is not enabled.");
    }
}

#[cfg(feature = "json_schema")]
fn generate_schema(path: impl AsRef<std::path::Path>) {
    let schema = schemars::schema_for!(rswind_core::config::GeneratorConfig);
    let schema_str = serde_json::to_string_pretty(&schema).unwrap();
    let _ = std::fs::write(&path, schema_str);
    println!("Schema written to: {:?}", Path::new(&path).canonicalize().unwrap());
}

#[cfg(test)]
#[cfg(feature = "json_schema")]
mod tests {
    #[test]
    fn test_main() {
        let file = tempfile::NamedTempFile::new();
        crate::generate_schema(file.path());
    }
}
