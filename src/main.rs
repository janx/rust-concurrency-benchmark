extern crate mioco;

fn main() {
    let (tx, rx) = std::sync::mpsc::channel::<usize>();

    mioco::start(move || {
        for i in 0..10 {
            let tx = tx.clone();
            mioco::spawn(move || {
                println!("{}?", i);
                tx.send(i).unwrap();
            });
        }

        mioco::spawn(move || {
            for _ in 0..10 {
                let v = rx.recv().unwrap();
                println!("{}!", v);
            }
        });

    }).unwrap();
}
