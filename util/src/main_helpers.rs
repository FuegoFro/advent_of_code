#[macro_export]
macro_rules! declare_and_run {
    ( $( $mod_name:ident ),* $(,)? ) => {
        $(
            mod $mod_name;
        )*
        fn main() {
            util::main_helpers::main_func(vec![
                $(
                    $mod_name::main,
                )*
            ]);
        }
    }
}

pub fn main_func(days: Vec<fn()>) {
    let day_func = match std::env::args().len() {
        0 | 1 => days.last().unwrap(),
        2 => &days[std::env::args().nth(1).unwrap().parse::<usize>().unwrap() - 1],
        _ => panic!("Incorrect number of args; expect no args or a single integer argument specifying the day"),
    };
    day_func();
}
