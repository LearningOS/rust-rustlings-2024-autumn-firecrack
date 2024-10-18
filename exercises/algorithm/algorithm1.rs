/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T: Ord> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T: Ord> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }
	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
	{
		if list_a.start.is_none() {
            return list_b;
        }
        if list_b.start.is_none() {
            return list_a;
        }
        
        //TODO
		let mut list = LinkedList::<T>::new();
        let mut cur_a = list_a.start; // option
        let mut cur_b = list_b.start;

        unsafe {
            if (*cur_a.unwrap().as_ptr()).val <= (*cur_b.unwrap().as_ptr()).val {
                list.start = cur_a;  // 将 list_a 的头节点作为新链表的头节点
                list.end = cur_a;    
                cur_a = (*cur_a.unwrap().as_ptr()).next.take(); // 移动 cur_a 到下一个节点
            } else {
                list.start = cur_b;  // 将 list_b 的头节点作为新链表的头节点
                list.end = cur_b;    
                cur_b = (*cur_b.unwrap().as_ptr()).next.take(); // 移动 cur_b 到下一个节点
            }
        }
        while let (Some(mut a_ptr), Some(mut b_ptr)) = (cur_a, cur_b) {
            let val_a = unsafe {&(*a_ptr.as_ptr()).val};
            let val_b = unsafe {&(*b_ptr.as_ptr()).val};
            
            if val_a <= val_b {
                unsafe {
                    cur_a = (*a_ptr.as_ptr()).next.take(); // 更新 cur_a
                    (*list.end.unwrap().as_ptr()).next = Some(a_ptr); // 将 a_ptr 移动到新链表
                }
                list.end = Some(a_ptr); // 更新链表末尾
            } else {
                unsafe {
                    cur_b = (*b_ptr.as_ptr()).next.take(); // 更新 cur_b
                    (*list.end.unwrap().as_ptr()).next = Some(b_ptr); // 将 b_ptr 移动到新链表
                }
                list.end = Some(b_ptr); // 更新链表末尾
            }
        
        }

        // 如果 cur_a 有剩余的节点，移动到新链表
        if let Some(mut a_ptr) = cur_a {
            unsafe {
                (*list.end.unwrap().as_ptr()).next = Some(a_ptr); // 将剩余的 cur_a 节点接入新链表
            }
            list.end = list_a.end; // 更新 list.end
        }

        // 如果 cur_b 有剩余的节点，移动到新链表
        if let Some(mut b_ptr) = cur_b {
            unsafe {
                (*list.end.unwrap().as_ptr()).next = Some(b_ptr); // 将剩余的 cur_b 节点接入新链表
            }
            list.end = list_b.end; // 更新 list.end
        }

        list
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            // 通过递归的方式来实现对链表的遍历
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}