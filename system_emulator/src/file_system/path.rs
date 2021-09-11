use super::{directory::Directory, file::File};

/*
Representation of a path to a file or directory.
Each member of inner is the name of the subsequent directory to traverse before reaching the file/directory.
*/

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Default)]
pub struct Path {
    pub inner: Vec<String>,
}

impl Path {
    pub fn new(inner: Vec<String>) -> Path {
        Path { inner }
    }

    pub fn get_dir<'a>(&self, root: &'a Directory) -> Result<&'a Directory, String> {
        let mut current_dir = root;
        for i in 0..self.inner.len() {
            match current_dir.get_dir(&self.inner[i]) {
                Some(dir) => current_dir = dir,
                None => {
                    return Err(format!(
                        "Directory '{}' is not a child of directory '{}'.",
                        self.inner[i - 1],
                        self.inner[i]
                    ))
                }
            }
        }

        Ok(current_dir)
    }

    pub fn get_file<'a>(&self, root: &'a Directory) -> Result<&'a File, String> {
        let mut current_dir = root;
        for i in 0..self.inner.len() {
            match current_dir.get_dir(&self.inner[i]) {
                Some(dir) => current_dir = dir,
                None => {
                    if i == self.inner.len() - 1 {
                        match current_dir.get_file(&self.inner[i]) {
                            Some(file) => return Ok(file),
                            None => {
                                return Err(format!(
                                    "File '{}' is not a child of directory '{}'.",
                                    self.inner[i],
                                    self.inner[i - 1]
                                ))
                            }
                        }
                    } else {
                        return Err(format!(
                            "Directory '{}' is not a child of directory '{}'.",
                            self.inner[i],
                            self.inner[i - 1]
                        ));
                    }
                }
            }
        }

        unreachable!()
    }

    //extends a path that starts from root with a path that starts at the destination fo the current path
    pub fn extend_from_string(&mut self, path: String) {
        let mut path: Vec<String> = path.split('/').map(|s| s.into()).collect();
        for dir in path.drain(0..) {
            if dir == ".." {
                self.inner.pop();
            } else {
                self.inner.push(dir)
            }
        }
    }
}
