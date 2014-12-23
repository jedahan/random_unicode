fn main() {

  for u in range(std::u32::MIN, std::char::MAX as u32) {
    match std::char::from_u32(u) {
      Some(x) => print!("{}", x),
      _ => print!("")
    };
  }

}
