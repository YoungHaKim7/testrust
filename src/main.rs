// () - empty tuple, unit type (void)
// expression-based languge

fn empty_tuple() {}

// Display {}
// Debug print 디버그는 type이름과 properties 프로그램을 보기 위한 print
fn main() {
    let tuple = empty_tuple();
    println!("{:?}", tuple);
}
