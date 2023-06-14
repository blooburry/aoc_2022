use std::io::Write;
use assert_cmd::Command;
use tempfile::NamedTempFile;

use rstest::rstest;

#[rstest]
#[case("mjqjpqmgbljsphdztnvjfqwrcgsmlb", 19)]
#[case("bvwbjplbgvbhsrlpgdmjqwftvncz", 23)]
#[case("nppdvjthqldpwncqszvftbrmjlhg", 23)]
#[case("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", 29)]
#[case("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", 26)]
fn simple_message(#[case] message: &str, #[case] start_index: u32) -> Result<(), Box<dyn std::error::Error>> {
    let mut temp_file = NamedTempFile::new_in("rsrc")?;
    let file_path = temp_file.path().to_str().ok_or("Path couldn't be converted to &str")?.to_owned();

    write!(temp_file, "{}", message)?;

    let mut cmd = Command::cargo_bin("day_6")?;
    cmd.arg(file_path);

    cmd.assert().stdout(predicates::str::contains(format!("Message starts at index {}", start_index)));

    Ok(())
}
