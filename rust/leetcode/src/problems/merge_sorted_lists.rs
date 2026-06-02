pub struct Node<T> {
    pub val: T,
    pub next: Option<Box<Node<T>>>,
}

pub fn merge_two_lists<T: PartialOrd + Clone>(
    mut list1: Option<Box<Node<T>>>,
    mut list2: Option<Box<Node<T>>>,
) -> Option<Box<Node<T>>> {
    let mut head = None;
    let mut tail = &mut head;

    while list1.is_some() && list2.is_some() {
        let source =
            if list1.as_ref().map(|n| n.val.clone()) < list2.as_ref().map(|n| n.val.clone()) {
                &mut list1
            } else {
                &mut list2
            };

        let mut node = source.take().unwrap();
        *source = node.next.take();

        *tail = Some(node);
        tail = &mut tail.as_mut().unwrap().next;
    }
    *tail = list1.or(list2);
    head
}

pub fn merge_two_lists_recursive<T: PartialOrd>(
    list1: Option<Box<Node<T>>>,
    list2: Option<Box<Node<T>>>,
) -> Option<Box<Node<T>>> {
    match (list1, list2) {
        (None, list) | (list, None) => list,
        (Some(mut node1), Some(mut node2)) => {
            if node1.val < node2.val {
                node1.next = merge_two_lists_recursive(node1.next.take(), Some(node2));
                Some(node1)
            } else {
                node2.next = merge_two_lists_recursive(Some(node1), node2.next.take());
                Some(node2)
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn list(values: &[i32]) -> Option<Box<Node<i32>>> {
        let mut head = None;

        for &val in values.iter().rev() {
            head = Some(Box::new(Node { val, next: head }));
        }

        head
    }

    fn values(mut head: Option<Box<Node<i32>>>) -> Vec<i32> {
        let mut result = Vec::new();

        while let Some(node) = head {
            result.push(node.val);
            head = node.next;
        }

        result
    }

    #[test]
    fn merges_two_interleaved_lists() {
        let merged = merge_two_lists(list(&[1, 2, 4]), list(&[1, 3, 4]));

        assert_eq!(values(merged), vec![1, 1, 2, 3, 4, 4]);
    }

    #[test]
    fn returns_second_list_when_first_is_empty() {
        let merged = merge_two_lists(list(&[]), list(&[0, 2, 5]));

        assert_eq!(values(merged), vec![0, 2, 5]);
    }

    #[test]
    fn returns_first_list_when_second_is_empty() {
        let merged = merge_two_lists(list(&[-3, 7, 11]), list(&[]));

        assert_eq!(values(merged), vec![-3, 7, 11]);
    }

    #[test]
    fn returns_empty_when_both_lists_are_empty() {
        let merged = merge_two_lists(list(&[]), list(&[]));

        assert_eq!(values(merged), Vec::<i32>::new());
    }

    #[test]
    fn merges_duplicates_and_negative_values() {
        let merged = merge_two_lists(list(&[-10, -3, -3, 4]), list(&[-5, -3, 0, 4, 9]));

        assert_eq!(values(merged), vec![-10, -5, -3, -3, -3, 0, 4, 4, 9]);
    }

    #[test]
    fn appends_remaining_tail_after_one_list_is_exhausted() {
        let merged = merge_two_lists(list(&[1, 2]), list(&[3, 4, 5, 6]));

        assert_eq!(values(merged), vec![1, 2, 3, 4, 5, 6]);
    }
}
