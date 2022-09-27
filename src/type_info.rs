use std::{alloc::Layout, any::TypeId};

#[derive(Clone, Copy, Eq, PartialEq)]
pub struct TypeInfo {
    pub(crate) id: TypeId,
    pub(crate) layout: Layout,
    pub(crate) drop: Option<unsafe fn(*mut u8)>,
}

impl TypeInfo {
    #[must_use]
    pub(crate) fn new<T: 'static>() -> Self {
        unsafe fn drop_as<T>(bytes: *mut u8) {
            let mut x = std::mem::MaybeUninit::uninit();
            std::ptr::copy_nonoverlapping(bytes.cast::<T>(), x.as_mut_ptr(), 1);
            x.assume_init();
        }

        Self {
            id: TypeId::of::<T>(),
            layout: Layout::new::<T>(),
            drop: if std::mem::needs_drop::<T>() {
                Some(drop_as::<T>)
            } else {
                None
            },
        }
    }
}
