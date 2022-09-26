use crate::{type_info::TypeInfo, Component};
use std::any::TypeId;

pub trait Batch {
    fn data(&self, type_id: TypeId) -> *const u8;
    fn types(&self) -> Vec<TypeInfo>;
}

impl<T: Component> Batch for T {
    fn data(&self, type_id: TypeId) -> *const u8 {
        if type_id == TypeId::of::<T>() {
            (self as *const T).cast::<u8>()
        } else {
            panic!()
        }
    }

    fn types(&self) -> Vec<TypeInfo> {
        vec![TypeInfo::new::<T>()]
    }
}

macro_rules! impl_batch_for_tuple {
    ({ $($type:ident),* }, { $($tuple_index:tt),* }) => {
        impl<$($type),*> Batch for ($($type),*)
        where
            $($type: Component),*
        {
            fn data(&self, type_id: TypeId) -> *const u8 {
                $(
                    if type_id == TypeId::of::<$type>() {
                        return (&self.$tuple_index as *const $type).cast::<u8>();
                    }
                )*

                panic!();
            }

            fn types(&self) -> Vec<TypeInfo> {
                let mut types: Vec<TypeInfo> = Vec::new();
                $(
                    let type_info = TypeInfo::new::<$type>();
                    if types.contains(&type_info) {
                        panic!("Actors can't contain multiple copies of same component!");
                    }

                    types.push(type_info);
                )*

                types
            }
        }
    };
}

impl_batch_for_tuple!({ T0, T1 }, { 0, 1 });
impl_batch_for_tuple!({ T0, T1, T2 }, { 0, 1, 2 });
impl_batch_for_tuple!({ T0, T1, T2, T3 }, { 0, 1, 2, 3 });
impl_batch_for_tuple!({ T0, T1, T2, T3, T4 }, { 0, 1, 2, 3, 4 });
impl_batch_for_tuple!({ T0, T1, T2, T3, T4, T5 }, { 0, 1, 2, 3, 4, 5 });
impl_batch_for_tuple!({ T0, T1, T2, T3, T4, T5, T6 }, { 0, 1, 2, 3, 4, 5, 6 });
impl_batch_for_tuple!({ T0, T1, T2, T3, T4, T5, T6, T7 }, { 0, 1, 2, 3, 4, 5, 6, 7 });
impl_batch_for_tuple!({ T0, T1, T2, T3, T4, T5, T6, T7, T8 }, { 0, 1, 2, 3, 4, 5, 6, 7, 8 });
impl_batch_for_tuple!({ T0, T1, T2, T3, T4, T5, T6, T7, T8, T9 }, { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 });
