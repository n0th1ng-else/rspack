use std::mem;

use tokio::task::spawn_blocking;

/// Fast set `src` into the referenced `dest`, and drop the old value in other thread
///
/// This method is used when the dropping time is long
pub fn fast_set<T>(dest: &mut T, src: T)
where
  T: Send + 'static,
{
  let old = mem::replace(dest, src);
  spawn_blocking(|| {
    mem::drop(old);
  });
}
