fn main() {
    let mut vec = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    // it should be only one &mut in every section. If not, when we try to modify a value in multi-threads. It can cause data problem like deadlock
    // So we just need to add two pair of rackets for it. Before I create and use b, a got released already.
    {
        let a = &mut vec;
        a.push(11);
    }

    {
        let b = &mut vec;
        b.push(12);
    }
}
