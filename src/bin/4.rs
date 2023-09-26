fn main() {
    let tests = [
        (vec![3], vec![1, 2, 4]),
        (vec![1, 3], vec![2]),
        (vec![1, 2], vec![3, 4]),
        (vec![1], vec![]),
        (vec![1, 2, 3], vec![]),
        (vec![2], vec![1, 3]),
        (vec![], vec![1, 2]),
        (vec![2, 2], vec![1, 3, 4, 5, 6, 7]),
        (vec![1, 2, 4], vec![3, 3, 3, 5, 6, 7]),
        (vec![1, 2, 4, 5, 6], vec![3, 3, 3, 4, 4, 4]),
        (vec![4, 4, 4, 4, 5, 6], vec![3, 3, 3, 4, 7, 8]),
        (vec![1, 1, 1], vec![1, 1, 1, 1, 1]),
        (vec![0, 0, 0, 0, 0], vec![-1, 0, 0, 0, 0, 0, 1]),
    ];
    let answers = [
        2.5, 2.0, 2.5, 1.0, 2.0, 2.0, 1.5, 3.5, 3.0, 4.0, 4.0, 1.0, 0.0,
    ];
    for ((mat, k), expected_answer) in tests.into_iter().zip(answers) {
        let answer = Solution::find_median_sorted_arrays(mat, k);
        assert_eq!(answer, expected_answer);
        println!("PASS");
    }
}
struct Solution;
impl Solution {
    pub fn find_kth_smallest_element(
        nums1: &Vec<i32>,
        nums2: &Vec<i32>,
        l1: i32,
        r1: i32,
        l2: i32,
        r2: i32,
        k: i32,
    ) -> i32 {
        // println!("{l1} {r1} {l2} {r2} {k}");
        if l1 > r1 {
            return nums2[(l2 + k - 1) as usize];
        }
        if l2 > r2 {
            return nums1[(l1 + k - 1) as usize];
        }
        if k == 1 {
            return std::cmp::min(nums1[l1 as usize], nums2[l2 as usize]);
        }
        let mid1 = (l1 + r1) / 2;
        let mid2 = (l2 + r2) / 2;
        let smaller_element_cnt = (mid1 - l1 + mid2 - l2 + 2) as i32;
        if k < smaller_element_cnt {
            return if nums1[mid1 as usize] <= nums2[mid2 as usize] {
                Solution::find_kth_smallest_element(nums1, nums2, l1, r1, l2, mid2 - 1, k)
            } else {
                Solution::find_kth_smallest_element(nums1, nums2, l1, mid1 - 1, l2, r2, k)
            };
        } else if k > smaller_element_cnt {
            return if nums1[mid1 as usize] <= nums2[mid2 as usize] {
                Solution::find_kth_smallest_element(
                    nums1,
                    nums2,
                    mid1 + 1,
                    r1,
                    l2,
                    r2,
                    k - ((mid1 - l1) as i32 + 1),
                )
            } else {
                Solution::find_kth_smallest_element(
                    nums1,
                    nums2,
                    l1,
                    r1,
                    mid2 + 1,
                    r2,
                    k - ((mid2 - l2) as i32 + 1),
                )
            };
        }
        if nums1[mid1 as usize] <= nums2[mid2 as usize] {
            return Solution::find_kth_smallest_element(
                nums1,
                nums2,
                mid1 + 1,
                r1,
                l2,
                mid2,
                k - ((mid1 - l1) as i32 + 1),
            );
        }
        Solution::find_kth_smallest_element(
            nums1,
            nums2,
            l1,
            mid1,
            mid2 + 1,
            r2,
            k - ((mid2 - l2) as i32 + 1),
        )
    }
    pub fn find_median_sorted_arrays(nums1: Vec<i32>, nums2: Vec<i32>) -> f64 {
        let total_len = nums1.len() + nums2.len();
        let k = total_len / 2 + total_len % 2;
        let kth = Solution::find_kth_smallest_element(
            &nums1,
            &nums2,
            0,
            nums1.len() as i32 - 1,
            0,
            nums2.len() as i32 - 1,
            k as i32,
        );
        if total_len % 2 == 0 {
            let k_plus1_th = Solution::find_kth_smallest_element(
                &nums1,
                &nums2,
                0,
                nums1.len() as i32 - 1,
                0,
                nums2.len() as i32 - 1,
                (k + 1) as i32,
            );
            return (kth as f64 + k_plus1_th as f64) / 2.0;
        }
        kth as f64
    }
}
