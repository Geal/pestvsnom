extern crate gcc;

fn main() {
    gcc::Build::new()
               .file("src/ultrajsondec.c")
               .file("src/ujdecode.c")
               .compile("ujson4c");
}
