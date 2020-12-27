use std::io;
use std::fs;
use std::path::Path;

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
