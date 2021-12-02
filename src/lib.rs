//! Roopert is an open-source toolkit for object-oriented programming patterns. 
//! Spend less time writing boilerplate and more time implementing functionality! 
//! 
//! ## Attributes
//! | [parent](#parent) | [extends](#extends) | [accessors](#accessors) | [get](#get) | [set](#set) |
//! | --- | --- | --- | --- | --- |
//!
//! ### parent
//! A field-level attribute for indicating which field is the parent of the struct in conjunction with the `extends` attribute macro.
//! This attribute macro takes no other parameters.
//! The standard form `#[roopert(parent)]` as well as a shortened form `#[parent]` macros may be used.
//! The `extends` attribute will automatically resolve types to fields, but the `parent` attribute overrides the default behaviour.
//!
//! ### extends
//! A struct-level attribute for making a named struct "extend" functionality of another Rust type.
//! Rust types are supplied as parameters to indicate which type(s) the struct extends.
//! A field with the same type must also be in the struct.
//! The standard form `#[roopert(extends)]` attribute macro is used, and the `#[roopert(parent)]` attribute can be used on a field to explicitly declare the parent.
//!
//! Extends or inheritance-like behaviour is accomplished by an automatic implementation of `AsRef`, `AsMut`, `Into`, `Deref` and `DerefMut` for the struct this attribute is applied to.
//! This creates smart-pointer behaviour along with the ability to explicitly downcast.
//! 
//! ```
//! # use roopert::roopert;
//! #[roopert(extends, String)]
//! struct MyStruct {
//!     not_parent_field: String, // ignored
//!
//!     #[roopert(parent)]
//!     parent_field: String,
//!
//!     another_field: String, // also ignored
//! }
//!
//! // automatically generated AsRef implementation by Roopert
//! // (not shown: AsMut, Into, Deref, and DerefMut implementations)
//! # /*
//! impl AsRef<String> for MyStruct {
//!     fn as_ref(&self) -> &String {
//!         &self.parent_field
//!     }
//! }
//! # */
//! # fn main() {}
//! ```
//!
//! ### accessors
//! A struct-level attribute for automatically creating getters and setters for fields of a named struct.
//! Optionally, this attribute accepts one or two parameters (in any order): `get = rule` and `set = rule`,
//! where rule can be one of `All` (generate all accessors), `Private` (generate accessors for all private fields), `No` (don't generate -- default).
//! Additionally, the get and set attribute types can be used to override the struct-level getter and setter rule, respectively.
//!
//! ```
//! # use roopert::roopert;
//! #[roopert(accessors, get = All)]
//! struct MyStruct {
//!     #[roopert(set)]
//!     my_field: String
//! }
//!
//! // automatically generated by Roopert
//! # /*
//! impl MyStruct {
//!     // generated from get rule in struct attribute
//!     pub fn get_my_field(&self) -> &String {
//!         &self.my_field
//!     }
//!
//!     // generated from field attribute
//!     pub fn set_my_field(&mut self, x: String) {
//!         self.my_field = x;
//!     }
//! }
//! # */
//! # fn main() {}
//! ```
//! 
//! ### get
//! A field-level attribute for overriding accessors attribute behaviour for getters methods.
//! Optionally, `pre = operation` can be supplied to do an operation before the get function returns.
//! The optional parameter `mutable = true` can be supplied to get a mutable reference (as well as allow mutable `self` operations with the pre parameter).
//! The optional parameter `name = "getter_name"` can be used to specify a custom get function name (defaults to `get_<field name>`).
//! The standard form `#[roopert(get)]` or the shortened `#[get]` attribute macro may be used.
//! **Note**: this doesn't work without `#[roopert(accessors)]` on the struct.
//!
//! ```
//! # use roopert::roopert;
//! #[roopert(accessors)]
//! struct MyStruct {
//!     #[roopert(get, mutable = true, pre = self.before_get_my_field(), name = "get_field" )]
//!     my_field: String,
//!     my_field_is_borrowed: bool
//! }
//!
//! impl MyStruct {
//!     fn before_get_my_field(&mut self) { // note that this function uses `&mut self` because mutable = true
//!         self.my_field_is_borrowed = true;
//!     }
//! }
//!
//! // automatically generated by Roopert
//! # /*
//! impl MyStruct {
//!     // generated from get rule in field attribute
//!     pub fn get_field(&mut self) -> &mut String {
//!         self.before_get_my_field(); // from `pre = self.pre_get_my_field()`
//!         &mut self.my_field
//!     }
//! }
//! # */
//! # fn main() {}
//! ```
//! 
//! ### set
//! A field-level attribute for overriding accessors attribute behaviour for setter methods.
//! Optionally, `pre = operation` and `post = operation` can be used to do an operation before and after the variable is set, respectively.
//! The optional parameter `name = "setter_name"` can be used to specify a custom set function name (defaults to `set_<field name>`).
//! The standard form `#[roopert(set)]` or shortened the `#[set]` attribute macro may be used.
//! **Note**: this doesn't work without `#[roopert(accessors)]` on the struct.
//! 
//! ```
//! # use roopert::roopert;
//! #[roopert(accessors)]
//! struct MyStruct {
//!     #[roopert(set, pre = self.before_set_my_field(), post = self.after_set_my_field())]
//!     my_field: String,
//!     setting_my_field: bool, // this will briefly be true (but unobservable unless you break Rust safety)
//! }
//! 
//! impl MyStruct {
//!     fn before_set_my_field(&mut self) { // these can always use &mut self, unlike getters
//!         self.setting_my_field = true;
//!     }
//! 
//!     fn after_set_my_field(&mut self) {
//!         self.setting_my_field = false;
//!     }
//! }
//!
//! // automatically generated by Roopert
//! # /*
//! impl MyStruct {
//!
//!     // generated from field attribute
//!     pub fn set_my_field(&mut self, x: String) {
//!         self.my_field = x;
//!     }
//! }
//! # */
//! # fn main() {}
//! ```
//!

#![warn(missing_docs)]

#[cfg(feature = "macro_root")]
pub use macro_root::*;


#[cfg(feature = "macro_parent")]
pub use macro_parent::*;
