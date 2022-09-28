use crate::TypeInfo;

pub trait Component: Send + Sync + 'static {}

pub trait ComponentSource {
    // TODO: Probably get rid of this allocation somehow
    fn types(&self) -> Vec<(TypeInfo, *const u8)>;
}

impl<T> ComponentSource for T
where
    T: Component,
{
    fn types(&self) -> Vec<(TypeInfo, *const u8)> {
        vec![(TypeInfo::new::<T>(), (self as *const T).cast::<u8>())]
    }
}

macro_rules! component_source_for_tuple {
    ({ $($type:ident),* }, { $($tuple_index:tt),* }) => {
        impl<$($type),*> ComponentSource for ($($type),*)
        where
            $($type:Component),*
        {
            fn types(&self) -> Vec<(TypeInfo, *const u8)> {
                vec![$({
                    let type_info = TypeInfo::new::<$type>();
                    let data = (&self.$tuple_index as *const $type).cast::<u8>();
                    (type_info, data)
                }),*]
            }
        }
    };
}

component_source_for_tuple!({ T0, T1 }, { 0, 1 });
component_source_for_tuple!({ T0, T1, T2 }, { 0, 1, 2 });
component_source_for_tuple!({ T0, T1, T2, T3 }, { 0, 1, 2, 3 });
component_source_for_tuple!({ T0, T1, T2, T3, T4 }, { 0, 1, 2, 3, 4 });
component_source_for_tuple!({ T0, T1, T2, T3, T4, T5 }, { 0, 1, 2, 3, 4, 5 });
component_source_for_tuple!({ T0, T1, T2, T3, T4, T5, T6 }, { 0, 1, 2, 3, 4, 5, 6 });
component_source_for_tuple!({ T0, T1, T2, T3, T4, T5, T6, T7 }, { 0, 1, 2, 3, 4, 5, 6, 7 });
component_source_for_tuple!({ T0, T1, T2, T3, T4, T5, T6, T7, T8 }, { 0, 1, 2, 3, 4, 5, 6, 7, 8 });
component_source_for_tuple!({ T0, T1, T2, T3, T4, T5, T6, T7, T8, T9 }, { 0, 1, 2, 3, 4, 5, 6, 7, 8, 9 });
