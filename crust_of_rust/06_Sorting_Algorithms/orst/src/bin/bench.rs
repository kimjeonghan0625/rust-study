use orst::*;
use rand::prelude::*;

use std::{cell::Cell, rc::Rc};
#[derive(Clone)]
struct SortEvaluator<T> {
    t: T,
    cmps: Rc<Cell<usize>>,
}

impl<T: PartialEq> PartialEq for SortEvaluator<T> {
    fn eq(&self, other: &Self) -> bool {
        self.cmps.set(self.cmps.get() + 1);
        self.t == other.t
    }
}

impl<T: Eq> Eq for SortEvaluator<T> {}

impl<T: Ord> Ord for SortEvaluator<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.cmps.set(self.cmps.get() + 1);
        self.t.cmp(&other.t)
    }
}

impl<T: PartialOrd> PartialOrd for SortEvaluator<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.cmps.set(self.cmps.get() + 1);
        self.t.partial_cmp(&other.t)
    }
}

fn main() {
    let mut rand = rand::thread_rng();
    let counter = Rc::new(Cell::new(0));
    for n in [0, 1, 10, 100, 1000, 10000] {
        for _ in 0..10 {
            let mut values = Vec::with_capacity(n);
            for _ in 0..n {
                values.push(SortEvaluator {
                    t: rand.gen::<usize>(),
                    cmps: Rc::clone(&counter),
                });
            }

            let took = bench(Bubblesort, &values, &counter);
            println!("{} {} {} {}", "bubble", n, took.0, took.1);
            let took = bench(InsertionSort { smart: true }, &values, &counter);
            println!("{} {} {} {}", "insertion-smart", n, took.0, took.1);
            let took = bench(InsertionSort { smart: false }, &values, &counter);
            println!("{} {} {} {}", "insertion-dumb", n, took.0, took.1);
            let took = bench(SelectionSort, &values, &counter);
            println!("{} {} {} {}", "selection", n, took.0, took.1);
            let took = bench(QuickSort1, &values, &counter);
            println!("{} {} {} {}", "quick1", n, took.0, took.1);
            let took = bench(QuickSort2, &values, &counter);
            println!("{} {} {} {}", "quick2", n, took.0, took.1);
            let took = bench(StdSort, &values, &counter);
            println!("{} {} {} {}", "std", n, took.0, took.1);
            println!()
        }
    }
}

fn bench<T: Ord + Clone, S: Sorter>(
    sorter: S,
    values: &[SortEvaluator<T>],
    counter: &Cell<usize>,
) -> (usize, f64) {
    // let mut values = Vec::from(values);
    // let mut values: Vec<_> = values.into_iter().cloned().collect();
    let mut values = values.to_vec();
    counter.set(0);
    let time = std::time::Instant::now();
    sorter.sort(&mut values);
    let took = time.elapsed();
    // assert!(values.is_sorted());
    let count = counter.get();
    for i in 1..values.len() {
        assert!(values[i] >= values[i - 1])
    }
    (count, took.as_secs_f64())
}
