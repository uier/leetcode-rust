fn main() {
    let tests = [
        vec![
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
            NestedInteger::Int(2),
            NestedInteger::List(vec![NestedInteger::Int(1), NestedInteger::Int(1)]),
        ],
        vec![
            NestedInteger::Int(1),
            NestedInteger::List(vec![
                NestedInteger::Int(4),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
            ]),
        ],
        vec![
            NestedInteger::Int(4),
            NestedInteger::List(vec![
                NestedInteger::Int(5),
                NestedInteger::List(vec![NestedInteger::Int(6)]),
                NestedInteger::List(vec![]),
                NestedInteger::List(vec![NestedInteger::Int(777)]),
                NestedInteger::Int(1),
            ]),
            NestedInteger::Int(3),
        ],
        vec![NestedInteger::List(vec![])],
    ];
    let answers = [
        vec![1, 1, 2, 1, 1],
        vec![1, 4, 6],
        vec![4, 5, 6, 777, 1, 3],
        vec![],
    ];
    for (test, expected_answer) in tests.into_iter().zip(answers) {
        let mut iter = NestedIterator::new(test);
        let mut answer = Vec::<i32>::new();
        while iter.has_next() {
            answer.push(iter.next());
        }
        assert_eq!(answer, expected_answer);
    }
}
#[derive(Debug, PartialEq, Eq)]
pub enum NestedInteger {
    Int(i32),
    List(Vec<NestedInteger>),
}
struct NestedIterator {
    next_value: Option<i32>,
    state: Box<dyn Iterator<Item = NestedInteger>>,
    iter: Option<Box<NestedIterator>>,
}
impl NestedIterator {
    fn new(nested_list: Vec<NestedInteger>) -> Self {
        Self {
            next_value: None,
            state: Box::new(nested_list.into_iter()),
            iter: None,
        }
    }
    fn next(&self) -> i32 {
        self.next_value.unwrap()
    }
    fn prepare_next(&mut self) -> bool {
        loop {
            match self.state.next() {
                Some(NestedInteger::Int(v)) => {
                    self.next_value = Some(v);
                    break true;
                }
                Some(NestedInteger::List(arr)) => {
                    self.iter = Some(Box::new(NestedIterator::new(arr)));
                    if !self.iter.as_mut().unwrap().has_next() {
                        continue;
                    }
                    self.next_value = Some(self.iter.as_mut().unwrap().next());
                    break true;
                }
                None => break false,
            }
        }
    }
    fn has_next(&mut self) -> bool {
        match self.iter.as_mut() {
            Some(self_iter) => {
                if self_iter.has_next() {
                    self.next_value = Some(self_iter.next());
                    true
                } else {
                    self.prepare_next()
                }
            }
            None => self.prepare_next(),
        }
    }
}
