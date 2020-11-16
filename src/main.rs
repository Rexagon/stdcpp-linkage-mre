#[link(name = "mylib", kind = "static")]
extern "C" {
    fn test();
}

fn main() {
    unsafe { test(); }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        unsafe { test(); }

        assert_eq!(2 + 2, 4);
    }
}
