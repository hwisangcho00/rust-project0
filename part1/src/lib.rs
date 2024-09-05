/// Returns the sqrt of `n`
///
/// Using built-in square root functions is not allowed.
pub fn sqrt(n: u32) -> u32 {
  let ans = {
    let mut ans = 1;
    loop {
      if ans * ans > n {
        break ans - 1;
      }
      ans += 1;
    }
  };
  ans
} 

/// Consumes a sorted list of integers and a query integer. Returns the index of the query integer
///
/// Note that a 3-valued comparison between numbers `a` and `b` can be done easily:
/// ```rust,ignore
/// match a.cmp(&b) {
///    std::cmp::Ordering::Less => { ... },
///    std::cmp::Ordering::Greater => { ... },
///    std::cmp::Ordering::Equal => { ... },
/// }
/// ```
pub fn binary_search(arr: &[i32], query: i32) -> Option<u32> {
    let mut lo = 0;
    let mut hi = arr.len();

    while lo < hi {
      let mid = lo + (hi - lo) / 2;

      if arr[mid] == query {
        return Some(mid as u32)
      } else if arr[mid] > query {
        hi = mid
      } else {
        lo = mid + 1
      }
    }

    return None

}

/// Consumes a list of numbers representing daily rainfall. The list may contain -999 signifying
/// the end of data of interest. Returns the average of non-negative values up to the first
/// occurrence of -999 (if it occurs). There may be negative numbers other -999 in the list.
/// Returns None if the average is incomputable.
///
/// example: rainfall([6, 8, -1, 1, -999, 4, 5, 6]) -> Some(5.0)
pub fn rainfall(values: &[i32]) -> Option<f64> {
    let mut sum = 0;
    let mut cnt = 0;
    for &n in values {
      if n == -999 {
        if cnt == 0 {
          return None
        }
        return Some(sum as f64 / cnt as f64);
      }
      
      if n >= 0 {
        sum += n;
        cnt += 1;
      }
      
    }

    if cnt == 0 {
      return None
    }

    return Some(sum as f64 / cnt as f64);
}
