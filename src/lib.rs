#![feature(test)]
extern crate test;
extern crate mioco;

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

    fn create_mioco_thread(num: usize) {
        mioco::start(move || {
            for i in 0..num {
                mioco::spawn(move || {});
            }
        }).unwrap();
    }

    fn mioco_channel_comm(num: usize) {
        let (tx, rx) = std::sync::mpsc::channel::<usize>();

        mioco::start(move || {
            for i in 0..num {
                let tx = tx.clone();
                mioco::spawn(move || {
                    tx.send(i).unwrap();
                });
            }

            mioco::spawn(move || {
                for _ in 0..num {
                    let _ = rx.recv().unwrap();
                }
            });
        }).unwrap();
    }


    #[bench]
    fn bench_thread_creation_1000(b: &mut Bencher) {
        b.iter(|| create_thread(1000));
    }

    #[bench]
    fn bench_channel_clone_1000(b: &mut Bencher) {
        b.iter(|| clone_channel(1000));
    }

    #[bench]
    fn bench_thread_channel_comm_1000(b: &mut Bencher) {
        b.iter(|| thread_channel_comm(1000));
    }

    #[bench]
    fn bench_mioco_creation_1000(b: &mut Bencher) {
        b.iter(|| create_mioco_thread(1000));
    }

    #[bench]
    fn bench_mioco_channel_comm_1000(b: &mut Bencher) {
        b.iter(|| mioco_channel_comm(1000));
    }
}
