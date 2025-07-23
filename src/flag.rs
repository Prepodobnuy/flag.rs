use std::collections::HashMap;
use std::env;
use std::str::FromStr;

pub struct Flag {
    descriptions: HashMap<String, String>,
    values: HashMap<String, String>,
    args: Vec<String>,
}

impl Flag {
    pub fn new() -> Self {
        let descriptions = HashMap::new();
        let mut values = HashMap::new();

        let args = env::args();

        let mut f = None;

        for arg in args {
            let is_flag = arg.starts_with("-");
            let has_val = arg.contains("=");

            if is_flag && has_val {
                let mut parts = arg.splitn(2, "=");
                if let Some(flag) = parts.next() {
                    let val = parts.next().unwrap_or("");
                    values.insert(flag.to_owned(), val.to_owned());
                }
                continue;
            }

            if is_flag {
                f = Some(arg.to_owned());
                continue;
            }

            if let Some(flag) = f.take() {
                values.insert(flag, arg.to_owned());
            }
        }

        Flag {
            descriptions,
            values,
            args: env::args().collect(),
        }
    }

    pub fn has(&mut self, key: &str, desc: &str) -> bool {
        self.descriptions.insert(key.to_string(), desc.to_string());
        self.args.contains(&key.to_string())
    }

    pub fn get_str(&mut self, key: &str, desc: &str) -> Option<String> {
        self.descriptions.insert(key.to_string(), desc.to_string());
        self.values.get(key).map(|s| s.to_string())
    }

    pub fn fget_str(&mut self, key: &str, fallback: &str, desc: &str) -> String {
        self.descriptions.insert(key.to_string(), desc.to_string());
        self.values
            .get(key)
            .map(|s| s.to_string())
            .unwrap_or(fallback.to_string())
    }

    pub fn get<T: FromStr>(&mut self, key: &str, desc: &str) -> Option<T> {
        self.descriptions.insert(key.to_string(), desc.to_string());

        if let Some(v) = self.values.get(key) {
            v.parse().ok()
        } else {
            None
        }
    }

    pub fn fget<T: FromStr>(&mut self, key: &str, fallback: T, desc: &str) -> T {
        self.descriptions.insert(key.to_string(), desc.to_string());

        if let Some(v) = self.values.get(key) {
            v.parse().ok().unwrap_or(fallback)
        } else {
            fallback
        }
    }
}
