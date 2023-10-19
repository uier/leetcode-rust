struct MountainArray {
    arr: Vec<i32>,
}
impl MountainArray {
    fn new(arr: Vec<i32>) -> MountainArray {
        MountainArray { arr }
    }
    fn get(&self, index: i32) -> i32 {
        self.arr[index as usize]
    }
    fn length(&self) -> i32 {
        self.arr.len() as i32
    }
}
fn main() {
    let tests = [
        (vec![1, 2, 3, 4, 5, 3, 1], 3),
        (vec![0, 1, 2, 4, 2, 1], 3),
        (vec![1, 3, 1], 2),
        (vec![1, 2, 3, 4, 5, 6, 2], 2),
        (vec![1, 1, 3, 4, 5, 6, 2], 2),
    ];
    let answers = [2, -1, -1, 1, 6];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let arr = MountainArray::new(test.0);
        let answer = Solution::find_in_mountain_array(test.1, &arr);
        assert_eq!(answer, expected_answer);
    }
}
struct Solution;
/**
 * // This is the MountainArray's API interface.
 * // You should not implement it, or speculate about its implementation
 *  struct MountainArray;
 *  impl MountainArray {
 *     fn get(index:i32)->i32;
 *     fn length()->i32;
 * };
 */

impl Solution {
    pub fn find_peak_index(n: i32, arr: &MountainArray) -> usize {
        let mut l = 0_usize;
        let mut r = (n - 1) as usize;
        loop {
            if r - l <= 2 {
                return (l..=r)
                    .map(|i| (i, arr.get(i as i32)))
                    .max_by(|(_, av), (_, bv)| av.cmp(bv))
                    .map(|(i, _)| i)
                    .unwrap();
            }
            let m1 = l + (r - l) / 3;
            let m2 = l + (r - l) / 3 * 2;
            if arr.get(m1 as i32) <= arr.get(m2 as i32) {
                l = m1 + 1;
            } else {
                r = m2 - 1;
            }
        }
    }
    pub fn binary_search(
        mut l: usize,
        mut r: usize,
        target: i32,
        arr: &MountainArray,
        decr: bool,
    ) -> usize {
        loop {
            if r - l <= 1 {
                return if arr.get(l as i32) == target {
                    l
                } else if r != l && arr.get(r as i32) == target {
                    r
                } else {
                    usize::MAX
                };
            }
            let m = (l + r) >> 1;
            match arr.get(m as i32).cmp(&target) {
                std::cmp::Ordering::Less => {
                    if decr {
                        r = m - 1
                    } else {
                        l = m + 1
                    }
                }
                std::cmp::Ordering::Greater => {
                    if decr {
                        l = m + 1
                    } else {
                        r = m - 1
                    }
                }
                std::cmp::Ordering::Equal => {
                    return m;
                }
            }
        }
    }
    pub fn find_in_mountain_array(target: i32, arr: &MountainArray) -> i32 {
        let n = arr.length();
        let index = Self::find_peak_index(n, arr);
        let li = Self::binary_search(0, index, target, arr, false);
        let ri = Self::binary_search(index, (n - 1) as usize, target, arr, true);
        if li != usize::MAX {
            li as i32
        } else if ri != usize::MAX {
            ri as i32
        } else {
            -1
        }
    }
}
