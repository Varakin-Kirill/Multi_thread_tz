pub const THRESHOLD: usize = 3;
pub const ITERATIONS: u32 = 8;

pub fn multi_thread<F, R, T>(data: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(T) -> R + std::marker::Send + Copy + 'static,
    T: Sync + Send + 'static + Clone,
    R: Send + 'static,
{
    use std::thread;

    if data.len() > THRESHOLD {
        data.chunks(THRESHOLD)
            .flat_map(|chunk| {
                let chunk = chunk.to_vec();
                thread::spawn(move || chunk.into_iter().map(|item| f(item)).collect::<Vec<R>>())
                    .join()
                    .unwrap()
            })
            .collect::<Vec<R>>()
    } else {
        data.into_iter().map(|item| f(item)).collect()
    }
}

pub fn converting(mut n: u32) -> u32 {
    let mut counter: u32 = 0;

    while n != 1 {
        n = if n % 2 == 0 { n / 2 } else { n * 3 + 1 };
        counter += 1;
        if counter == ITERATIONS {
            return n;
        }
    }
    counter
}

pub async fn multi_thread_tokio<F, R, T>(data: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(T) -> R + std::marker::Send + Copy + 'static,
    T: Sync + Send + 'static + Clone,
    R: Send + 'static,
{
    use tokio::task::JoinHandle;

    if data.len() > THRESHOLD {
        let handles = data
            .chunks(THRESHOLD)
            .map(|chunk| {
                let chunk = chunk.to_vec();
                tokio::spawn(
                    async move { chunk.into_iter().map(|item| f(item)).collect::<Vec<R>>() },
                )
            })
            .collect::<Vec<JoinHandle<Vec<R>>>>();

        let mut results = vec![];
        for handle in handles {
            let result = handle.await.unwrap();
            results.extend(result);
        }
        results
    } else {
        data.into_iter().map(|item| f(item)).collect()
    }
}

pub fn multi_thread_rayon<F, R, T>(data: Vec<T>, f: F) -> Vec<R>
where
    F: Fn(T) -> R + Send + Sync + Copy + 'static,
    T: Sync + Send + 'static + Clone,
    R: Send + 'static,
{
    use rayon::prelude::*;

    if data.len() > THRESHOLD {
        data.par_chunks(THRESHOLD)
            .flat_map(|chunk| {
                let chunk = chunk.to_vec();
                chunk.into_iter().map(|item| f(item)).collect::<Vec<R>>()
            })
            .collect::<Vec<R>>()
    } else {
        data.into_iter().map(|item| f(item)).collect()
    }
}
