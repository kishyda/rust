#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl std::fmt::Display for ListNode {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        let mut current = Some(self);
        while let Some(node) = current {
            write!(f, "{}", node.val)?;
            current = node.next.as_deref();
            if current.is_some() {
                write!(f, " -> ")?;
            }
        }
        Ok(())
    }
}

impl ListNode {
    //#[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
    fn add(self, value: i32) -> ListNode {
        ListNode { 
            val: value, 
            next: Some(Box::new(self))
        }
    }
}
pub fn test(){
    let mut l1 = ListNode::new(2);
    l1 = l1.add(4);
    l1 = l1.add(3);
    let mut l2 = ListNode::new(5);
    l2 = l2.add(6);
    l2 = l2.add(4);
    let l3 = add_two_numbers(Some(Box::new(l1)), Some(Box::new(l2)));
    let l3 = match l3 {
        Some(value) => value,
        _ => panic!("Error"),
    };
    println!("{}", l3);
}
pub fn add_two_numbers(l1: Option<Box<ListNode>>, l2: Option<Box<ListNode>>) -> Option<Box<ListNode>>{
    let mut l1 = l1;
    let mut l2 = l2;
    let mut carry = 0;
    let mut dummy_head = ListNode::new(0);
    let mut current = &mut dummy_head;

    while l1.is_some() || l2.is_some() || carry != 0 {
        let sum = carry 
            + l1.as_ref().map_or(0, |node| node.val)
            + l2.as_ref().map_or(0, |node| node.val);

        carry = sum / 10;
        current.next = Some(Box::new(ListNode::new(sum % 10)));
        current = current.next.as_mut().unwrap();

        if let Some(node) = l1 {
            l1 = node.next;
        }
        if let Some(node) = l2 {
            l2 = node.next;
        }
    }

    dummy_head.next
}

pub fn access_value(l: Option<Box<ListNode>>, string: &mut String) {
    match l {
        None => (),
        Some(value) => {
            let character: char = char::from_u32(value.val as u32).expect("Error");
            string.push(character);
            access_value(value.next, string)
        }
    }
}
