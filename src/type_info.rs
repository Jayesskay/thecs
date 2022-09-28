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
            std::ptr::read::<T>(bytes.cast());
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
