use std::process::Child;

pub struct ChildGuard(pub Child);

impl Drop for ChildGuard {
    fn drop(&mut self) {
        self.0.kill().expect("could not kill child process");
    }
}
