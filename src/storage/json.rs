use serde::Serialize;
use serde::de::DeserializeOwned;
use std::fs::{self, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

static TEMP_COUNTER: AtomicUsize = AtomicUsize::new(0);

fn sibling_path(path: &Path, suffix: &str) -> PathBuf {
    let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);
    let file_name = path.file_name().unwrap_or_default().to_string_lossy();

    path.with_file_name(format!(
        ".{file_name}.{}.{}.{suffix}",
        std::process::id(),
        id
    ))
}

pub fn save_json<T: Serialize>(path: &Path, data: &T) -> Result<(), Box<dyn std::error::Error>> {
    let json = serde_json::to_vec_pretty(data)?;
    let temp_path = sibling_path(path, "tmp");

    let result = (|| {
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&temp_path)?;

        file.write_all(&json)?;
        file.sync_all()?;
        drop(file);

        replace_file(&temp_path, path)?;

        Ok::<(), Box<dyn std::error::Error>>(())
    })();

    if result.is_err() {
        let _ = fs::remove_file(&temp_path);
    }

    result
}

#[cfg(not(windows))]
fn replace_file(temp: &Path, destination: &Path) -> std::io::Result<()> {
    fs::rename(temp, destination)
}

#[cfg(windows)]
fn replace_file(temp: &Path, destination: &Path) -> std::io::Result<()> {
    if !destination.try_exists()? {
        return fs::rename(temp, destination);
    }

    let backup = sibling_path(destination, "bak");

    fs::rename(destination, &backup)?;

    if let Err(error) = fs::rename(temp, destination) {
        return match fs::rename(temp, destination) {
            Ok(()) => Err(error),
            Err(rollback_error) => Err(std::io::Error::new(
                error.kind(),
                format!(
                    "Не удалось установить новый файл: {error}.\
                    Не удалось восстановить резервную копию: {rollback_error}"
                ),
            )),
        };
    }
    let _ = fs::remove_file(&backup);

    Ok(())
}

pub fn load_json<T: DeserializeOwned>(path: &Path) -> Result<T, Box<dyn std::error::Error>> {
    let json = fs::read(path)?;
    let data = serde_json::from_slice(&json)?;

    Ok(data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::{Value, json};

    fn temp_path(name: &str) -> PathBuf {
        let id = TEMP_COUNTER.fetch_add(1, Ordering::Relaxed);

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
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn save_json_overwrites_existing_file() {
        let path = temp_path("overwrite");

        save_json(&path, &json!({"value": 1})).unwrap();
        save_json(&path, &json!({"value": 2})).unwrap();

        let loaded: Value = load_json(&path).unwrap();

        assert_eq!(loaded, json!({"value": 2}));
        fs::remove_file(path).unwrap();
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

        fs::write(&path, "{ invalid json").unwrap();

        let result: Result<Value, _> = load_json(&path);

        assert!(result.is_err());
        fs::remove_file(path).unwrap();
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
        fs::remove_file(path).unwrap();
    }

    #[test]
    fn save_json_does_not_create_missing_parent_directory() {
        let path = temp_path("missing_parent").join("data.json");

        let result = save_json(&path, &json!({"value": 1}));

        assert!(result.is_err());
    }
}
