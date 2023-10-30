#[macro_export]
macro_rules! pointerchain {
    ($result: ident, $( $x:expr ),* ) => {
        unsafe {
            let mut current = 0;
            $(current = *((current + ($x as usize)) as *const usize);)*
            &*(current as *const $result)
        }
    };
}
