/*
  MIT License http://www.opensource.org/licenses/mit-license.php
  Author Tobias Koppers @sokra
*/

// Port from https://github.com/webpack/webpack/blob/4b4ca3bb53f36a5b8fc6bc1bd976ed7af161bd80/lib/util/numberHash.js

const SAFE_LIMIT: usize = 0x80000000usize;
const SAFE_PART: usize = SAFE_LIMIT - 1;
const COUNT: usize = 4;

pub fn get_number_hash(str: &str, range: usize) -> usize {
  let mut arr = [0u64; COUNT];
  let primes = [3u64, 7u64, 17u64, 19u64];

  for i in 0..str.len() {
    let c = str.as_bytes()[i] as u64;
    arr[0] = (arr[0] + c * primes[0] + arr[3]) & SAFE_PART as u64;
    arr[1] = (arr[1] + c * primes[1] + arr[0]) & SAFE_PART as u64;
    arr[2] = (arr[2] + c * primes[2] + arr[1]) & SAFE_PART as u64;
    arr[3] = (arr[3] + c * primes[3] + arr[2]) & SAFE_PART as u64;

    arr[0] ^= arr[(arr[0] % COUNT as u64) as usize] >> 1;
    arr[1] ^= arr[(arr[1] % COUNT as u64) as usize] >> 1;
    arr[2] ^= arr[(arr[2] % COUNT as u64) as usize] >> 1;
    arr[3] ^= arr[(arr[3] % COUNT as u64) as usize] >> 1;
  }

  if range <= SAFE_PART {
    ((arr[0] + arr[1] + arr[2] + arr[3]) % range as u64) as usize
  } else {
    let range_ext = range / SAFE_LIMIT;
    let sum1 = (arr[0] + arr[2]) & SAFE_PART as u64;
    let sum2 = (arr[0] + arr[2]) % range_ext as u64;
    ((sum2 * SAFE_LIMIT as u64 + sum1) % range as u64) as usize
  }
}

#[test]
fn test_number_hash() {
  for n in [10, 100, 1000, 10000].iter() {
    let mut set = std::collections::HashSet::new();
    for i in 0..(*n * 200) {
      set.insert(get_number_hash(&format!("{i}"), *n));
      if set.len() >= (*n - 1) {
        break;
      }
    }
    assert_eq!(set.len(), (*n - 1));
  }
}
