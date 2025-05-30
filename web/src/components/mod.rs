// mod.rs
pub mod button;
pub mod input;
pub mod card;
pub mod inline_form;
pub mod hero;
// pub mod toast;
pub mod steps;
pub mod accordion;
pub mod loading_spinner;
pub mod icons;
pub mod team_member;
// pub mod program_accordion;
pub mod resource_table;



// Re-export from modules
pub use resource_table::{ResourceTable, ResourceItem};
// pub use program_accordion::ProgramsAccordion;
pub use button::{Button, ButtonSize, ButtonScheme, ButtonType};
pub use card::{Card, CardTitle, CardBody};

use dioxus::html::mo;
pub use input::{Input, InputSize, InputType, InputProps, TextInput, PasswordInput, DateInput, NumberInput,SelectInput};
pub use inline_form::{InlineForm, InlineFormProps};
pub use steps::Steps;
pub use hero::Hero;
pub use icons::{BuildingIcon, HomeGarage, ArrowRight, HomePerson, HomeCheck, HomeHeart};
pub use team_member::TeamMember;
// pub use toast::{ToastFrame, ToastFrameProps, ToastManager, ToastInfo, ToastScheme, ToastSize, Position};
pub use loading_spinner::{
    SpinnerSize,
    SpinnerColor,
    SpinnerProps
};
// Optional prelude module for convenient imports
// pub mod prelude {
//     pub use super::{
//         ToastFrame, 
//         ToastFrameProps, 
//         ToastManager,
//         ToastInfo,
//         ToastScheme,
//         ToastSize,
//         Position
//     };
    
//     // Convenience functions for common toast types
//     pub fn simple_toast(text: &str) -> ToastInfo {
//         ToastInfo::simple(text)
//     }
    
//     pub fn success_toast(text: &str, heading: Option<&str>) -> ToastInfo {
//         ToastInfo::success(text, heading)
//     }
    
//     pub fn warning_toast(text: &str, heading: Option<&str>) -> ToastInfo {
//         ToastInfo::warning(text, heading)
//     }
    
//     pub fn error_toast(text: &str, heading: Option<&str>) -> ToastInfo {
//         ToastInfo::error(text, heading)
//     }
    
//     pub fn info_toast(text: &str, heading: Option<&str>) -> ToastInfo {
//         ToastInfo::info(text, heading)
//     }
// }