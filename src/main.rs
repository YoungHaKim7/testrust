// todo!() macro
// type aliases
// alias = different name

fn skip_five_take_five(input: Vec<char>) -> std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>> {
    input.into_iter().skip(5).take(5)
}

fn main() {}
