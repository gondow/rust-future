fn main() {
   let mut iter = 0..3;
   println!("{:?}", iter.next()); // Some(0)
   println!("{:?}", iter.next()); // Some(1)
   println!("{:?}", iter.next()); // Some(2)
   println!("{:?}", iter.next()); // None
}

