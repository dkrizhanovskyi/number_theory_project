use crate::strategy::FactorizationStrategy;
use crate::factorization::{trial_division, pollards_rho};

/// Strategy that uses Trial Division for factorization
pub struct TrialDivisionStrategy;

impl FactorizationStrategy for TrialDivisionStrategy {
    fn factorize(&self, n: u64) -> Option<u64> {
        trial_division(n)
    }
}

/// Strategy that uses Pollard's Rho algorithm for factorization
pub struct PollardsRhoStrategy;

impl FactorizationStrategy for PollardsRhoStrategy {
    fn factorize(&self, n: u64) -> Option<u64> {
        pollards_rho(n)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trial_division_strategy() {
        let strategy = TrialDivisionStrategy;
        assert_eq!(strategy.factorize(15), Some(3));
        assert_eq!(strategy.factorize(17), None); // 17 is prime
    }

    #[test]
    fn test_pollards_rho_strategy() {
        let strategy = PollardsRhoStrategy;
        assert_eq!(strategy.factorize(91), Some(7));
        assert_eq!(strategy.factorize(17), None); // 17 is prime
    }
}
