pub fn a() {
    let x = false;
    let _ = x.clone(); // warning: using `clone` on type `bool` which implements the `Copy` trait
}
