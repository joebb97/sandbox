use std::collections::VecDeque;
use std::io::Write;
use std::path::{Component, Path, PathBuf};

pub struct FileSystem {
    pub root: TreeNode,
}

#[derive(PartialEq, Debug)]
pub enum TreeNode {
    Directory {
        name: PathBuf,
        children: Vec<TreeNode>,
    },
    File {
        name: PathBuf,
        data: Vec<u8>,
    },
}

impl TreeNode {
    pub fn name(&self) -> &Path {
        match self {
            TreeNode::Directory { name, .. } => name,
            TreeNode::File { name, .. } => name,
        }
    }

    pub fn children(&mut self) -> Option<&mut Vec<TreeNode>> {
        match self {
            TreeNode::Directory { children, .. } => Some(children),
            TreeNode::File { .. } => None,
        }
    }
}

impl FileSystem {
    pub fn new() -> Self {
        Self {
            root: TreeNode::Directory {
                name: "".into(),
                children: Vec::new(),
            },
        }
    }

    pub fn find_node(&mut self, query: &Path) -> Option<&mut TreeNode> {
        let mut cur_node = Some(&mut self.root);
        let components = query.components();
        for component in components {
            let Component::Normal(part) = component else {
                continue;
            };
            let Some(TreeNode::Directory { children, .. }) = cur_node else {
                return None;
            };
            cur_node = children.iter_mut().find(|c| c.name() == part);
        }
        cur_node
    }

    pub fn find_node1(&mut self, query: &Path) -> Option<&mut TreeNode> {
        let mut queue = VecDeque::new();
        let node = &mut self.root;
        queue.push_back((node, PathBuf::new()));
        let query: PathBuf = query
            .components()
            .filter(|c| !matches!(c, Component::RootDir))
            .collect();
        while let Some((node, path)) = queue.pop_front() {
            if path == query {
                return Some(node);
            }
            let TreeNode::Directory { name: _, children } = node else {
                continue;
            };
            for child in children {
                // assume path ends in /
                let next_path = path.join(child.name());
                queue.push_back((child, next_path));
            }
        }
        None
    }

    pub fn mkdir(&mut self, path: &Path) -> Option<&mut TreeNode> {
        let new_path = path.parent().unwrap();
        let new_name = path.file_name().unwrap();
        dbg!(path, new_path, new_name);
        dbg!();
        let node = self.find_node(new_path);
        let Some(TreeNode::Directory { children, .. }) = node else {
            return None;
        };
        let new_node = TreeNode::Directory {
            name: new_name.into(),
            children: Vec::new(),
        };
        children.push(new_node);
        children.last_mut()
    }

    pub fn write(&mut self, path: &Path, new_data: &[u8]) -> usize {
        let parent = path.parent().unwrap();
        let fname = path.file_name().unwrap();
        let mut node = self.find_node(parent);
        let Some(TreeNode::Directory { children, .. }) = &mut node else {
            return 0;
        };
        let mut child = children.iter_mut().find(|c| c.name() == fname);
        if child.is_none() {
            let new = TreeNode::File {
                name: fname.into(),
                data: vec![],
            };
            children.push(new);
            child = children.last_mut();
        }
        let Some(TreeNode::File { data, .. }) = child else {
            return 0;
        };
        data.truncate(0);
        data.write_all(new_data).unwrap();
        new_data.len()
    }

    pub fn read(&mut self, path: &Path) -> Option<Vec<u8>> {
        let node = self.find_node(path);
        let Some(TreeNode::File { data, .. }) = node else {
            return None;
        };
        Some(data.clone())
    }
}

impl Default for FileSystem {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::problems::filesystem::TreeNode;

    use super::FileSystem;

    #[test]
    fn test() {
        let mut fs = FileSystem::new();
        fs.mkdir(Path::new("/usr/"));
        assert_eq!(
            fs.root,
            TreeNode::Directory {
                name: "".into(),
                children: vec![TreeNode::Directory {
                    name: "usr".into(),
                    children: vec![]
                }]
            }
        );
        fs.mkdir(Path::new("/usr/local/"));
        assert_eq!(
            fs.root,
            TreeNode::Directory {
                name: "".into(),
                children: vec![TreeNode::Directory {
                    name: "usr".into(),
                    children: vec![TreeNode::Directory {
                        name: "local".into(),
                        children: vec![]
                    }]
                }]
            }
        );
        fs.mkdir(Path::new("/usr/local/bin"));
        fs.write(Path::new("/usr/local/bin/a.sh"), b"aoeu");

        assert_eq!(
            fs.read(Path::new("/usr/local/bin/a.sh")),
            Some(b"aoeu".to_vec())
        );

        fs.write(Path::new("/usr/local/bin/a.sh"), b"xyzd");

        assert_eq!(
            fs.read(Path::new("/usr/local/bin/a.sh")),
            Some(b"xyzd".to_vec())
        );

        assert_eq!(fs.read(Path::new("/usr/local/bin/b.sh")), None);
    }
}
