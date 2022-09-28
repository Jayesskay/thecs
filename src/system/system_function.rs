use super::{System, SystemArg};
use crate::Scene;

pub struct SystemFunction<F, Args> {
    function: F,
    args: std::marker::PhantomData<Args>,
}

impl<F, Args> SystemFunction<F, Args> {
    pub fn new(function: F) -> Self {
        Self {
            function,
            args: std::marker::PhantomData,
        }
    }
}

macro_rules! system_for_system_function {
    ($($arg:ident),*) => {
        #[allow(unused_parens)]
        impl<F, $($arg),*> System for SystemFunction<F, ($($arg),*)>
        where
            F: Fn($($arg),*),
            $($arg: SystemArg),*
        {
            fn run(&self, _scene: &Scene) {
                (self.function)($($arg::from_scene(_scene)),*);
            }
        }
    };
}

system_for_system_function!();
system_for_system_function!(Arg0);
system_for_system_function!(Arg0, Arg1);
system_for_system_function!(Arg0, Arg1, Arg2);
system_for_system_function!(Arg0, Arg1, Arg2, Arg3);
system_for_system_function!(Arg0, Arg1, Arg2, Arg3, Arg4);
system_for_system_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5);
system_for_system_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6);
system_for_system_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7);
system_for_system_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8);
system_for_system_function!(Arg0, Arg1, Arg2, Arg3, Arg4, Arg5, Arg6, Arg7, Arg8, Arg9);
