// should merge two arrays
// find mid value
// return f64 from i32
// if number is even add two mids and return sum
impl Solution {
  // result
  pub fn find_median_sorted_arrays(mut nums1: Vec<i32>, mut nums2: Vec<i32>) -> f64 {
    // sorting them
    let mut concated: Vec<i32> = Self::concate(nums1, nums2);
    concated.sort();
    // returning sum
    let n1: f64 = Self::sum(concated);
    n1
  }

  // returns sum of mid
  fn sum(nums: Vec<i32>) -> f64 {
    let leen: f64 = nums.len() as f64;
  	let middle_index: f64 = leen / 2.;
    match nums.len() % 2 {
      0 => {
        let first = nums[(middle_index - 0.5) as u32 as usize];
        let second = nums[(middle_index + 0.5) as u32 as usize];
        (first as f64 + second as f64) / 2.
      }
      _ => {
        nums[middle_index as u32 as usize] as f64
		  }
	  }
  }
  
  // concating them
  fn concate(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let length: usize = nums1.len() + nums2.len();
    let out: Vec<i32> = {
      let mut out: Vec<i32> = vec![0; length]; 
      let (one, two) = out.split_at_mut(nums1.len().clone());
      one.copy_from_slice(&nums1);
      two.copy_from_slice(&nums2);
      out
    };
    out
  }
}
