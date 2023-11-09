fn main() {
    let mut obj = SeatManager::new(5);
    let mut ans = vec![];
    ans.push(obj.reserve());
    ans.push(obj.reserve());
    obj.unreserve(2);
    ans.push(obj.reserve());
    ans.push(obj.reserve());
    ans.push(obj.reserve());
    ans.push(obj.reserve());
    obj.unreserve(2);
    assert_eq!(ans, vec![1, 2, 2, 3, 4, 5]);
}
// use std::collections::BTreeSet;
// struct SeatManager {
//     seat_set: BTreeSet<i32>,
// }
// impl SeatManager {
//     fn new(n: i32) -> Self {
//         SeatManager {
//             seat_set: (1..=n).collect(),
//         }
//     }
//     fn reserve(&mut self) -> i32 {
//         let smallest = *(self.seat_set.iter().next().unwrap());
//         self.seat_set.remove(&smallest);
//         smallest
//     }
//     fn unreserve(&mut self, seat_number: i32) {
//         self.seat_set.insert(seat_number);
//     }
// }
use std::cmp::Reverse;
use std::collections::BinaryHeap;
struct SeatManager {
    next_empty_seat: i32,
    unreserved_seats: BinaryHeap<Reverse<i32>>,
}
impl SeatManager {
    fn new(_n: i32) -> Self {
        SeatManager {
            next_empty_seat: 0,
            unreserved_seats: BinaryHeap::new(),
        }
    }
    fn reserve(&mut self) -> i32 {
        if self.unreserved_seats.is_empty() {
            self.next_empty_seat += 1;
            self.next_empty_seat
        } else {
            self.unreserved_seats.pop().unwrap().0
        }
    }
    fn unreserve(&mut self, seat_number: i32) {
        if seat_number == self.next_empty_seat {
            self.next_empty_seat -= 1;
        } else {
            self.unreserved_seats.push(Reverse(seat_number));
        }
    }
}
