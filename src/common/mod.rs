/// This is made for common functions which can be used everywhere
///

#[allow(dead_code)]
pub fn repeat_in_place<T, F>(mut f: F, mut argument: T, times: u32) -> T
where
    F: FnMut(T) -> T,
{
    for _ in 0..times {
        argument = f(argument);
    }
    argument
}
