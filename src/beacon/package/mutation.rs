//! In-process lifecycle and inspection serialization. External writers still face
//! the package's stable-source, confinement, inventory and no-replace checks.

use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Mutex, OnceLock, Weak};

use super::{canonical_root, LifecycleError};

type RootLocks = HashMap<PathBuf, Weak<Mutex<()>>>;

fn registry() -> &'static Mutex<RootLocks> {
    static LOCKS: OnceLock<Mutex<RootLocks>> = OnceLock::new();
    LOCKS.get_or_init(|| Mutex::new(HashMap::new()))
}

struct Lease {
    root: PathBuf,
    mutex: Arc<Mutex<()>>,
}

impl Lease {
    fn new(root: &Path) -> Self {
        let mut locks = registry().lock().unwrap_or_else(|error| error.into_inner());
        locks.retain(|_, mutex| mutex.strong_count() > 0);
        let mutex = locks.get(root).and_then(Weak::upgrade).unwrap_or_else(|| {
            let mutex = Arc::new(Mutex::new(()));
            locks.insert(root.to_owned(), Arc::downgrade(&mutex));
            mutex
        });
        Self {
            root: root.to_owned(),
            mutex,
        }
    }
}

impl Drop for Lease {
    fn drop(&mut self) {
        let mut locks = registry().lock().unwrap_or_else(|error| error.into_inner());
        // Waiting callers hold strong leases too. Only the final caller may
        // retire this registry entry, so a new caller cannot obtain a second
        // mutex while an earlier operation is still running or waiting.
        if Arc::strong_count(&self.mutex) == 1
            && locks
                .get(&self.root)
                .is_some_and(|registered| Weak::ptr_eq(registered, &Arc::downgrade(&self.mutex)))
        {
            locks.remove(&self.root);
        }
    }
}

pub(super) fn with_root<T>(
    addons_root: &Path,
    operation: impl FnOnce(&Path) -> Result<T, LifecycleError>,
) -> Result<T, LifecycleError> {
    let root = canonical_root(addons_root)?;
    with_canonical_root(&root, || {
        if canonical_root(&root)? != root {
            return Err(LifecycleError::Integrity(
                "AddOns root changed while awaiting its lifecycle operation".into(),
            ));
        }
        operation(&root)
    })
}

pub(super) fn with_canonical_root<T>(root: &Path, operation: impl FnOnce() -> T) -> T {
    let lease = Lease::new(root);
    // A previous panic leaves its journal for ordinary recovery. The next caller
    // still acquires the same mutex and runs recovery before another mutation.
    let _guard = lease
        .mutex
        .lock()
        .unwrap_or_else(|error| error.into_inner());
    operation()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn shared_leases_release_the_registry_entry_after_the_last_caller() {
        let sandbox = tempfile::tempdir().unwrap();
        let root = std::fs::canonicalize(sandbox.path()).unwrap();
        let first = Lease::new(&root);
        let second = Lease::new(&root);
        assert!(Arc::ptr_eq(&first.mutex, &second.mutex));
        drop(first);
        assert!(registry().lock().unwrap().contains_key(&root));
        drop(second);
        assert!(!registry().lock().unwrap().contains_key(&root));
    }
}
