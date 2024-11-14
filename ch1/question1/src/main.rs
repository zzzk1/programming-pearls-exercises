// In Question 1, we have unlimited memory available for use, so there is no need to save space.
pub fn main() {
    // This data is read from a file that we want to sort.
    let unsorted_data: Vec<usize> = vec![10, 1, 6, 3, 7, 5, 11, 32, 2];
    let acctual = soft_by_external_array(&unsorted_data);
    let except = vec![1, 2, 3, 5, 6, 7, 10, 11, 32];

    assert_eq!(acctual, except);
}

fn soft_by_external_array(unsoft_data: &Vec<usize>) -> Vec<usize> {
    let mut temp = [0; 100];

    for num in unsoft_data {
        temp[*num] = 1;
    }

    let mut result: Vec<usize> = Vec::new();
    for i in 0..temp.len() {
        if temp[i] != 0 {
            result.push(i);
        }
    }

    result
}
