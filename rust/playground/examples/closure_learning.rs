#[derive(Clone, Debug)]
struct Thing {
}

fn do_with_owned(thing: Thing) {
    drop(thing)
}

fn use_closure(mut func: impl FnMut(Thing) -> Thing, thing: Thing) -> Thing {
    func(thing)
}

fn main() {
    let vec = vec![Thing{}, Thing{}];

    for v in vec {
        let clo = move |_| {
            v.clone()
        };
        use_closure(clo, v);
    }
    drop(vec)
}
