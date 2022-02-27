// todo!() macro
// type aliases
// alias = different name

type SkipAndTake = std::iter::Take<std::iter::Skip<std::vec::IntoIter<char>>> 

fn skip_five_take_five(input: Vec<char>) -> SkipAndTake {
    input.into_iter().skip(5).take(5)
}

fn main() {}
