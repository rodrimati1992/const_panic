#[derive(Copy, Clone)]
#[repr(transparent)]
pub struct Wrapper<T>(pub T);
