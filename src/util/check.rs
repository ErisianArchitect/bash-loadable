

#[track_caller]
pub const fn assert_same_size_align<L, R>() {
    let size_mismatch = size_of::<L>() != size_of::<R>();
    let align_mismatch = align_of::<L>() != align_of::<R>();
    match (size_mismatch, align_mismatch) {
        (true, true) => panic!("Size and alignment mismatch."),
        (true, false) => panic!("Size mismatch."),
        (false, true) => panic!("Align mismatch."),
        _ => (),
    }
}

#[track_caller]
pub const fn assert_pointer_size_align<T>() {
    assert_same_size_align::<usize, T>();
}

#[track_caller]
pub const fn assert_niche<Niched, Target>() {
    assert_same_size_align::<Option<Niched>, Target>();
    assert_same_size_align::<Result<Niched, ()>, Target>();
    assert_same_size_align::<Result<(), Niched>, Target>();
}

#[track_caller]
pub const fn assert_pointer_niche<Niched>() {
    assert_niche::<Niched, usize>();
}

const _: () = {
    assert_pointer_niche::<core::ptr::NonNull<()>>();
};