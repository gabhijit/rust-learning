static mut COMPARISONS: u32 = 0;

static mut EXCHANGES: u32 = 0;

pub fn sort<T>(collection: &mut [T], low: usize, high: usize) -> ()
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

pub fn k_th<T>(collection: &mut [T], k: usize, mut low: usize, mut high: usize) -> usize
where
    T: PartialOrd + Copy + std::fmt::Display + std::fmt::Debug,
{
    while high > low {
        let i = partition(collection, low, high);
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
    loop {
        while collection[i] <= pivot {
            unsafe {
                COMPARISONS += 1;
            }
            if i == high {
                break;
            }
            i += 1;
        }

        while pivot <= collection[j] {
            unsafe {
                COMPARISONS += 1;
            }
            if j == low {
                break;
            }
            j -= 1;
        }

        if i >= j {
            break;
        }
        collection.swap(i, j);
        unsafe {
            EXCHANGES += 1;
        }
    }

    collection.swap(low, j);
    unsafe {
        EXCHANGES += 1;
    }

    j
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn small_random_array() {
        let mut array = vec![100, 23, 1, 33, 7, -1, 10, 11, 1, 4, 102, 87, 91, 2, 22, -9];
        let high = array.len() - 1;

        sort(&mut array, 0, high);
        assert_eq!(
            &array,
            &vec![-9, -1, 1, 1, 2, 4, 7, 10, 11, 22, 23, 33, 87, 91, 100, 102]
        );
    }

    #[test]
    fn small_array_same_elements() {
        let mut array = [1; 10];
        let high = array.len() - 1;

        sort(&mut array, 0, high);

        assert_eq!(&array, &[1; 10]);
    }

    #[test]
    fn small_array_repeating_few_elems() {
        let mut array = [2, 1, 1, 2, 1, 1, 2, 4, 4, 5, 5, 5];

        let high = array.len() - 1;

        sort(&mut array, 0, high);

        assert_eq!(&array, &[1, 1, 1, 1, 2, 2, 2, 4, 4, 5, 5, 5]);
    }
}
