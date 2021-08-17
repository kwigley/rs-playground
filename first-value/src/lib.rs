#[cfg(test)]
fn first_value_a<A: Copy>(v: Vec<Option<A>>, default: A) -> A {
    for o in v {
        match o {
            Some(a) => return a,
            None => (),
        }
    }

    default
}

// This is pretty much your solution with slightly different syntax
#[cfg(test)]
fn first_value_b<A: Copy>(v: Vec<Option<A>>, default: A) -> A {
    let flat: Vec<&A> = v.iter().flatten().collect();
    match flat[..] {
        [] => default,
        _ => *flat[0],
    }
}
#[cfg(test)]
fn first_value_c<A: Copy>(v: Vec<Option<A>>, default: A) -> A {
    **v.iter()
        .flatten()
        .collect::<Vec<&A>>()
        .first()
        .unwrap_or(&&default)
}
#[cfg(test)]
fn first_value_d<A: Copy>(v: Vec<Option<A>>, default: A) -> A {
    v.iter().rev().fold(default, |x, elem| elem.unwrap_or(x))
}

#[cfg(test)]
fn first_value_e<A: Copy>(v: Vec<Option<A>>, default: A) -> A {
    v.first().map_or(default, |elem| {
        elem.unwrap_or_else(|| first_value_e(v[1..].to_vec(), default))
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_a() {
        assert_eq!(first_value_a(vec![None, Some(1), Some(2)], 100), 1)
    }
    #[test]
    fn it_works_b() {
        assert_eq!(first_value_b(vec![None, Some(1), Some(2)], 100), 1)
    }
    #[test]
    fn it_works_c() {
        assert_eq!(first_value_c(vec![None, Some(1), Some(2)], 100), 1)
    }
    #[test]
    fn it_works_d() {
        assert_eq!(first_value_d(vec![None, Some(1), Some(2)], 100), 1)
    }
    #[test]
    fn it_works_e() {
        assert_eq!(first_value_e(vec![None, Some(1), Some(2)], 100), 1)
    }
}
