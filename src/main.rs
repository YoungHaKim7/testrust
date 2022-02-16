// String
// &str

// string literal 프로그램이 존재하는 동안 있는다 아주 간단한 스트링
// borrowed str

fn main() {
    let my_name = "David"; // 'static string이라 계속 살아 있는다.

    {
        let my_string = "David".to_string(); // &'static - for the life of the program
        let my_string_ref = &my_string; // &str - reference to something else
    }
}
