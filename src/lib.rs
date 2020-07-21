#![allow(unused_mut, unused_parens)]

#[macro_export]
macro_rules! system {
    ($name:ident, |$( $v:ident : $t:ty ),*| $r:block) => {
        #[derive(Default, Clone, Copy)]
        pub struct $name;

        impl<'a> System<'a> for $name {
            type SystemData = ($(
                $t,
            )*);
            fn run(&mut self, ($(mut $v,)*): Self::SystemData) {
                $r
            }
        }
    };
    ($name:ident <
                   $( $generic:tt $( : $clt:tt $(+ $dlt:tt )* )? ),+
                   >, |$( $v:ident : $t:ty ),*| $r:block) => {
        #[derive(Clone, Copy)]
        pub struct $name < $( $generic ),+ > {
            _phantom: std::marker::PhantomData<($( $generic ),+)>,
        }

        impl<'a, $( $generic $( : $clt $(+ $dlt )* )? ),+ > System<'a> for $name < $( $generic ),+ >{
            type SystemData = ($(
                $t,
            )*);
            fn run(&mut self, ($(mut $v,)*): Self::SystemData) {
                $r
            }
        }

        impl<$( $generic ),+> Default for $name < $( $generic ),+ >{
            fn default() -> Self {
                Self {
                    _phantom: std::marker::PhantomData,
                }
            }
        }
    };
}

#[cfg(test)]
mod tests {
    use specs::*;

    #[derive(Debug, Default)]
    pub struct Test {
        pub a: i32,
    }

    #[test]
    fn it_works() {
        system!(
            SystemName,
            |test: Read<'a, Test>, _test2: Write<'a, Test>| {
                println!("{}", test.a);
            }
        );
    }

    #[test]
    fn it_works2() {
        system!(
            SystemName<T: Send + Sync + 'static + Default, A>,
            |_test: Read<'a, T>| {}
        );
    }

    #[test]
    fn default_test() {
        enum E {}
        system!(SystemName<T: Send + Sync + 'static>, |_test: ReadExpect<
            'a,
            T,
        >| {});
        let _sys = SystemName::<E>::default();
    }
}
