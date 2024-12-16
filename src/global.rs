use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};

use rand::{SeedableRng, rngs::StdRng};

use crate::Unreal;

/// # Errors
/// Returns an error if the global [`Mutex`] is poisoned.
pub fn global_lock()
-> Result<MutexGuard<'static, Unreal<StdRng>>, PoisonError<MutexGuard<'static, Unreal<StdRng>>>> {
    GLOBAL_UNREAL.lock()
}

/// # Errors
/// Returns an error if the global [`Mutex`] is poisoned.
pub fn seed_global(seed: u64) -> Result<(), PoisonError<MutexGuard<'static, Unreal<StdRng>>>> {
    global_lock()?.rng = StdRng::seed_from_u64(seed);
    Ok(())
}

/// # Errors
/// Returns an error if the global [`Mutex`] is poisoned.
pub fn seed_global_from_entropy() -> Result<(), PoisonError<MutexGuard<'static, Unreal<StdRng>>>> {
    global_lock()?.rng = StdRng::from_entropy();
    Ok(())
}

static GLOBAL_UNREAL: LazyLock<Mutex<Unreal<StdRng>>> =
    LazyLock::new(|| Mutex::new(Unreal::from_rng(StdRng::from_entropy())));
