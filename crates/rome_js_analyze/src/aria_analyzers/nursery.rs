//! Generated file, do not edit by hand, see `xtask/codegen`

use rome_analyze::declare_group;
mod no_noninteractive_element_to_interactive_role;
mod use_aria_prop_types;
mod use_aria_props_for_role;
declare_group! { pub (crate) Nursery { name : "nursery" , rules : [self :: no_noninteractive_element_to_interactive_role :: NoNoninteractiveElementToInteractiveRole , self :: use_aria_prop_types :: UseAriaPropTypes , self :: use_aria_props_for_role :: UseAriaPropsForRole ,] } }
