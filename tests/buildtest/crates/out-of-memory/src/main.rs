const MAX_ALLOCATE: usize = 1_073_741_824;

pub fn allocate() {
}

fn main() {
    let data = [0u8; 4096];
    let mut allocated = 0;

    while allocated < MAX_ALLOCATE {
        Box::leak(Box::new(data));
        allocated += data.len();
    }

    println!("Allocated {} bytes of memory!", MAX_ALLOCATE);
}
