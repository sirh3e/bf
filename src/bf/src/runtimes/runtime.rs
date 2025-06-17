macro_rules! inc_val_by {
    ($memory:expr, $index:expr, $amount:expr) => {
        $memory[$index] = $memory[$index].wrapping_add($amount);
    };
}

macro_rules! dec_val_by {
    ($memory:expr, $index:expr, $amount:expr) => {
        $memory[$index] = $memory[$index].wrapping_sub($amount);
    };
}

macro_rules! mul_val_by {
    ($memory:expr, $index:expr, $offset:expr, $amount:expr) => {
        let offset = $index.checked_add_signed($offset).unwrap();

        $memory[offset] = $memory[offset].wrapping_add($memory[$index].wrapping_mul($amount));
    };
}

macro_rules! inc_ptr_by {
    ($pointer:expr, $amount:expr) => {
        $pointer += $amount
    };
}

macro_rules! dec_ptr_by {
    ($pointer:expr, $amount:expr) => {
        $pointer -= $amount
    };
}

macro_rules! r#loop {
     ($memory:expr, $index:expr, $( $expression:expr ),*) => {
        while $memory[$index] != 0 {
         $(
             $expression;
         )*
        }
     };
}

macro_rules! clear {
    ($memory:expr, $index:expr) => {
		$memory[$index] = 0;
    };
}

macro_rules! output {
    ($memory:expr, $pointer:expr) => {
        print!("{}", $memory[$pointer] as char);
    };
}

fn main() {
    let mut <POINTER> = 0 as usize;
    let mut <MEMORY> = [0 as u8; 30_000];

<CODE>
}