use macro_vis::macro_vis;

#[macro_vis(pub(crate))]
macro_rules! choose {
    (@method $visibility:vis fn $name:ident(&mut self) from $from:path;) => {

        $visibility fn $name(&mut self) -> &'static str {
            use rand::seq::SliceRandom;
            $from.choose(self).expect(concat!(stringify!($from), " should not be empty"))
        }
    };
    (@method $visibility:vis fn $name:ident(&mut self) -> $type:ty; from $from:path;) => {

        $visibility fn $name(&mut self) -> $type {
            use rand::seq::SliceRandom;
            $from.choose(self).expect(concat!(stringify!($from), " should not be empty"))
        }
    };
    {
        $(
            $visibility:vis fn $name:ident(&mut self) $(-> $type:ty;)? from $from:path;
        )+
    } => {
        $(
            $crate::choose!(@method $visibility fn $name(&mut self) $(-> $type;)? from $from;);
        )+
    };
}

#[macro_vis(pub(crate))]
macro_rules! array_consts {
    [
        $(
            pub const $name:ident: [$type:ty; _] = $array:expr;
        )+
    ] => {
        $(
            pub const $name: [$type; $array.len()] = $array;
        )+
    };
}
