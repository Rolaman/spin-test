use crate::executor::smart_execute;

#[derive(Copy, Clone)]
struct ItemWithStep {
    item: u64,
    k: u64,
}

impl ItemWithStep {
    fn new(item: u64, k: u64) -> ItemWithStep {
        ItemWithStep {
            item,
            k,
        }
    }
}

pub fn process_numbers(numbers: Vec<u64>, k: u64) -> Vec<u64> {
    let items = numbers.iter()
        .map(|i| {
            ItemWithStep::new(*i, k)
        }).collect();
    smart_execute(items, result_function)
}


fn result_function(i: ItemWithStep) -> u64 {
    return step(i.item, 0, i.k);
}

fn step(input: u64, step_number: u64, k: u64) -> u64 {
    if step_number >= k {
        return input;
    }
    if input == 1 {
        return step_number;
    }
    match input.clone() % 2 {
        0 => step(input / 2, step_number + 1, k),
        1 => step(input * 3 + 1, step_number + 1, k),
        _ => panic!("impossible remainder"),
    }
}

#[cfg(test)]
mod tests {
    use crate::processor::process_numbers;

    #[test]
    fn return_correct_answers() {
        let params = vec![1, 2, 3, 100];
        let k = 8;
        let result = process_numbers(params, k);
        assert_eq!(vec![0, 1, 7, 88], result);
    }
}
