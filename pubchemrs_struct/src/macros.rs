/// Generates Display, FromStr, and AsRef<str> for simple enums.
macro_rules! impl_enum_str {
    (
        $enum_name:ident {
            $( $variant:ident => $str:literal ),+ $(,)?
        }
    ) => {
        impl ::std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                f.write_str(::std::convert::AsRef::<str>::as_ref(self))
            }
        }

        impl ::std::convert::AsRef<str> for $enum_name {
            fn as_ref(&self) -> &str {
                match self {
                    $( $enum_name::$variant => $str, )+
                }
            }
        }

        impl ::std::str::FromStr for $enum_name {
            type Err = $crate::error::ParseEnumError;

            fn from_str(s: &str) -> ::std::result::Result<Self, Self::Err> {
                match s {
                    $( $str => Ok($enum_name::$variant), )+
                    _ => Err($crate::error::ParseEnumError::VariantNotFound),
                }
            }
        }
    };
}

/// Generates `from_repr(repr_type) -> Option<Self>` for repr enums.
macro_rules! impl_from_repr {
    (
        $enum_name:ident : $repr_type:ty {
            $( $variant:ident = $value:expr ),+ $(,)?
        }
    ) => {
        impl $enum_name {
            pub fn from_repr(value: $repr_type) -> Option<Self> {
                match value {
                    $( $value => Some($enum_name::$variant), )+
                    _ => None,
                }
            }
        }
    };
}

/// Generates `VARIANTS` constant for enums.
macro_rules! impl_variant_array {
    (
        $enum_name:ident { $( $variant:ident ),+ $(,)? }
    ) => {
        impl $enum_name {
            pub const VARIANTS: &[Self] = &[ $( $enum_name::$variant, )+ ];
        }
    };
}
