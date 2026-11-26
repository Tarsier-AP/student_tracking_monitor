use serde::de::DeserializeOwned;
use serde_json::Serializer;
use std::path::Path;

pub fn save_json<T: Seroalize>(path: &Path, data: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_vec_pretty(data)?;
    std::fs::write(path, json)?;

    Ok(())
}

pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let json = std::fs::read(path)?;
    let data = serde_json::from_slice(&json)?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};
    use std::sync::atomic::{AtomicUsize, Ordering};

    static COUNTER: AtomicUsize = AtomicUsize::new(0);

    fn temp_path(name: &str) -> std::path::PathBuf {
        let id = COUNTER.fetch_add(1, Ordering::Relaxed);

        std::env::temp_dir().join(format!(
            "classroom_manager_{name}_{}_{}.json",
            std::process::id(),
            id
        ))
    }

    #[test]
    fn save_and_load_json() {
        let path = temp_path("roundtrip");
        let original = json!({
            "name": "PC-01",
            "online": true,
            "students": [1, 2, 3]
        });

        save_json(&path, &original).unwrap();

        let loaded: Value = load_json(&path).unwrap();

        assert_eq!(loaded, original);
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn save_json_overwrites_existing_file() {
        let path = temp_path("overwrite");

        save_json(&path, &json!({"value": 1})).unwrap();
        save_json(&path, &json!({"value": 2})).unwrap();

        let loaded: Value = load_json(&path).unwrap();

        assert_eq!(loaded, json!({"value": 2}));
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn load_json_fails_if_file_does_not_exist() {
        let path = temp_path("missing");

        let result: Result<Value, _> = load_json(&path);

        assert!(result.is_err());
    }

    #[test]
    fn load_json_fails_on_invalid_json() {
        let path = temp_path("invalid");

        std::fs::write(&path, "{ invalid json").unwrap();

        let result: Result<Value, _> = load_json(&path);

        assert!(result.is_err());
        std::fs::remove_file(path).unwrap();
    }

    #[test]
    fn save_json_preserves_nested_data() {
        let path = temp_path("nested");

        let original = json!({
            "student": {
                "id": 42,
                "group": "Group 1",
                "repositories": [
                    "https://github.com/example/project"
                ]
            }
        });

        save_json(&path, &original).unwrap();

        let loaded: Value = load_json(&path).unwrap();

        assert_eq!(loaded, original);
        std::fs::remove_file(path).unwrap();
    }
}
