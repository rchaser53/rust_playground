// fn main() {
//   thread_local!(static FOO: RefCell<u32> = RefCell::new(1));

//   FOO.with(|f| {
//       assert_eq!(*f.borrow(), 1);
//       *f.borrow_mut() = 2;
//   });

//   thread::spawn(move|| {
//       FOO.with(|f| {
//           assert_eq!(*f.borrow(), 1);
//           *f.borrow_mut() = 3;

//           println!("{}", f.borrow())    // 3
//       });
//   });

//   FOO.with(|f| {
//     assert_eq!(*f.borrow(), 2);
//   });
// }
