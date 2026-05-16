use soroban_sdk::{Env, Symbol, contracterror, symbol_short};

const REENTRANCY_KEY: Symbol = symbol_short!("reent");

/// ReentrancyGuard prevents recursive calls to the same function.
/// Critical for financial contracts to prevent reentrancy attacks.
///
/// Usage:
///   let guard = ReentrancyGuard::new(&env)?;
///   // ... mutating logic ...
///   drop(guard); // or let it go out of scope
pub struct ReentrancyGuard {
    env: Env,
}

impl ReentrancyGuard {
    /// Acquires the reentrancy lock. Returns an error if already locked.
    pub fn new(env: &Env) -> Result<Self, ReentrancyError> {
        let locked: bool = env.storage().instance().get(&REENTRANCY_KEY).unwrap_or(false);
        if locked {
            return Err(ReentrancyError::ReentrantCall);
        }
        env.storage().instance().set(&REENTRANCY_KEY, &true);
        Ok(Self { env: env.clone() })
    }
}

impl Drop for ReentrancyGuard {
    fn drop(&mut self) {
        self.env.storage().instance().set(&REENTRANCY_KEY, &false);
    }
}

#[contracterror]
#[derive(Debug)]
pub enum ReentrancyError {
    ReentrantCall = 1,
}
