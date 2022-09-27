use crate::Scene;

pub trait System {
    fn run(&self, scene: &Scene);
}

struct SystemFunction<F, Args> {
    function: F,
    args: std::marker::PhantomData<Args>,
}

impl<F, Args> SystemFunction<F, Args> {
    fn new(function: F) -> Self {
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

pub trait SystemArg: 'static {
    fn from_scene(scene: &Scene) -> Self;
}

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
