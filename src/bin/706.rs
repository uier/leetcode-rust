use std::i32::MAX;
fn main() {
    let tests: Vec<(Vec<&str>, Vec<Vec<i32>>)> = vec![(
        vec![
            "MyHashMap",
            "put",
            "put",
            "get",
            "get",
            "put",
            "get",
            "remove",
            "get",
        ],
        vec![
            vec![],
            vec![1, 1],
            vec![2, 2],
            vec![1],
            vec![3],
            vec![2, 1],
            vec![2],
            vec![2],
            vec![2],
        ],
    )];
    let answers: Vec<Vec<i32>> = vec![vec![MAX, MAX, MAX, 1, -1, MAX, 1, MAX, -1]];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        assert_eq!(test.0.len(), expected_answer.len());
        assert_eq!(test.1.len(), expected_answer.len());
        let mut obj = MyHashMap::new();
        for i in 1..test.0.len() {
            match test.0[i] {
                "put" => {
                    obj.put(test.1[i][0], test.1[i][1]);
                }
                "get" => {
                    assert_eq!(obj.get(test.1[i][0]), expected_answer[i]);
                }
                "remove" => {
                    obj.remove(test.1[i][0]);
                }
                &_ => panic!(),
            };
        }
    }
}

use std::collections::HashMap;
struct MyHashMap {
    h: HashMap<i32, i32>,
}
impl MyHashMap {
    fn new() -> Self {
        MyHashMap { h: HashMap::new() }
    }

    fn put(&mut self, key: i32, value: i32) {
        self.h.insert(key, value);
    }

    fn get(&self, key: i32) -> i32 {
        match self.h.get(&key) {
            Some(&v) => v,
            None => -1,
        }
    }

    fn remove(&mut self, key: i32) {
        self.h.remove(&key);
    }
}
