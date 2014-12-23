fn main() {

  for x in range(std::u32::MIN, std::u32::MAX){
    print!("{}", std::char::from_u32(x).unwrap());
  }

}
