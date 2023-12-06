use std::fs;

pub fn read_file(path_to_file: &'static str) -> String {
    fs::read_to_string(path_to_file)
        .expect("Should have been able to read the file. Try running the code in the root directory of the development project.")
}

pub fn create_two_dimensional_array<T>(
    x: usize,
    y: usize,
    default_value: T,
) -> Vec<Vec<T>>
where
    T: Copy,
{
    let mut two_dimensional_array: Vec<Vec<T>> = Vec::with_capacity(x);

    for _ in 0..x {
        let mut tmp_array: Vec<T> = Vec::new();

        for _ in 0..y {
            tmp_array.push(default_value);
        }

        two_dimensional_array.push(tmp_array);
    }

    two_dimensional_array
}

pub fn reset_two_dimensional_array<T>(
    two_dimensional_array: &mut [Vec<T>],
    default_value: T,
) where
    T: Copy,
{
    // Wrestled with the borrow checker. This is bad since we create a new Vec by cloning. Should
    // be referencing the original array when we loop.
    for (x, _) in two_dimensional_array.to_owned().iter().enumerate() {
        for (y, _) in two_dimensional_array[x].to_owned().iter().enumerate() {
            two_dimensional_array[x][y] = default_value;
        }
    }
}
