pub trait HumanSize {
    fn human_size(&self) -> String;
}

impl HumanSize for u64 {
    fn human_size(&self) -> String {
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
}


