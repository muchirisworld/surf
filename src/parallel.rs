use std::{
    fs,
    io::BufReader,
    path::PathBuf,
    sync::{Arc, Mutex, mpsc},
    thread,
};

use crate::{matcher, search};

pub struct FileResult {
    pub path: PathBuf,
    pub result: Result<Vec<search::SearchEvent>, String>,
}

pub fn search_parallel(
    paths: Vec<PathBuf>,
    matcher: matcher::Matcher,
    context: search::Context,
    workers: usize,
) -> Vec<FileResult> {
    if workers <= 1 || paths.len() <= 1 {
        return paths
            .into_iter()
            .map(|path| search_single(path, &matcher, context.clone()))
            .collect();
    }

    let matcher = Arc::new(matcher);
    let (work_tx, work_rx) = mpsc::channel::<PathBuf>();
    let (res_tx, res_rx) = mpsc::channel::<FileResult>();
    let work_rx = Arc::new(Mutex::new(work_rx));

    let mut handles = Vec::new();

    for _ in 0..workers {
        let matcher = Arc::clone(&matcher);
        let work_rx = Arc::clone(&work_rx);
        let res_tx = res_tx.clone();
        let context = context.clone();

        handles.push(thread::spawn(move || {
            loop {
                let path = {
                    let rx = work_rx.lock().expect("Work receiver poisoned");
                    rx.recv()
                };

                match path {
                    Ok(path) => {
                        let res = search_single(path, &matcher, context.clone());
                        if res_tx.send(res).is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        }));
    }
    drop(res_tx);

    for path in paths {
        if work_tx.send(path).is_err() {
            break;
        }
    }
    drop(work_tx);

    let mut results = Vec::new();
    for res in res_rx {
        results.push(res);
    }

    for handle in handles {
        handle.join().expect("Worker panicked");
    }

    results.sort_by(|left, right| left.path.cmp(&right.path));
    results
}

fn search_single(
    path: PathBuf,
    matcher: &matcher::Matcher,
    context: search::Context,
) -> FileResult {
    let result = fs::File::open(&path)
        .map_err(|err| format!("failed to open {}: {err}", path.display()))
        .and_then(|file| {
            let reader = BufReader::new(file);
            search::search_reader(reader, matcher, context)
                .map_err(|err| format!("failed to read {}: {err}", path.display()))
        });

    FileResult { path, result }
}
