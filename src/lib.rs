/*
extern crate byteorder;
extern crate memmem;
*/
pub mod parser;
pub mod editor;
pub mod writer;
pub mod section;
pub mod segment;
pub mod header;


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}

