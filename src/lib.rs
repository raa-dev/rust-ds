pub mod linked_lists;
#[cfg(test)]
mod tests {
    use crate::linked_lists::traits::List;

    #[test]
    fn singly_linked_list_test() {
        use crate::linked_lists::singly::Singly;

        let mut list = Singly::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
        list.insert(5);
        assert_eq!(list.search(&3).unwrap(), true);
        assert_eq!(list.search(&6).unwrap(), false);
        assert_eq!(list.update(3, 6).unwrap(), true);
        assert_eq!(list.is_empty(),false);
        assert_eq!(list.len(),5);
        let res = list.get(2).unwrap();
        assert_eq!(res.unwrap(),&6);
    }

    #[test]
    fn double_linked_list_test() {
        use crate::linked_lists::double::Double;

        let mut list = Double::new();
        list.insert(1);
        list.insert(2);
        list.insert(3);
        list.insert(4);
        list.insert(5);
        assert_eq!(list.search(&3).unwrap(), true);
        assert_eq!(list.search(&6).unwrap(), false);
        assert_eq!(list.update(3, 6).unwrap(), true);
        assert_eq!(list.is_empty(),false);
        assert_eq!(list.len(),5);
        let res = list.get(2).unwrap();
        assert_eq!(res.unwrap(),&6);
    }
}

