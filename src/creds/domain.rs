/// Trait for domain selection strategies.
pub trait DomainSource: Send + Sync {
    /// Select a domain for email generation.
    fn select_domain(&self) -> &str;
}

/// Default domain source with random selection between gmail.com and outlook.com.
pub struct DefaultDomainSource;

impl DomainSource for DefaultDomainSource {
    fn select_domain(&self) -> &str {
        if fastrand::bool() {
            "gmail.com"
        } else {
            "outlook.com"
        }
    }
}

/// Fixed domain source for user-specified domains.
pub struct FixedDomainSource {
    domain: String,
}

impl FixedDomainSource {
    /// Create a new fixed domain source with the specified domain.
    pub fn new(domain: String) -> Self {
        Self { domain }
    }
}

impl DomainSource for FixedDomainSource {
    fn select_domain(&self) -> &str {
        &self.domain
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_domain_returns_valid_domain() {
        let source = DefaultDomainSource;
        let domain = source.select_domain();

        assert!(
            domain == "gmail.com" || domain == "outlook.com",
            "Domain should be either gmail.com or outlook.com, got: {}",
            domain
        );
    }

    #[test]
    fn test_fixed_domain_returns_specified_domain() {
        let source = FixedDomainSource::new("custom.example.com".to_string());

        assert_eq!(source.select_domain(), "custom.example.com");
    }

    #[test]
    fn test_fixed_domain_is_consistent() {
        let source = FixedDomainSource::new("test.org".to_string());

        // Should always return the same domain
        assert_eq!(source.select_domain(), "test.org");
        assert_eq!(source.select_domain(), "test.org");
        assert_eq!(source.select_domain(), "test.org");
    }
}
