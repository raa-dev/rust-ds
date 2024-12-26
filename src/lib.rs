pub mod linked_lists;
#[cfg(test)]
mod tests {
    #[test]
    fn linked_list_test() {
        use crate::linked_lists::traits::List;
        use crate::linked_lists::singly::Singly;

        let mut list = Singly::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
        list.insert(5);
        list.print();
        assert_eq!(list.search(&3).unwrap(), true);
        assert_eq!(list.search(&6).unwrap(), false);
        assert_eq!(list.update(3, 6).unwrap(), true);
        assert_eq!(list.is_empty(),false);
    }
}

