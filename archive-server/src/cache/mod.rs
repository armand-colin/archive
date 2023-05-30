use serde::{de::DeserializeOwned, Serialize};
use std::{
    io::Write,
    marker::PhantomData,
    path::{Path, PathBuf},
};

pub struct Cache<T: Serialize + DeserializeOwned> {
    path: PathBuf,
    _mark: PhantomData<T>,
}

impl<T: Serialize + DeserializeOwned> Cache<T> {
    pub fn new(name: String) -> Self {
        Self {
            path: Path::new("cache").join(format!("{}.json", name)),
            _mark: PhantomData,
        }
    }

    pub fn get(&self) -> Vec<T> {
        let Ok(file) = std::fs::File::open(&self.path) else {
            return vec![]
        };

        let string = std::io::read_to_string(file).unwrap();
        let result: Vec<T> = serde_json::from_str(&string).unwrap();
        result
    }

    pub fn save(&self, values: Vec<T>) {
        let mut file = std::fs::File::create(&self.path).unwrap();
        let string = serde_json::to_string(&values).unwrap();
        file.write_all(string.as_bytes()).unwrap();
    }
}

impl Cache<()> {
    pub fn init() {
        std::fs::create_dir_all("cache").unwrap();
    }
}
