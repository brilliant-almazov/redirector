use super::*;

#[test]
fn test_generates_unique_ids() {
    let gen = create_generator(1);
    let ids: Vec<i64> = (0..100).map(|_| gen.generate()).collect();

    let mut sorted = ids.clone();
    sorted.sort();
    sorted.dedup();
    assert_eq!(sorted.len(), ids.len());
}

#[test]
fn test_ids_are_positive() {
    let gen = create_generator(0);
    for _ in 0..100 {
        let id: i64 = gen.generate();
        assert!(id > 0);
    }
}

#[test]
fn test_ids_are_monotonically_increasing() {
    let gen = create_generator(42);
    let mut prev: i64 = gen.generate();
    for _ in 0..100 {
        let next: i64 = gen.generate();
        assert!(next > prev, "{next} should be > {prev}");
        prev = next;
    }
}

#[test]
fn test_different_machine_ids_produce_different_ids() {
    let gen1 = create_generator(1);
    let gen2 = create_generator(2);
    let id1: i64 = gen1.generate();
    let id2: i64 = gen2.generate();
    assert_ne!(id1, id2);
}
