#[macro_export]
macro_rules! impl_state_check {
    ($enum_name:ident, $variant:ident, $fn_name:ident) => {
        impl $enum_name {
            #[allow(dead_code)]
            pub fn $fn_name(&self) -> bool {
                matches!(self, $enum_name::$variant)
            }
        }
    };
}
