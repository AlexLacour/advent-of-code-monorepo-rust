// Generic CS Algorithms

pub fn floyd_cycle_finding<T: std::cmp::PartialEq + Clone>(
    root: &T,
    next_fn: fn(&T) -> T,
) -> (i32, i32) {
    // Floyd cycle detection algorithm
    // see https://en.wikipedia.org/wiki/Cycle_detection#Floyd.27s_Tortoise_and_Hare
    let mut tortoise = next_fn(root);
    let mut hare = next_fn(&next_fn(root));

    while tortoise != hare {
        tortoise = next_fn(&tortoise);
        hare = next_fn(&next_fn(&hare));
    }

    // mu : the position of the first repetition
    let mut mu: i32 = 0;
    tortoise = root.clone();

    while tortoise != hare {
        tortoise = next_fn(&tortoise);
        hare = next_fn(&hare);
        mu += 1;
    }

    // lambda : the period
    let mut lambda = 1;
    hare = next_fn(&tortoise);

    while tortoise != hare {
        hare = next_fn(&hare);
        lambda += 1;
    }

    // return
    (lambda, mu)
}
