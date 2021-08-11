use std::fmt::Debug;

fn main() {
    first_value(vec![Some(1), Some(2), None], 100);
}

fn first_value<A: Copy + Debug>(values: Vec<Option<A>>, default: A) -> A {
    // using flatten
    values
        .into_iter()
        .flatten()
        .collect::<Vec<_>>()
        .first()
        .map_or(default, |x| *x)

    // using filter_map
    // values
    //     .into_iter()
    //     .filter_map(|x| x)
    //     .collect::<Vec<_>>()
    //     .first()
    //     .map_or(default, |x| *x)

    //using filter
    // values
    //     .into_iter()
    //     .filter(|x| x.is_some())
    //     .collect::<Vec<_>>()
    //     .first()
    //     .map_or(default, |x| x.unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn foo() {
        assert_eq!(first_value(vec![Some(1), Some(2), None], 100), 1);
        assert_eq!(first_value(vec![None, None, Some(1), Some(2)], 100), 1);
    }
}
