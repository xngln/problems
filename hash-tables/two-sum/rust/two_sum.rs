use std::collections::HashMap;

fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut map = HashMap::new();
    for (index, num) in nums.iter().enumerate() {
        if map.contains_key(&(target - num)) {
            return vec![index as i32, map[&(target-num)] as i32]
        } else {
            map.insert(*num, index);
        }
    }
    vec![]
}

fn main() {
	let nums = vec![2, 7, 11, 15];
	let target = 9;

	println!("{:?}", two_sum(nums, target));
}		
