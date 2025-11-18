
            /// Get the kernel selected by a Cargo feature.
            ///
            /// # Example
            ///
            /// ```
            /// let kernel = tt_call::tt_call! { macro = [{ itron::macros::tt_kernel }] };
            ///
            /// println!("We are running on {}", kernel);
            /// ```
            ///
            #[cfg(feature = "asp3")]
            pub macro tt_kernel($caller:tt) {
                tt_call::tt_return! {
                    $caller
                    output = [{ "asp3" }]
                }
            }
            
            /// Determine if this crate was compiled for the specified kernel.
            ///
            /// # Example
            ///
            /// ```
            /// tt_call::tt_if! {
            ///     condition = [{ itron::macros::tt_is_kernel }]
            ///     input = [{ "asp3" }]
            ///     true = [{ println!("We are on TOPPERS/ASP3, yay!"); }]
            ///     false = [{}]
            /// }
            /// ```
            ///
            #[cfg(feature = "asp3")]
            pub macro tt_is_kernel {
                (
                    $caller:tt
                    input = [{ "asp3" $(| $($rest:tt)* )? }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ true }]
                    }
                },
                (
                    $caller:tt
                    input = [{ $other_kernel:literal $(| $($rest:tt)* )? }]
                ) => {
                    tt_is_kernel! {
                        $caller
                        input = [{ $( $($rest)* )? }]
                    }
                },
                (
                    $caller:tt
                    input = [{ }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ false }]
                    }
                },
            }
            
            /// Get the kernel selected by a Cargo feature.
            ///
            /// # Example
            ///
            /// ```
            /// let kernel = tt_call::tt_call! { macro = [{ itron::macros::tt_kernel }] };
            ///
            /// println!("We are running on {}", kernel);
            /// ```
            ///
            #[cfg(feature = "fmp3")]
            pub macro tt_kernel($caller:tt) {
                tt_call::tt_return! {
                    $caller
                    output = [{ "fmp3" }]
                }
            }
            
            /// Determine if this crate was compiled for the specified kernel.
            ///
            /// # Example
            ///
            /// ```
            /// tt_call::tt_if! {
            ///     condition = [{ itron::macros::tt_is_kernel }]
            ///     input = [{ "asp3" }]
            ///     true = [{ println!("We are on TOPPERS/ASP3, yay!"); }]
            ///     false = [{}]
            /// }
            /// ```
            ///
            #[cfg(feature = "fmp3")]
            pub macro tt_is_kernel {
                (
                    $caller:tt
                    input = [{ "fmp3" $(| $($rest:tt)* )? }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ true }]
                    }
                },
                (
                    $caller:tt
                    input = [{ $other_kernel:literal $(| $($rest:tt)* )? }]
                ) => {
                    tt_is_kernel! {
                        $caller
                        input = [{ $( $($rest)* )? }]
                    }
                },
                (
                    $caller:tt
                    input = [{ }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ false }]
                    }
                },
            }
            
            /// Get the kernel selected by a Cargo feature.
            ///
            /// # Example
            ///
            /// ```
            /// let kernel = tt_call::tt_call! { macro = [{ itron::macros::tt_kernel }] };
            ///
            /// println!("We are running on {}", kernel);
            /// ```
            ///
            #[cfg(feature = "solid_asp3")]
            pub macro tt_kernel($caller:tt) {
                tt_call::tt_return! {
                    $caller
                    output = [{ "solid_asp3" }]
                }
            }
            
            /// Determine if this crate was compiled for the specified kernel.
            ///
            /// # Example
            ///
            /// ```
            /// tt_call::tt_if! {
            ///     condition = [{ itron::macros::tt_is_kernel }]
            ///     input = [{ "asp3" }]
            ///     true = [{ println!("We are on TOPPERS/ASP3, yay!"); }]
            ///     false = [{}]
            /// }
            /// ```
            ///
            #[cfg(feature = "solid_asp3")]
            pub macro tt_is_kernel {
                (
                    $caller:tt
                    input = [{ "solid_asp3" $(| $($rest:tt)* )? }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ true }]
                    }
                },
                (
                    $caller:tt
                    input = [{ $other_kernel:literal $(| $($rest:tt)* )? }]
                ) => {
                    tt_is_kernel! {
                        $caller
                        input = [{ $( $($rest)* )? }]
                    }
                },
                (
                    $caller:tt
                    input = [{ }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ false }]
                    }
                },
            }
            
            /// Get the kernel selected by a Cargo feature.
            ///
            /// # Example
            ///
            /// ```
            /// let kernel = tt_call::tt_call! { macro = [{ itron::macros::tt_kernel }] };
            ///
            /// println!("We are running on {}", kernel);
            /// ```
            ///
            #[cfg(feature = "solid_fmp3")]
            pub macro tt_kernel($caller:tt) {
                tt_call::tt_return! {
                    $caller
                    output = [{ "solid_fmp3" }]
                }
            }
            
            /// Determine if this crate was compiled for the specified kernel.
            ///
            /// # Example
            ///
            /// ```
            /// tt_call::tt_if! {
            ///     condition = [{ itron::macros::tt_is_kernel }]
            ///     input = [{ "asp3" }]
            ///     true = [{ println!("We are on TOPPERS/ASP3, yay!"); }]
            ///     false = [{}]
            /// }
            /// ```
            ///
            #[cfg(feature = "solid_fmp3")]
            pub macro tt_is_kernel {
                (
                    $caller:tt
                    input = [{ "solid_fmp3" $(| $($rest:tt)* )? }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ true }]
                    }
                },
                (
                    $caller:tt
                    input = [{ $other_kernel:literal $(| $($rest:tt)* )? }]
                ) => {
                    tt_is_kernel! {
                        $caller
                        input = [{ $( $($rest)* )? }]
                    }
                },
                (
                    $caller:tt
                    input = [{ }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ false }]
                    }
                },
            }
            
            /// Get the kernel selected by a Cargo feature.
            ///
            /// # Example
            ///
            /// ```
            /// let kernel = tt_call::tt_call! { macro = [{ itron::macros::tt_kernel }] };
            ///
            /// println!("We are running on {}", kernel);
            /// ```
            ///
            #[cfg(feature = "none")]
            pub macro tt_kernel($caller:tt) {
                tt_call::tt_return! {
                    $caller
                    output = [{ "none" }]
                }
            }
            
            /// Determine if this crate was compiled for the specified kernel.
            ///
            /// # Example
            ///
            /// ```
            /// tt_call::tt_if! {
            ///     condition = [{ itron::macros::tt_is_kernel }]
            ///     input = [{ "asp3" }]
            ///     true = [{ println!("We are on TOPPERS/ASP3, yay!"); }]
            ///     false = [{}]
            /// }
            /// ```
            ///
            #[cfg(feature = "none")]
            pub macro tt_is_kernel {
                (
                    $caller:tt
                    input = [{ "none" $(| $($rest:tt)* )? }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ true }]
                    }
                },
                (
                    $caller:tt
                    input = [{ $other_kernel:literal $(| $($rest:tt)* )? }]
                ) => {
                    tt_is_kernel! {
                        $caller
                        input = [{ $( $($rest)* )? }]
                    }
                },
                (
                    $caller:tt
                    input = [{ }]
                ) => {
                    tt_call::tt_return! {
                        $caller
                        is = [{ false }]
                    }
                },
            }
            