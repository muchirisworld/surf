use std::{collections::VecDeque, fs, io, path::PathBuf};

pub struct WalkOptions {
    pub recursive: bool,
}

pub struct WorkItem {
    pub path: PathBuf,
}

pub fn collect_files(paths: &[PathBuf], options: &WalkOptions) -> io::Result<Vec<WorkItem>> {
    let mut files = Vec::new();
    let mut queue: VecDeque<PathBuf> = paths.iter().cloned().collect();

    while let Some(path) = queue.pop_front() {
        let metadata = fs::metadata(&path)?;

        if metadata.is_file() {
            files.push(WorkItem { path });
        } else if metadata.is_dir() && options.recursive {
            push_dir_entries(&path, &mut queue)?;
        }
    }

    Ok(files)
}

fn push_dir_entries(path: &PathBuf, queue: &mut VecDeque<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let file = entry?;
        queue.push_back(file.path());
    }

    Ok(())
}
