// closure는 주변의 변수를 가질 수 가 있음. 최고!!
fn fn_closure<F>(f: F)
where
    F: Fn(),
{
    f();
}

fn fn_mut_closure<F>(mut f: F)
where
    F: FnMut(),
{
    f();
}

fn fn_once_closure<F>(f: F)
where
    F: FnOnce(),
{
    f();
}

fn main() {
    let mut my_string = String::from("Hello there");

    fn_once_closure(|| {
        my_string.push('a');
        println!("{my_string}");
        drop(my_string);
    });
}
