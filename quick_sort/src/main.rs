fn sort<T>(collection: &mut [T], low: usize, high: usize) -> ()
where
    T: PartialOrd + Copy + std::fmt::Display + std::fmt::Debug,
{
    if high <= low {
        return;
    }
    let i = partition(collection, low, high);
    if i > 0 {
        sort(collection, low, i - 1);
    }
    sort(collection, i + 1, high);
}

fn k_th<T>(collection: &mut [T], k: usize, mut low: usize, mut high: usize) -> usize
where
    T: PartialOrd + Copy + std::fmt::Display + std::fmt::Debug,
{
    while (high > low) {
        let mut i = partition(collection, low, high);
        if i == k {
            break;
        } else if i > k {
            high = i - 1;
        } else if k > i {
            low = i + 1;
        }
    }
    k
}

fn partition<T>(collection: &mut [T], low: usize, high: usize) -> usize
where
    T: PartialOrd + Copy + std::fmt::Display + std::fmt::Debug,
{
    let mut i = low + 1;
    let mut j = high;

    let pivot = collection[low];

    println!("low: {}, hi: {}", low, high);
    let mut x: T;
    loop {
        while collection[i] <= pivot {
            if i == high {
                break;
            }
            i += 1;
        }

        while pivot <= collection[j] {
            if j == low {
                break;
            }
            j -= 1;
        }

        if i >= j {
            break;
        }
        x = collection[i];
        collection[i] = collection[j];
        collection[j] = x;
    }

    x = collection[low];
    collection[low] = collection[j];
    collection[j] = x;

    j
}

fn main() {
    println!("Hello, world!");

    let mut array = vec![2, 4, 1, 3, 7, -1, 10, 11, 23, 22, -9];
    //let mut array = [1; 100000];
    let high = array.len() - 1;

    println!("{:#?}", array);
    let x = k_th(&mut array, 5, 0, high);
    println!("{:#?}", array);

    println!("{:#?}", array[x]);
}
