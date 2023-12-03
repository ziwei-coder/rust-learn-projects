pub fn stats(silent: bool, num_read: usize, total_bytes: &mut usize) {
    *total_bytes += num_read;

    if !silent {
        println!("\r{}", total_bytes);
    }
}
