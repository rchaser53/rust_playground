pub fn read_and_write() {
  let f = File::open("input.txt").unwrap();
  let mut buf_reader = BufReader::new(&f);
  let mut buffer = Vec::new();
  buf_reader.read_to_end(&mut buffer).unwrap();
  println!("{}", std::str::from_utf8(&buffer).unwrap());
}