use std::vec::Vec;

static mut FOO: Vec<i32> = Vec::new();

macro_rules! safe {
    ($x:expr) => {
        unsafe { $x }
    };
}

macro_rules! uprintln {
    ($($x:expr),*) => {
        unsafe { println!($($x),*) }
    };
}


fn get()->Vec<i32>{
    return unsafe{FOO.clone()};
}

fn foo_push(data:i32) {
    return unsafe { FOO.push(data); };
}


#[you_can::turn_off_the_borrow_checker]
fn unchecked<'a, T>(mut x: T) -> &'a T {
    return &mut x;
}



fn unref<T:Copy>(x: *const T) -> T{
    return unsafe{*x};
}

macro_rules! unref {
    ($x:expr) => {
        unsafe { *$x }
    };
}

macro_rules! maw {
    ($x:expr) => {
        &mut $x as *mut _
    };
}

macro_rules! raw {
    ($x:expr) => {
        &$x as *const _
    };
}

    macro_rules! met {
        ($var:ident = $val:expr) => {
            let mut $var = $val;
        };
    }

    macro_rules! mef {
        ($var:ident = $val:expr) => {
            ref mut $var = $val;
        };
    }

    macro_rules! set {
        ($var:ident:$typ:ty = $val:expr) => {
            static $var:$typ = $val;
        };
    }

    macro_rules! sut {
        ($var:ident:$typ:ty = $val:expr) => {
            static mut $var:$typ = $val;
        };
    }
macro_rules! put {
    ($var:ident = $val:expr) => {
       unsafe{$var = $val};
    };
}

macro_rules! at {
    ($var:ident[$val:expr]) => {
        unsafe{$var[$val]};
    };
}

fn leak<T>(val: T) -> &'static T {
    Box::leak(Box::new(val))
}

fn stat<'a, T>(val: &'a T) -> &'static T {
    unsafe{std::mem::transmute::<&'a T, &'static T>(val)}
}

macro_rules! stat {
    ($val:expr) => {
        unsafe { std::mem::transmute::<_, &'static _>($val) }
    };
}

    fn mstat<'a, T>(val: &'a mut T) -> &'static mut T {
        unsafe { std::mem::transmute::<&'a mut T, &'static mut T>(val) }
    }

    macro_rules! mstat {
        ($val:expr) => {
            unsafe { std::mem::transmute::<_, &'static mut _>($val) }
        };
    }

struct App {
    foo: Vec<i32>
}

impl App {
    fn new() -> Self {
        Self {
            foo: Vec::from([1,2,3])
        }
    }

    fn main(&mut self) {
        self.foo.push(42);
        println!("{:?}", self.foo);
        let owned = vec![1, 32];

        let mut_1 = &unchecked(at!(owned[0]));
        let mut_2 = &unchecked(owned[1]);
        //~^ WARNING the borrow checker is suppressed for these references.

        drop(owned);
        let undefined = *mut_1 + *mut_2;
        println!("{undefined}");

        set!(test1:i32 = 1);
        sut!(test2:i32 = 1);
        put!(test2 = 3);
        met!(num = stat(&5));
        met!(r1 = maw!(num));
        let r2 = maw!(r1);
        println!("{:?}", unref(r1));
        println!("{:?}", unref!(r2));
        println!("{:?}", test1);
        println!("{:?}", safe!(test2));
        foo_push(42);
        uprintln!("{:?}", FOO);
    }
}

fn main() {
    App::new().main();
}
