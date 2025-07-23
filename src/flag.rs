use std::collections::HashMap;
use std::env;
use std::str::FromStr;

pub struct Flag {
    descriptions: HashMap<String, (String, Type)>,
    values: HashMap<String, String>,
    args: Vec<String>,
    help_message_prefix: Option<String>,
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
            help_message_prefix: None,
        }
    }

    pub fn help(&self) -> String {
        let usage = format!(
            "usage: {} {}",
            env!("CARGO_PKG_NAME"),
            self.descriptions
                .iter()
                .map(|f| format!("[{}, {}]", f.0, f.1.1.to_string()))
                .collect::<Vec<String>>()
                .join(" ")
        );

        let options: String = format!(
            "options:\n{}",
            self.descriptions
                .iter()
                .map(|(k, v)| {
                    format!(
                        "  {}{} {}",
                        k,
                        {
                            let len = 20;
                            if k.len() > len {
                                let mut r = " ".repeat(len);
                                r.extend([' ', ' ', '\n'].iter());
                                r
                            } else {
                                " ".repeat(len - k.len())
                            }
                        },
                        v.0
                    )
                })
                .collect::<Vec<String>>()
                .join("\n")
        );

        [usage, options].join("\n")
    }

    fn add_desc(&mut self, key: &str, desc: &str, _type: Type) {
        self.descriptions
            .insert(key.to_string(), (desc.to_string(), _type));
    }

    pub fn set_help_message_prefix(&mut self, msg: &str) {
        self.help_message_prefix = Some(msg.to_string())
    }

    pub fn get_bool(&mut self, key: &str, desc: &str) -> bool {
        self.add_desc(key, desc, Type::Bool);
        self.args.contains(&key.to_string())
    }

    pub fn get_str(&mut self, key: &str, desc: &str) -> Option<String> {
        self.add_desc(key, desc, Type::String);
        self.values.get(key).map(|s| s.to_string())
    }

    pub fn fget_str(&mut self, key: &str, fallback: &str, desc: &str) -> String {
        self.add_desc(key, desc, Type::String);
        self.values
            .get(key)
            .map(|s| s.to_string())
            .unwrap_or(fallback.to_string())
    }

    pub fn get_i32(&mut self, key: &str, desc: &str) -> Option<i32> {
        self.add_desc(key, desc, Type::Number);
        self.values.get(key).and_then(|i| i.parse().ok())
    }

    pub fn fget_i32(&mut self, key: &str, fallback: i32, desc: &str) -> i32 {
        self.add_desc(key, desc, Type::Number);
        self.values
            .get(key)
            .and_then(|i| i.parse().ok())
            .unwrap_or(fallback)
    }

    pub fn get_u32(&mut self, key: &str, desc: &str) -> Option<u32> {
        self.add_desc(key, desc, Type::Unsigned);
        self.values.get(key).and_then(|u| u.parse().ok())
    }

    pub fn fget_u32(&mut self, key: &str, fallback: u32, desc: &str) -> u32 {
        self.add_desc(key, desc, Type::Unsigned);
        self.values
            .get(key)
            .and_then(|u| u.parse().ok())
            .unwrap_or(fallback)
    }

    pub fn get_f32(&mut self, key: &str, desc: &str) -> Option<f32> {
        self.add_desc(key, desc, Type::Double);
        self.values.get(key).and_then(|f| f.parse().ok())
    }

    pub fn fget_f32(&mut self, key: &str, fallback: f32, desc: &str) -> f32 {
        self.add_desc(key, desc, Type::Double);
        self.values
            .get(key)
            .and_then(|f| f.parse().ok())
            .unwrap_or(fallback)
    }

    pub fn get<T: FromStr>(&mut self, key: &str, desc: &str) -> Option<T> {
        self.add_desc(key, desc, Type::Complex);
        self.values.get(key).and_then(|f| f.parse().ok())
    }

    pub fn fget<T: FromStr>(&mut self, key: &str, fallback: T, desc: &str) -> T {
        self.add_desc(key, desc, Type::Complex);
        self.values
            .get(key)
            .and_then(|f| f.parse().ok())
            .unwrap_or(fallback)
    }
}

#[derive(Debug)]
enum Type {
    String,
    Number,
    Double,
    Unsigned,
    Bool,
    Complex,
}

impl ToString for Type {
    fn to_string(&self) -> String {
        match self {
            Type::String => "string".to_string(),
            Type::Number => "integer".to_string(),
            Type::Double => "float".to_string(),
            Type::Unsigned => "unsigned".to_string(),
            Type::Bool => "bool".to_string(),
            Type::Complex => "complex".to_string(),
        }
    }
}
