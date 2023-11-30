use std::str::{pattern::Pattern, FromStr};

pub fn to_vec<'a, T, P>(s: &'a str, pat: P) -> Vec<T>
where
    T: FromStr,
    P: Pattern<'a>,
{
    s.split(pat).filter_map(|x| x.parse().ok()).collect()
}

pub fn to_vec_map<'a, T, U, P>(s: &'a str, pat: P, func: impl FnMut(T) -> U) -> Vec<U>
where
    T: FromStr,
    P: Pattern<'a>,
{
    s.split(pat)
        .filter_map(|x| x.parse().ok())
        .map(func)
        .collect()
}
