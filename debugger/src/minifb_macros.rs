macro_rules! handle_keys_down {
    ( $window:expr, $( $($key:expr ),* => $f:expr ,)* ) => {
        $($(if $window.is_key_down($key) {
            $f;
        })*)*
    };
}

macro_rules! handle_keys_pressed {
    ( $window:expr, $( $($key:expr ),* => $f:expr ,)* ) => {
        $($(if $window.is_key_pressed($key, KeyRepeat::No) {
            $f;
        })*)*
    };
}

macro_rules! handle_keys_pressed_repeat {
    ( $window:expr, $( $($key:expr ),* => $f:expr ,)* ) => {
        $($(if $window.is_key_pressed($key, KeyRepeat::Yes) {
            $f;
        })*)*
    };
}