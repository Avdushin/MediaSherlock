#[cfg(target_os = "windows")]
extern crate winres;

fn main() {
  let mut res = winres::WindowsResource::new();
  res.set_icon("icon.ico");
  res.compile().unwrap();
}