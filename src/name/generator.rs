use std::collections::HashSet;

use anyhow::bail;

use super::data::NameSource;
use super::types::{Gender, NameConfig, NameParts};

/// Generates random names using a provided name source.
pub struct NameGenerator<S: NameSource> {
    source: S,
}

impl<S: NameSource> NameGenerator<S> {
    pub fn new(source: S) -> Self {
        Self { source }
    }

    /// Calculate max unique combinations for the given config.
    fn max_unique_names(&self, config: &NameConfig) -> usize {
        let first_count = match config.gender {
            Gender::Male => self.source.male_first_names().len(),
            Gender::Female => self.source.female_first_names().len(),
            Gender::Any => {
                self.source.male_first_names().len() + self.source.female_first_names().len()
            }
        };
        let last_count = self.source.last_names().len();

        if config.include_middle_name {
            first_count * first_count * last_count
        } else {
            first_count * last_count
        }
    }

    /// Generate multiple names according to the config.
    pub fn generate_many(
        &self,
        config: &NameConfig,
        count: usize,
    ) -> anyhow::Result<Vec<NameParts>> {
        if config.allow_duplicates {
            return (0..count)
                .map(|_| self.generate_one_random(config))
                .collect();
        }

        let max_unique = self.max_unique_names(config);
        if count > max_unique {
            bail!(
                "Cannot generate {} unique names. Maximum possible with current settings: {}",
                count,
                max_unique
            );
        }

        let mut seen: HashSet<String> = HashSet::with_capacity(count);
        let mut results: Vec<NameParts> = Vec::with_capacity(count);

        while results.len() < count {
            let name = self.generate_one_random(config)?;
            let key = name.full_name();

            if seen.insert(key) {
                results.push(name);
            }
        }

        Ok(results)
    }

    fn generate_one_random(&self, config: &NameConfig) -> anyhow::Result<NameParts> {
        let first_names = self.select_first_name_pool(config.gender);

        let first_name = first_names[fastrand::usize(..first_names.len())].to_string();

        let middle_name = if config.include_middle_name {
            Some(first_names[fastrand::usize(..first_names.len())].to_string())
        } else {
            None
        };

        let last_names = self.source.last_names();
        let last_name = last_names[fastrand::usize(..last_names.len())].to_string();

        Ok(NameParts {
            first_name,
            middle_name,
            last_name,
        })
    }

    fn select_first_name_pool(&self, gender: Gender) -> &[&str] {
        match gender {
            Gender::Male => self.source.male_first_names(),
            Gender::Female => self.source.female_first_names(),
            Gender::Any => {
                if fastrand::bool() {
                    self.source.male_first_names()
                } else {
                    self.source.female_first_names()
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::name::data::EmbeddedNameSource;

    #[test]
    fn test_generate_single_name() {
        let generator = NameGenerator::new(EmbeddedNameSource);
        let config = NameConfig::default();
        let names = generator.generate_many(&config, 1).unwrap();

        assert_eq!(names.len(), 1);
        assert!(!names[0].first_name.is_empty());
        assert!(!names[0].last_name.is_empty());
        assert!(names[0].middle_name.is_none());
    }

    #[test]
    fn test_generate_with_middle_name() {
        let generator = NameGenerator::new(EmbeddedNameSource);
        let config = NameConfig {
            gender: Gender::Male,
            include_middle_name: true,
            allow_duplicates: false,
        };
        let names = generator.generate_many(&config, 1).unwrap();

        assert_eq!(names.len(), 1);
        assert!(names[0].middle_name.is_some());
    }

    #[test]
    fn test_generate_unique_names() {
        let generator = NameGenerator::new(EmbeddedNameSource);
        let config = NameConfig::default();
        let names = generator.generate_many(&config, 100).unwrap();

        let unique: HashSet<String> = names.iter().map(|n| n.full_name()).collect();
        assert_eq!(names.len(), unique.len());
    }

    #[test]
    fn test_error_on_too_many_unique() {
        let generator = NameGenerator::new(EmbeddedNameSource);
        let config = NameConfig {
            gender: Gender::Male,
            include_middle_name: false,
            allow_duplicates: false,
        };

        // Male only, no middle = 741 * 482 = 357,162 max
        let result = generator.generate_many(&config, 400_000);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Cannot generate"));
    }

    #[test]
    fn test_male_only() {
        let generator = NameGenerator::new(EmbeddedNameSource);
        let config = NameConfig {
            gender: Gender::Male,
            include_middle_name: false,
            allow_duplicates: false,
        };
        let names = generator.generate_many(&config, 10).unwrap();

        let male_names: Vec<&str> = EmbeddedNameSource.male_first_names().to_vec();
        for name in &names {
            assert!(
                male_names.contains(&name.first_name.as_str()),
                "Expected male name, got: {}",
                name.first_name
            );
        }
    }

    #[test]
    fn test_female_only() {
        let generator = NameGenerator::new(EmbeddedNameSource);
        let config = NameConfig {
            gender: Gender::Female,
            include_middle_name: false,
            allow_duplicates: false,
        };
        let names = generator.generate_many(&config, 10).unwrap();

        let female_names: Vec<&str> = EmbeddedNameSource.female_first_names().to_vec();
        for name in &names {
            assert!(
                female_names.contains(&name.first_name.as_str()),
                "Expected female name, got: {}",
                name.first_name
            );
        }
    }
}
