

#[macro_export]
macro_rules! cenum {
    (
        $(#[$struct_attr:meta])*
        $vis:vis enum $struct_name:ident {
            $(
                #[$doc_meta:meta]
                $const_name:ident $func_name:ident = $value:expr
            ),*
            $(,)?
        }
    ) => {

        // ***************************************
        
        #[repr(transparent)]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
        $vis struct $struct_name(core::ffi::c_int);

        impl $struct_name {
            pub const NONE: Self = Self(0);

            #[must_use]
            #[inline(always)]
            pub const fn has_flags(self, flags: Self) -> bool {
                self.0 & flags.0 == flags.0
            }

            #[inline(always)]
            pub const fn add_flags(&mut self, flags: Self) {
                self.0 |= flags.0;
            }

            #[inline(always)]
            pub const fn remove_flags(&mut self, flags: Self) {
                self.0 &= !flags.0;
            }

            #[inline(always)]
            pub const fn toggle_flags(&mut self, flags: Self) {
                self.0 ^= flags.0;
            }

            #[must_use]
            #[inline(always)]
            pub const fn with_flags(mut self, flags: Self) -> Self {
                self.add_flags(flags);
                self
            }

            #[must_use]
            #[inline(always)]
            pub const fn without_flags(mut self, flags: Self) -> Self {
                self.remove_flags(flags);
                self
            }

            #[must_use]
            #[inline]
            pub const fn count_ones(self) -> u32 {
                self.0.count_ones()
            }

            #[must_use]
            #[inline]
            pub const fn pop_bottom_index(&mut self) -> Option<u32> {
                if self.0 == 0 {
                    return None;
                }
                let mut mask = self.0.cast_unsigned();
                let next_bit = mask.trailing_zeros();
                mask ^= 1 << next_bit;
                self.0 = mask.cast_signed();
                Some(next_bit)
            }

            #[must_use]
            #[inline]
            pub fn get_flags(self) -> Box<[(Self, &'static str)]> {
                self.collect()
            }
        }

        impl Iterator for $struct_name {
            type Item = ($struct_name, &'static str);
            fn size_hint(&self) -> (usize, Option<usize>) {
                (self.count_ones() as usize, Some(self.count_ones() as usize))
            }

            fn next(&mut self) -> Option<Self::Item> {
                let index = self.pop_bottom_index()?;
                let index = index as usize;
                Some((Self::ALL_FLAGS[index], Self::FLAG_NAMES[index]))
            }
        }

        // ***************************************

        paste::paste!{
            impl $struct_name {
                pub const ALL: Self = {
                    let mut builder = Self::NONE;
                    $(
                        builder.add_flags(Self::$const_name);
                    )*
                    builder
                };
                pub const ALL_FLAGS: &'static [Self] = &[
                    $(
                        Self::$const_name,
                    )*
                ];
                pub const FLAG_NAMES: &'static [&'static str] = &[
                    $(
                        stringify!($func_name),
                    )*
                ];
                $(
                    #[$doc_meta]
                    pub const $const_name: Self = Self($value);
                    
                    #[must_use]
                    #[inline(always)]
                    pub const fn [<get_ $func_name>](self) -> bool {
                        self.has_flags(Self::$const_name)
                    }

                    #[inline(always)]
                    pub const fn [<add_ $func_name>](&mut self) {
                        self.add_flags(Self::$const_name);
                    }

                    #[inline(always)]
                    pub const fn [<remove_ $func_name>](&mut self) {
                        self.remove_flags(Self::$const_name);
                    }

                    #[inline]
                    pub const fn [<set_ $func_name>](&mut self, on: bool) {
                        if on {
                            self.add_flags(Self::$const_name);
                        } else {
                            self.remove_flags(Self::$const_name);
                        }
                    }

                    #[inline(always)]
                    pub const fn [<toggle_ $func_name>](&mut self) {
                        self.0 ^= Self::$const_name.0;
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn [<with_ $func_name>](self) -> Self {
                        self.with_flags(Self::$const_name)
                    }

                    #[must_use]
                    #[inline(always)]
                    pub const fn [<without_ $func_name>](self) -> Self {
                        self.without_flags(Self::$const_name)
                    }
                )*
            }
        }
    };
}