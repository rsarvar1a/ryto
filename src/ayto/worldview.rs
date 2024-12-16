/// Generates the kth permutation of the numbers 0..n.
pub fn generate(n: usize, k: usize) -> Vec<usize> {
    let mut atoms: Vec<usize> = (0..n).collect();
    let mut v = vec![0; n];
    (0..n).fold((&mut v, k), |(res, k), i| {
        let item = k % (n - i);
        let k = k / (n - i);
        res[i] = atoms[item];
        atoms.remove(item);
        (res, k)
    });
    v
}
