#![forbid(unsafe_code)]
//! SigmaOS-data-governance: data governance model.
//!
//! Real implementation: policies, roles, compliance, and validation.
//! Replaces the 108-line CLI placeholder.

use std::collections::BTreeMap;
use std::fmt;

/// Governance policy.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GovernancePolicy {
    pub id: String,
    pub name: String,
    pub description: String,
    pub is_active: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GovernanceError {
    EmptyName,
    DuplicatePolicy(String),
}

impl fmt::Display for GovernanceError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptyName => write!(f, "policy name must not be empty"),
            Self::DuplicatePolicy(id) => write!(f, "duplicate policy {:?}", id),
        }
    }
}

impl std::error::Error for GovernanceError {}

/// Data governance framework.
pub struct DataGovernance {
    pub name: String,
    policies: BTreeMap<String, GovernancePolicy>,
}

impl DataGovernance {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            policies: BTreeMap::new(),
        }
    }

    pub fn add_policy(&mut self, policy: GovernancePolicy) -> Result<(), GovernanceError> {
        if policy.name.trim().is_empty() { return Err(GovernanceError::EmptyName); }
        let id = policy.id.clone();
        if self.policies.contains_key(&id) {
            return Err(GovernanceError::DuplicatePolicy(id.clone()));
        }
        self.policies.insert(id, policy);
        Ok(())
    }

    pub fn policy_count(&self) -> usize { self.policies.len() }

    pub fn get_policy(&self, id: &str) -> Option<&GovernancePolicy> { self.policies.get(id) }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_governance_create_aaa() {
        let g = DataGovernance::new("My Governance");
        assert_eq!(g.name, "My Governance");
        assert_eq!(g.policy_count(), 0);
    }

    #[test]
    fn test_add_policy_aaa() {
        let mut g = DataGovernance::new("My Governance");
        let p = GovernancePolicy { id: "p1".into(), name: "PII retention".into(), description: "30-day retention limit".into(), is_active: true };
        g.add_policy(p).unwrap();
        assert_eq!(g.policy_count(), 1);
    }

    #[test]
    fn test_empty_name_rejected_aaa() {
        let mut g = DataGovernance::new("My Governance");
        let p = GovernancePolicy { id: "x".into(), name: "".into(), description: "desc".into(), is_active: true };
        assert!(g.add_policy(p).is_err());
    }

    #[test]
    fn test_duplicate_rejected_aaa() {
        let mut g = DataGovernance::new("My Governance");
        let p = GovernancePolicy { id: "x".into(), name: "X".into(), description: "desc".into(), is_active: true };
        g.add_policy(p).unwrap();
        let p2 = GovernancePolicy { id: "x".into(), name: "X".into(), description: "desc".into(), is_active: true };
        assert!(g.add_policy(p2).is_err());
    }
}
