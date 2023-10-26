#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

fn are_lists_equal<T: Clone + Copy + PartialEq>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.len() != second_list.len() {
        return false;
    }
    if first_list.is_empty() && second_list.is_empty() {
        return true;
    }
    let equal_elements: Vec<(&T, &T)> = first_list
        .into_iter()
        .zip(second_list)
        .filter(|(el1, el2)| *el1 == *el2)
        .collect();
    if equal_elements.len() == first_list.len() {
        return true;
    }
    false
}
fn is_first_list_sublist<T: Clone + PartialEq + Copy>(first_list: &[T], second_list: &[T]) -> bool {
    if first_list.is_empty() {
        return true;
    }
    let sublist_len = first_list.len();
    let first_sublist_char = first_list.get(0).unwrap();

    for (i, el) in second_list.into_iter().enumerate() {
        if el == first_sublist_char {
            let second_list_potentially_equal_sublist: Vec<&T> =
                second_list.into_iter().skip(i).take(sublist_len).collect();
            let sublist: Vec<&T> = first_list.into_iter().collect();
            if sublist == second_list_potentially_equal_sublist {
                return true;
            }
        }
    }
    false
}

fn is_first_list_superlist<T: Clone + Copy + PartialEq>(
    first_list: &[T],
    second_list: &[T],
) -> bool {
    is_first_list_sublist(second_list, first_list)
}

pub fn sublist<T: PartialEq + Copy>(_first_list: &[T], _second_list: &[T]) -> Comparison {
    if are_lists_equal(_first_list, _second_list) {
        return Comparison::Equal;
    };
    if is_first_list_sublist(_first_list, _second_list) {
        return Comparison::Sublist;
    }
    if is_first_list_superlist(_first_list, _second_list) {
        return Comparison::Superlist;
    }
    Comparison::Unequal
}
