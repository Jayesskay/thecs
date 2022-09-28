use super::{System, SystemArg, SystemFunction};

pub trait IntoSystem<Args> {
    fn into_system(self) -> Box<dyn System>;
}

macro_rules! into_system_for_function {
    ($($arg:ident),*) => {
        #[allow(unused_parens)]
        impl<F, $($arg),*> IntoSystem<(($($arg),*))> for F
        where
            F: Fn($($arg),*) + 'static,
            $($arg: SystemArg),*
        {
            fn into_system(self) -> Box<dyn System> {
                Box::new(SystemFunction::new(self))
            }
        }
    };
}
into_system_for_function!();
into_system_for_function!(Arg0);
into_system_for_function!(Arg0, Arg1);
into_system_for_function!(Arg0, Arg1, Arg2);
into_system_for_function!(Arg0, Arg1, Arg2, Arg3);
into_system_for_function!(Arg0, Arg1, Arg2, Arg3, Arg4);
into_system_for_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5);
into_system_for_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6);
into_system_for_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7);
into_system_for_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8);
into_system_for_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9);
