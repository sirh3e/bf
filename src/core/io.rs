use std::path::Path;

pub struct Reader;

impl Reader {
    pub fn read_from_file<'a, T: AsRef<Path>>(path: &'a str) -> String {
        unimplemented!()
    }
}