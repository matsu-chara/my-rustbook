pub mod toy_vec;

#[derive(Debug)]
struct Parent(usize, Child, Child);

impl Drop for Parent {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

#[derive(Debug)]
struct Child(usize);
impl Drop for Child {
    fn drop(&mut self) {
        println!("Dropping {:?}", self);
    }
}

fn main() {
    let p1 = Parent(1, Child(11), Child(12));
    {
        let p2 = Parent(2, Child(21), Child(22));
        println!("{:?}, {:?}", p1, p2);
    }

    println!("{:?}", p1);
    let p3 = Parent(3, Child(31), Child(32));
    println!("{:?}, {:?}", p1, p3);
}
