use std::io;
use std::fs;
use std::path::Path;
use std::cell::RefCell;

pub fn read_num_from_file<NumT: std::str::FromStr>(fpath: &Path) -> io::Result<NumT> {
    let data = match fs::read_to_string(fpath) {
        Err(err) => return Err(err),
        Ok(val) => val,
    };
    match data.trim().parse::<NumT>() {
        Err(_) => Err(io::Error::new(io::ErrorKind::InvalidData, "number can't be parsed")),
        Ok(val) => Ok(val),
    }
}

pub struct Cache<T: Copy> {
    cache_: RefCell<Option<T>>,
    generator_: fn() -> T,
}

impl<T: Copy> Cache<T> {
    pub fn new(func: fn() -> T) -> Self {
        Self {
            cache_: RefCell::new(None),
            generator_: func,
        }
    }
    pub fn invalidate(&self) -> () {
        let mut cache = self.cache_.borrow_mut();
        *cache = None;
    }

    pub fn update(&self) -> () {
        let mut cache = self.cache_.borrow_mut();
        *cache = Some((self.generator_)());
    }

    pub fn get(&self) -> Option<T> {
        if self.cache_.borrow().is_none() {
            self.update();
        }
        self.cache_.borrow().clone()
    }
}
