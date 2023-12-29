#[macro_export]
macro_rules! impl_debug_serde {
    ($name:ty) => {
        impl std::fmt::Debug for $name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                let string = serde_json::to_string(self).unwrap();
                f.write_str(&string[1..string.len() - 1])
            }
        }
    };
}
