use directories::ProjectDirs;

mod log_utils;
pub mod namespace;
pub mod plugin;
pub mod repository;
pub mod template;

mod macros {
    macro_rules! generate_wrap_error {
        ($error_name:ident, $trait_name:ident) => {
            trait $trait_name<T> {
                fn wrap(self, msg: impl ToString) -> Result<T, $error_name>;
            }

            impl<T, E> $trait_name<T> for Result<T, E>
            where
                $error_name: From<E>,
            {
                fn wrap(self, msg: impl ToString) -> Result<T, $error_name> {
                    self.map_err(|e| {
                        $error_name::Wrapped(Box::new($error_name::from(e)), msg.to_string())
                    })
                }
            }
        };
    }
    pub(crate) use generate_wrap_error;
}

pub fn project_dirs() -> ProjectDirs {
    ProjectDirs::from("", "", "fig")
        .expect("Failed to find home directory, maybe your operating system is unsupported?")
}
