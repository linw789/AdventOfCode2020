use std::rc::Rc;
use std::cell::RefCell;

struct Cup {
    label: u8,
    next: Option<Rc<RefCell<Cup>>>,
}

impl PartialEq for Cup {
    fn eq(&self, other: &Self) -> bool {
        return self.label == other.label;
    }
}

impl Cup {
    pub fn new(label: u8) -> Self {
        return Self {
            label,
            next: None,
        };
    }

    pub fn next(&self) -> Rc<RefCell<Cup>> {
        return Rc::clone(self.next.as_ref().unwrap());
    }

    pub fn link(&mut self, cup: Rc<RefCell<Cup>>) {
        self.next = Some(cup);
    }

    pub fn unlink_next(&mut self) -> Rc<RefCell<Cup>> {
        let next = Rc::clone(self.next.as_ref().unwrap()); 
        self.next = Some(Rc::clone(next.borrow().next.as_ref().unwrap()));
        return next;
    }
}

fn find_cup_by_label(start: Rc<RefCell<Cup>>, label: u8) -> Option<Rc<RefCell<Cup>>> {
    let first_label = start.borrow().label;
    if label == first_label {
        return Some(start);
    }

    let mut node = Rc::clone(start.borrow().next.as_ref().unwrap());
    loop {
        if label == node.borrow().label {
            return Some(node);
        }
        if node.borrow().label == first_label {
            return None;
        }
        node = {
            let x = Rc::clone(node.borrow().next.as_ref().unwrap());
            x
        };
    }
}

fn part_1(start: Rc<RefCell<Cup>>) -> String {
    let mut curr = start;
    for _ in 0..100 {
        let pick1 = { 
            curr.borrow_mut().unlink_next()
        };
        let pick2 = {
            curr.borrow_mut().unlink_next()
        };
        let pick3 = {
            curr.borrow_mut().unlink_next()
        };

        let mut dest_label = curr.borrow().label - 1;
        loop {
            if dest_label == 0 {
                dest_label = 9;
            }
            if dest_label != pick1.borrow().label && 
               dest_label != pick2.borrow().label && 
               dest_label != pick3.borrow().label {
                break;
            }
            dest_label -= 1;
        }
        let dest = find_cup_by_label(Rc::clone(&curr), dest_label).unwrap();
        {  
            dest.borrow_mut().link(pick1);
        }
        {  
            dest.borrow_mut().link(pick2);
        }
        {  
            dest.borrow_mut().link(pick3);
        }

        curr = { 
            let x = curr.borrow().next();
            x
        };
    }

    let cup_1 = find_cup_by_label(Rc::clone(&curr), 1).unwrap();
    let mut res: String = "".to_string();
    let mut node = cup_1.borrow().next();
    loop {
        if node.borrow().label == 1 {
            break;
        }
        {
            res += &node.borrow().label.to_string();
        }
        node = {
            let x = node.borrow().next();
            x
        };
    }

    return res;
}

fn main() {
    let cups = vec![7, 3, 9, 8, 6, 2, 5, 4, 1];
    
    let start = Rc::new(RefCell::new(Cup::new(cups[0])));
    let mut last = Rc::clone(&start);
    for label in cups.iter().skip(1) {
        let next = Rc::new(RefCell::new(Cup::new(*label)));
        last.borrow_mut().link(Rc::clone(&next));
        last = next;
    }
    last.borrow_mut().link(Rc::clone(&start));

    /*
    let mut cup = Rc::clone(&start);
    loop {
        cup = {
            let x = Rc::clone(cup.borrow().next.as_ref().unwrap());
            x
        };
        if cup == start {
            break;
        }
    }
    */

    println!("Part 1: {}", part_1(start));
}
