use regex::Regex;
use lazy_static::lazy;
use std::str::FromStr;

pub trait HumanSize : Sized {
    fn to_human_size(&self) -> String;
    fn from_human_size(s : &str) -> Option<Self>;
}

impl HumanSize for u64 {
    fn to_human_size(&self) -> String {
        if *self < 1024 {
            return format!("{} bytes", self);
        }
        else if *self < 1024 * 1024 {
            return format!("{} KiB", self / 1024);
        }
        else if *self < 1024 * 1024 * 1024 {
            return format!("{} MiB", self / (1024 * 1024));
        }
        else {
            return format!("{} GiB", self / (1024 * 1024 * 1024));
        }
    }

    fn from_human_size(s : &str) -> Option<u64> {
        lazy_static! {
            static ref re : Regex = Regex::new(r"(?P<val>[0-9]+)\w*(?P<unit>[a-zA-Z])").unwrap();
        }
        match re.captures(s) {
            Some(caps) => {
                let val = u64::from_str(&caps["val"]).unwrap();
                match String::from_str(&caps["unit"]).unwrap().to_lowercase().as_str() {
                    "k" | "kb" | "kib" => Some(val * 1024),
                    "m" | "mb" | "mib" => Some(val * 1024 * 1024),
                    "g" | "gb" | "gib" => Some(val * 1024 * 1024 * 1024),
                    "t" | "tb" | "tib" => Some(val * 1024 * 1024 * 1024 * 1024),
                    _ => None
                }
            },
            None => None
        }
    }
}


