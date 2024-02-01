use std::{fs, io};
use std::path::Path;

pub fn copy_by_condition<T>(from: impl AsRef<Path>, to: impl AsRef<Path>, condition: T)
  where
    T: Fn(&str) -> bool + Sized + 'static,
{
  use std::fs;
  let entries = from.as_ref().read_dir().unwrap();
  for entry in entries {
    let entry = entry.unwrap();
    if entry.file_type().unwrap().is_dir() {
      continue;
    }

    let file_name = entry.file_name();

    if !condition(file_name.to_str().unwrap()) {
      continue;
    }
    let from = entry.path();
    let to = to.as_ref().join(file_name);
    println!("copy from {from:#?} to: {to:#?}");
    fs::copy(&from, &to).unwrap();
  }
}


pub fn read_dir_by_condition<T>(dir: &Path, condition: T) -> Vec<String>
  where
    T: Fn(&str) -> bool + Sized + 'static,
{
  let mut res = Vec::new();
  let entries = dir.read_dir().unwrap();
  for entry in entries {
    let entry = entry.unwrap();
    if entry.file_type().unwrap().is_dir() {
      continue;
    }

    let file_name = entry.file_name();
    let file_name = file_name.to_str().unwrap();

    if !condition(file_name) {
      continue;
    }

    res.push(dir.join(file_name).to_str().unwrap().to_string());
  }

  res
}


pub fn create_folder_if_not_exists(folder_path: &Path) -> Result<(), io::Error> {
  if !fs::metadata(folder_path).is_ok() {
    fs::create_dir(folder_path)
  } else {
    Ok(())
  }
}
