use crate::stitch::HalfStitch;
use std::path::{Path, PathBuf};

pub fn write_sequence_to_file(sequence: &Vec<HalfStitch>, file_name: &PathBuf) -> () {
    let file = Path::new(&file_name);
    let mut writer = csv::Writer::from_path(file).expect("Make CSV writer");
    for stitch in sequence {
        writer.serialize(stitch).expect("");
    }
}

#[cfg(test)]
mod test {
    use crate::csv_writer::write_sequence_to_file;
    use std::fs::remove_file;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_write() {
        let test_stitches = crate::stitch::make_full_stitch(1, 1).to_vec();
        let file_name = PathBuf::from("test.csv");
        write_sequence_to_file(&test_stitches, &file_name);
        remove_file(Path::new(&file_name)).expect("");
    }
}
