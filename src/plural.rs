pub fn plural<S: AsRef<str>>(count: usize, singular: S, plural: S) -> String {
    if count == 1 {
        format!("1 {}", singular.as_ref())
    } else {
        format!("{} {}", count, plural.as_ref())
    }
}
