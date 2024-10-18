/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T:PartialOrd>(array: &mut [T]){
	//TODO
    if array.len() <= 1 {
        return;
    }

    let sz = array.len();
    for i in 0..(sz-1) { // 一共进行sz-1轮
        let mut flag = false;

        for j in 1..(sz-i) {// 后i个已经排序好
            if (array[j - 1] > array[j]) {
                array.swap(j-1, j);
                flag = true;
            }
        }
        if !flag {
            break;
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}