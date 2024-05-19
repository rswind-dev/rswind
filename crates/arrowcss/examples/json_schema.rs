fn main() {
    #[cfg(feature = "json_schema")]
    {
        let path = std::env::var_os("SCHEMA_OUT_PATH").unwrap_or_else(|| "schema.json".into());
        let schema = schemars::schema_for!(arrowcss::config::ArrowConfig);
        let schema_str = serde_json::to_string_pretty(&schema).unwrap();
        let _ = std::fs::write(&path, schema_str);
        println!("Schema written to: {:?}", std::path::Path::new(&path).canonicalize().unwrap());
    }
}
