use std::sync::LazyLock;

static MALE_RAW: &str = include_str!("../../files/male-first.txt");
static FEMALE_RAW: &str = include_str!("../../files/female-first.txt");
static LAST_RAW: &str = include_str!("../../files/last-names.txt");

fn parse_names(raw: &'static str) -> Vec<&'static str> {
    raw.lines().filter(|line| !line.is_empty()).collect()
}

pub static MALE_NAMES: LazyLock<Vec<&'static str>> = LazyLock::new(|| parse_names(MALE_RAW));
pub static FEMALE_NAMES: LazyLock<Vec<&'static str>> = LazyLock::new(|| parse_names(FEMALE_RAW));
pub static LAST_NAMES: LazyLock<Vec<&'static str>> = LazyLock::new(|| parse_names(LAST_RAW));

/// Trait for name data sources (extensibility point).
pub trait NameSource: Send + Sync {
    fn male_first_names(&self) -> &[&str];
    fn female_first_names(&self) -> &[&str];
    fn last_names(&self) -> &[&str];
}

/// Default implementation using embedded data.
pub struct EmbeddedNameSource;

impl NameSource for EmbeddedNameSource {
    fn male_first_names(&self) -> &[&str] {
        &MALE_NAMES
    }

    fn female_first_names(&self) -> &[&str] {
        &FEMALE_NAMES
    }

    fn last_names(&self) -> &[&str] {
        &LAST_NAMES
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_male_names_loaded() {
        let source = EmbeddedNameSource;
        let names = source.male_first_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"Michael"));
        assert!(names.contains(&"William"));
    }

    #[test]
    fn test_female_names_loaded() {
        let source = EmbeddedNameSource;
        let names = source.female_first_names();
        assert!(!names.is_empty());
        assert!(names.contains(&"Jessica"));
        assert!(names.contains(&"Emily"));
    }

    #[test]
    fn test_last_names_loaded() {
        let source = EmbeddedNameSource;
        let names = source.last_names();
        assert!(!names.is_empty());
    }

    #[test]
    fn test_no_empty_names() {
        let source = EmbeddedNameSource;
        assert!(source.male_first_names().iter().all(|n| !n.is_empty()));
        assert!(source.female_first_names().iter().all(|n| !n.is_empty()));
        assert!(source.last_names().iter().all(|n| !n.is_empty()));
    }
}
