/// Represents the gender for first name selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Gender {
    Male,
    Female,
    /// Randomly selects male or female.
    #[default]
    Any,
}

/// Represents the parts of a generated name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NameParts {
    pub first_name: String,
    pub middle_name: Option<String>,
    pub last_name: String,
}

impl NameParts {
    /// Returns the full name as a single string.
    pub fn full_name(&self) -> String {
        match &self.middle_name {
            Some(middle) => format!("{} {} {}", self.first_name, middle, self.last_name),
            None => format!("{} {}", self.first_name, self.last_name),
        }
    }
}

/// Configuration for name generation.
#[derive(Debug, Clone)]
pub struct NameConfig {
    pub gender: Gender,
    pub include_middle_name: bool,
    pub allow_duplicates: bool,
}

impl Default for NameConfig {
    fn default() -> Self {
        Self {
            gender: Gender::Any,
            include_middle_name: false,
            allow_duplicates: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_full_name_without_middle() {
        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: None,
            last_name: "Smith".to_string(),
        };
        assert_eq!(name.full_name(), "John Smith");
    }

    #[test]
    fn test_full_name_with_middle() {
        let name = NameParts {
            first_name: "John".to_string(),
            middle_name: Some("William".to_string()),
            last_name: "Smith".to_string(),
        };
        assert_eq!(name.full_name(), "John William Smith");
    }

    #[test]
    fn test_default_config() {
        let config = NameConfig::default();
        assert_eq!(config.gender, Gender::Any);
        assert!(!config.include_middle_name);
        assert!(!config.allow_duplicates);
    }
}
