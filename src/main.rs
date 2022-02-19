use std::cell::Cell;
// Cell<T>
// Cell - small room
// get - Cell을 가지고 옴
// replace
// set - Sets the contained value.
// Cell 은 멀티풀 Reference에 넣을 수 없다. 안전하기 때문에
// Cell속에 데이터가 들어가면 그 안에 데이터는 만질 수 없다.
// Cell만 &Cell 이렇게 Reference가 가능하다!!

#[derive(Debug)]
struct PhoneModel {
    company_name: String,
    model_name: String,
    screen_size: f32,
    memory: usize,
    date_issued: u32,
    on_sale: Cell<bool>,
}

fn main() {
    let super_phone_3000 = PhoneModel {
        company_name: "YY Electronics".to_string(),
        model_name: "Super Phone 3000".to_string(),
        screen_size: 7.5,
        memory: 4_000_000,
        date_issued: 2020,
        on_sale: Cell::new(true),
    };
    println!("{super_phone_3000:?}");

    super_phone_3000.on_sale.set(false);

    println!("{super_phone_3000:?}");
}
