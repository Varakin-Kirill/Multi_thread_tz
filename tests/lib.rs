use multithreading::{converting, multi_thread, multi_thread_rayon, multi_thread_tokio};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_converting() {
        assert_eq!(converting(1), 0);
        assert_eq!(converting(5), 5);
        assert_eq!(converting(100), 88);
    }

    #[test]
    fn small_data() {
        let nums = vec![1, 2, 3];
        let result_thread: Vec<u32> = multi_thread(nums.clone(), converting);
        assert_eq!(result_thread, vec![0, 1, 7]);

        let result_rayon: Vec<u32> = multi_thread_rayon(nums.clone(), converting);
        assert_eq!(result_rayon, vec![0, 1, 7]);

        let result_tokio = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(multi_thread_tokio(nums.clone(), converting));
        assert_eq!(result_tokio, vec![0, 1, 7]);
    }
    #[test]
    fn large_data() {
        let nums = vec![1, 2, 3, 100, 1, 2, 3, 100, 1, 2, 3, 100];
        let result_thread: Vec<u32> = multi_thread(nums.clone(), converting);
        assert_eq!(result_thread, vec![0, 1, 7, 88, 0, 1, 7, 88, 0, 1, 7, 88,]);

        let result_rayon: Vec<u32> = multi_thread_rayon(nums.clone(), converting);
        assert_eq!(result_rayon, vec![0, 1, 7, 88, 0, 1, 7, 88, 0, 1, 7, 88,]);

        let result_tokio = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(multi_thread_tokio(nums.clone(), converting));
        assert_eq!(result_tokio, vec![0, 1, 7, 88, 0, 1, 7, 88, 0, 1, 7, 88,]);
    }
    #[test]
    fn empty_data() {
        let nums: Vec<u32> = Vec::new();
        let result_thread = multi_thread(nums.clone(), converting);
        assert!(result_thread.is_empty());

        let result_rayon = multi_thread_rayon(nums.clone(), converting);
        assert!(result_rayon.is_empty());

        let result_tokio = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(multi_thread_tokio(nums.clone(), converting));
        assert!(result_tokio.is_empty());
    }

    #[test]
    fn closure_usage() {
        let nums = vec![1, 2, 3, 100];

        let result_thread = multi_thread(nums.clone(), |num| num * 2);
        assert_eq!(result_thread, vec![2, 4, 6, 200]);

        let result_rayon = multi_thread_rayon(nums.clone(), |num| num * 2);
        assert_eq!(result_rayon, vec![2, 4, 6, 200]);

        let result_tokio = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(multi_thread_tokio(nums.clone(), |num| num * 2));
        assert_eq!(result_tokio, vec![2, 4, 6, 200]);
    }

    #[test]
    fn struct_data() {
        #[derive(Clone, Debug)]
        struct Number {
            value: u32,
        }

        impl Number {
            fn new(value: u32) -> Self {
                Self { value }
            }
        }
        let nums = vec![
            Number::new(1),
            Number::new(2),
            Number::new(3),
            Number::new(100),
        ];

        let result_thread = multi_thread(nums.clone(), |num: Number| converting(num.value));
        assert_eq!(result_thread, vec![0, 1, 7, 88]);

        let result_rayon = multi_thread_rayon(nums.clone(), |num: Number| converting(num.value));
        assert_eq!(result_rayon, vec![0, 1, 7, 88]);

        let result_tokio = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(multi_thread_tokio(nums.clone(), |num: Number| {
                converting(num.value)
            }));
        assert_eq!(result_tokio, vec![0, 1, 7, 88]);
    }

    #[test]
    fn string_usage() {
        fn reverse_string(input: &str) -> String {
            input.chars().rev().collect()
        }

        let data = vec!["hello", "hello", "hello", "hello", "hello"];

        let result_thread = multi_thread(data.clone(), reverse_string);
        assert_eq!(
            result_thread,
            vec!["olleh", "olleh", "olleh", "olleh", "olleh"]
        );

        let result_rayon = multi_thread_rayon(data.clone(), reverse_string);
        assert_eq!(
            result_rayon,
            vec!["olleh", "olleh", "olleh", "olleh", "olleh"]
        );

        let result_tokio = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(multi_thread_tokio(data.clone(), reverse_string));
        assert_eq!(
            result_tokio,
            vec!["olleh", "olleh", "olleh", "olleh", "olleh"]
        );
    }
}
