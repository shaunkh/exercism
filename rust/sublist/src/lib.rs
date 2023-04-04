#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

pub fn sublist<T: PartialEq>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if _first_list.len() == 0 && _second_list.len() != 0 {
        return Comparison::Sublist;
    } else if _second_list.len() == 0 && _first_list.len() != 0 {
        return Comparison::Superlist;
    }

    if _first_list.len() == _second_list.len() {
        for n in 0.._second_list.len() {
            if _first_list[n] != _second_list[n] {
                return Comparison::Unequal;
            }
        }
        return Comparison::Equal;
    } else if _first_list.len() < _second_list.len() {
        let diff = _second_list.len() - _first_list.len();

        for n in 0..diff + 1 {
            if (_first_list[0] == _second_list[n]) {
                let mut i = 0;
                while i < _first_list.len() {
                    if _first_list[i] != _second_list[n + i] {
                        break;
                    }
                    i += 1;
                }
                if i == _first_list.len() {
                    return Comparison::Sublist;
                }
            }
        }
    }
    return Comparison::Unequal;
}
