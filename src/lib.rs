#![feature(test)]
extern crate test;

#[cfg(test)]
mod tests {
    use super::*;
    use test::Bencher;
    use std::thread;

    fn create_thread(num: usize) {
        for i in 0..num {
            thread::spawn(move || {});
        }
    }

    fn clone_channel(num: usize) {
        let (tx, rx) = std::sync::mpsc::channel::<usize>();

        for i in 0..num {
            let _ = tx.clone();
        }
    }

    fn thread_channel_comm(num: usize) {
        let (tx, rx) = std::sync::mpsc::channel::<usize>();

        for i in 0..num {
            let tx = tx.clone();
            thread::spawn(move || {
                tx.send(i).unwrap();
            });
        }

        for _ in 0..num {
            let _ = rx.recv().unwrap();
        }
    }

    #[bench]
    fn bench_thread_creation_10000(b: &mut Bencher) {
        b.iter(|| create_thread(10000));
    }

    #[bench]
    fn bench_channel_clone_10000(b: &mut Bencher) {
        b.iter(|| clone_channel(10000));
    }

    #[bench]
    fn bench_thread_channel_comm_10000(b: &mut Bencher) {
        b.iter(|| thread_channel_comm(10000));
    }
}
