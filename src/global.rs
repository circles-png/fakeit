use std::sync::{LazyLock, Mutex, MutexGuard, PoisonError};

use rand::{SeedableRng, rngs::StdRng};

use crate::Unreal;

/// Lock the global [`Mutex`] and return a RAII guard to the global [`Unreal`] instance. If there
/// is no global [`Unreal`] instance, one will be lazily initialised with a random seed.
///
/// # Errors
/// Returns an error if the global [`Mutex`] is poisoned.
pub fn global_lock()
-> Result<MutexGuard<'static, Unreal<StdRng>>, PoisonError<MutexGuard<'static, Unreal<StdRng>>>> {
    GLOBAL_UNREAL.lock()
}

/// Seed the global [`Mutex`] with the given seed.
///
/// # Errors
/// Returns an error if the global [`Mutex`] is poisoned.
pub fn seed_global(seed: u64) -> Result<(), PoisonError<MutexGuard<'static, Unreal<StdRng>>>> {
    global_lock()?.rng = StdRng::seed_from_u64(seed);
    Ok(())
}

/// Randomly seed the global [`Mutex`] with entropy.
///
/// # Errors
/// Returns an error if the global [`Mutex`] is poisoned.
pub fn seed_global_from_entropy() -> Result<(), PoisonError<MutexGuard<'static, Unreal<StdRng>>>> {
    global_lock()?.rng = StdRng::from_entropy();
    Ok(())
}

static GLOBAL_UNREAL: LazyLock<Mutex<Unreal<StdRng>>> =
    LazyLock::new(|| Mutex::new(Unreal::from_rng(StdRng::from_entropy())));
