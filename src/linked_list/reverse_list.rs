// Definition for singly-linked list.
pub struct List {
    head: Option<Box<ListNode>>,
}

impl Default for List {
    fn default() -> Self {
        Self::new()
    }
}

impl List {
    pub fn new() -> Self {
        List { head: None }
    }

    pub fn iter(&self) -> Iter {
        Iter {
            next: self.head.as_deref(),
        }
    }

    pub fn from_head(head: Option<Box<ListNode>>) -> Self {
        Self { head }
    }

    pub fn push(&mut self, val: i32) {
        let new_node = Box::new(ListNode::new(val));

        let mut current = &mut self.head;

        while let Some(node) = current {
            current = &mut node.next;
        }

        *current = Some(new_node);
    }

    // # Definition for singly-linked list.
    // # class ListNode:
    // #     def __init__(self, val=0, next=None):
    // #         self.val = val
    // #         self.next = next
    // class Solution:
    //     def reverseList(self, head: Optional[ListNode]) -> Optional[ListNode]:
    //         prev, curr = None, head

    //         while curr:
    //             next_node = curr.next
    //             curr.next = prev
    //             prev = curr
    //             curr = next_node

    //         return prev
    pub fn reverse(self) -> Self {
        let (mut prev, mut curr) = (None, self.head);

        while let Some(mut node) = curr {
            curr = node.next;

            node.next = prev;

            prev = Some(node);
        }

        Self::from_head(prev)
    }
}

pub struct Iter<'a> {
    next: Option<&'a ListNode>,
}

impl Iterator for Iter<'_> {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        self.next.map(|node| {
            self.next = node.next.as_deref();
            node.val
        })
    }
}

impl FromIterator<i32> for List {
    fn from_iter<I: IntoIterator<Item = i32>>(iter: I) -> Self {
        let mut l = List::default();

        for i in iter {
            l.push(i);
        }

        l
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

pub struct Solution;

impl Solution {
    /// # [Reverse Linked List](https://leetcode.com/problems/reverse-linked-list/description/)
    pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let list = List::from_head(head);
        list.reverse().head
    }
}

#[cfg(test)]
mod tests {
    use super::List;

    fn linked_list_from_i32_slice(list: &[i32]) -> List {
        list.iter().cloned().collect()
    }

    #[test]
    fn test_linked_list_head() {
        let linked_list = linked_list_from_i32_slice(&[1, 2, 3, 4, 5]);
        insta::assert_debug_snapshot!(linked_list.head.unwrap(), @r###"
        ListNode {
            val: 1,
            next: Some(
                ListNode {
                    val: 2,
                    next: Some(
                        ListNode {
                            val: 3,
                            next: Some(
                                ListNode {
                                    val: 4,
                                    next: Some(
                                        ListNode {
                                            val: 5,
                                            next: None,
                                        },
                                    ),
                                },
                            ),
                        },
                    ),
                },
            ),
        }
        "###);
    }

    #[test]
    fn reverse_list() {
        let list = linked_list_from_i32_slice(&[1, 2, 3, 4, 5]);
        insta::assert_debug_snapshot!(list.head, @r###"
        Some(
            ListNode {
                val: 1,
                next: Some(
                    ListNode {
                        val: 2,
                        next: Some(
                            ListNode {
                                val: 3,
                                next: Some(
                                    ListNode {
                                        val: 4,
                                        next: Some(
                                            ListNode {
                                                val: 5,
                                                next: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
        )
        "###);

        insta::assert_debug_snapshot!(list.reverse().head, @r###"
        Some(
            ListNode {
                val: 5,
                next: Some(
                    ListNode {
                        val: 4,
                        next: Some(
                            ListNode {
                                val: 3,
                                next: Some(
                                    ListNode {
                                        val: 2,
                                        next: Some(
                                            ListNode {
                                                val: 1,
                                                next: None,
                                            },
                                        ),
                                    },
                                ),
                            },
                        ),
                    },
                ),
            },
        )
        "###);
    }
}
