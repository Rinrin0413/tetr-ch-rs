/// A macro to implement the methods for `role` field.
///
/// # Methods
///
/// ```ignore
/// pub fn is_normal_user(&self) -> bool
/// pub fn is_anon(&self) -> bool
/// pub fn is_bot(&self) -> bool
/// pub fn is_sysop(&self) -> bool
/// pub fn is_admin(&self) -> bool
/// pub fn is_mod(&self) -> bool
/// pub fn is_halfmod(&self) -> bool
/// pub fn is_banned(&self) -> bool
/// pub fn is_hidden(&self) -> bool
/// ```
///
/// # Dependencies
///
/// - `role: Role` field
///
/// Go to [Role](crate::model::util::role::Role)
macro_rules! impl_for_role {
    () => {
        /// Whether the user is a normal user.
        pub fn is_normal_user(&self) -> bool {
            self.role.is_normal_user()
        }

        /// Whether the user is an anonymous.
        pub fn is_anon(&self) -> bool {
            self.role.is_anon()
        }

        /// Whether the user is a bot.
        pub fn is_bot(&self) -> bool {
            self.role.is_bot()
        }

        /// Whether the user is a SYSOP.
        pub fn is_sysop(&self) -> bool {
            self.role.is_sysop()
        }

        /// Whether the user is an administrator.
        pub fn is_admin(&self) -> bool {
            self.role.is_admin()
        }

        /// Whether the user is a moderator.
        pub fn is_mod(&self) -> bool {
            self.role.is_mod()
        }

        /// Whether the user is a community moderator.
        pub fn is_halfmod(&self) -> bool {
            self.role.is_halfmod()
        }

        /// Whether the user is banned.
        pub fn is_banned(&self) -> bool {
            self.role.is_banned()
        }

        /// Whether the user is hidden.
        pub fn is_hidden(&self) -> bool {
            self.role.is_hidden()
        }
    };
}
