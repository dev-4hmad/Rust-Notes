use std::collections::HashMap;
struct Solution;
impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut map = HashMap::new(); // value -> index

        for (i, &num) in nums.iter().enumerate() {
            let complement = target - num;
            if let Some(&idx) = map.get(&complement) {
                return vec![idx as i32, i as i32];
            }
            map.insert(num, i);
        }

        vec![]
    }
}
fn main(){
    let nums = vec![4,5,6,7];
    let target = 10;
    println!("{:?}", Solution::two_sum(nums, target));
}
