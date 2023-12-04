// use eyre::Result;
use futures::channel::oneshot::{channel, Sender};
use futures::future::join_all;
use std::fmt::Error;
use std::time::Duration;
use tokio::time::sleep;

type Param = (Duration, u64);
type CompletionHandler = Sender<Result<u64, Error>>;

trait MultiCallable {
    fn call<T: IntoIterator<Item = (Duration, u64)>>(&self, param: T) -> Vec<Result<u64, Error>>;
}

#[allow(dead_code)]
struct MultiCall;

#[allow(dead_code)]
async fn sleep_duration(duration: Duration, task_id: u64) -> Result<u64, Error> {
    sleep(duration).await;

    Ok(task_id)
}

#[allow(dead_code)]
impl MultiCall {
    async fn call<T>(&self, params: T) -> Vec<Result<u64, Error>>
    where
        T: IntoIterator<Item = (Duration, u64)>,
    {
        let mut futures = Vec::new();
        for (execution_cost, task_id) in params {
            // sleep(execution_cost);
            futures.push(sleep_duration(execution_cost, task_id));
            println!("executing task {}", execution_cost.as_secs());
        }
        // unpack the futures
        let results = join_all(futures).await;

        results
    }
}

#[allow(dead_code)]
struct CallBatch {
    requests: Vec<(Param, CompletionHandler)>,
}

#[allow(dead_code)]
impl CallBatch {
    pub fn new() -> Self {
        Self {
            requests: Vec::new(),
        }
    }

    pub fn push(
        &mut self,
        execution_cost: Duration,
        task_id: u64,
    ) -> impl std::future::Future<Output = Result<u64, Error>> {
        let (tx, rx) = channel();
        self.requests.push(((execution_cost, task_id), tx));
        async move { rx.await.unwrap() }
    }

    pub async fn execute_all(self, chunk_size: usize) {
        let mut iterator = self.requests.into_iter().peekable();

        while iterator.peek().is_some() {
            let (requests, senders): (Vec<_>, Vec<_>) = iterator.by_ref().take(chunk_size).unzip();

            let batch_result = MultiCall.call(requests).await;

            println!("batch_result: {:?}", batch_result);

            for (i, sender) in senders.into_iter().enumerate() {
                let result = batch_result[i].clone();
                sender.send(result).unwrap();
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_multi_call() {
        let params = vec![
            (Duration::from_secs(1), 1),
            (Duration::from_secs(2), 2),
            (Duration::from_secs(3), 6),
        ];
        // time now
        let start = std::time::Instant::now();
        let results = MultiCall.call(params).await;
        // duration
        let duration = start.elapsed();

        println!("Time elapsed in expensive_function() is: {:?}", duration);

        assert_eq!(results.len(), 3);
        assert_eq!(results[0], Ok(1));
        assert_eq!(results[1].unwrap(), 2);
        assert_eq!(results[2].unwrap(), 6);
    }

    #[tokio::test]
    async fn test_sleep_duration() {
        let duration = Duration::from_secs(2);
        let task_id = 1;
        let result = sleep_duration(duration, task_id).await;
        assert_eq!(result, Ok(task_id));
    }

    #[tokio::test]
    async fn test_call_batch() {
        let mut call_batch = CallBatch::new();
        let results = vec![
            call_batch.push(Duration::from_secs(1), 1),
            call_batch.push(Duration::from_secs(2), 2),
            call_batch.push(Duration::from_secs(3), 3),
        ];
        let start = std::time::Instant::now();
        let _ = call_batch.execute_all(2).await;

        let results = join_all(results).await;
        let duration = start.elapsed();
        println!("Time elapsed in expensive_function() is: {:?}", duration);
        println!("results: {:?}", results);
        assert_eq!(results.len(), 3);
        // Add assertions here to verify the results of the batch execution
    }
}
