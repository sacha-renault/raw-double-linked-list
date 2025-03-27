use super::list_item::ItemPtr;

#[derive(Debug, Copy, Clone)]
pub enum Side {
    Left,
    Right,
}

pub fn get_ptr_starting_point(index: usize, len: usize) -> Side {
    if len / 2 >= index {
        Side::Left
    } else {
        Side::Right
    }
}

pub fn find_index_through<T>(
    mut raw_ptr: ItemPtr<T>,
    target_index: usize,
    len: usize,
    side: &Side,
) -> Option<ItemPtr<T>> {
    // Starting index
    let mut current_index = match side {
        &Side::Left => 0,
        &Side::Right => len - 1,
    };

    // Iterate until we find the correct index (or we get a null value)
    while current_index != target_index {
        match side {
            &Side::Left => {
                current_index += 1;
                raw_ptr = unsafe { (*raw_ptr).next? }
            }
            &Side::Right => {
                current_index -= 1;
                raw_ptr = unsafe { (*raw_ptr).previous? }
            }
        };
    }

    // Some value was found
    Some(raw_ptr)
}
