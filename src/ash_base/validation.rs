use crate::error::UrnError;
use crate::util::vk_to_string;

use ash::version::EntryV1_0;

pub struct Validation {
    pub is_enabled: bool,
    pub required_layer_names: &'static [&'static str],
}

pub const VALIDATION: Validation = Validation {
    is_enabled: cfg!(debug_assertions),
    required_layer_names: &["VK_LAYER_KHRONOS_validation"],
};

unsafe extern "system" fn vulkan_debug_utils_callback(
    message_severity: ash::vk::DebugUtilsMessageSeverityFlagsEXT,
    message_type: ash::vk::DebugUtilsMessageTypeFlagsEXT,
    p_callback_data: *const ash::vk::DebugUtilsMessengerCallbackDataEXT,
    _p_user_data: *mut std::ffi::c_void,
) -> ash::vk::Bool32 {
    let severity = match message_severity {
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE => "[Verbose]",
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING => "[Warning]",
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR => "[Error]",
        ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO => "[Info]",
        _ => "[Unknown]",
    };
    let types = match message_type {
        ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL => "[General]",
        ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE => "[Performance]",
        ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION => "[Validation]",
        _ => "[Unknown]",
    };
    let message = std::ffi::CStr::from_ptr((*p_callback_data).p_message);
    println!("[Debug]{}{}{:?}", severity, types, message);

    ash::vk::FALSE
}

pub fn populate_debug_messenger_create_info() -> ash::vk::DebugUtilsMessengerCreateInfoEXT {
    ash::vk::DebugUtilsMessengerCreateInfoEXT::builder()
        .message_severity(
            ash::vk::DebugUtilsMessageSeverityFlagsEXT::WARNING
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::VERBOSE
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::INFO
                | ash::vk::DebugUtilsMessageSeverityFlagsEXT::ERROR,
        )
        .message_type(
            ash::vk::DebugUtilsMessageTypeFlagsEXT::GENERAL
                | ash::vk::DebugUtilsMessageTypeFlagsEXT::PERFORMANCE
                | ash::vk::DebugUtilsMessageTypeFlagsEXT::VALIDATION,
        )
        .pfn_user_callback(Some(vulkan_debug_utils_callback))
        .build()
}

pub fn check_validation_layer_support(ash_entry: &ash::Entry) -> Result<(), UrnError> {
    let layer_properties = ash_entry.enumerate_instance_layer_properties()?;

    if layer_properties.is_empty() {
        return Err(UrnError::Generic("No available layers."));
    }

    for layer_needed in VALIDATION.required_layer_names.iter() {
        let mut is_layer_found = false;
        for layer in layer_properties.iter() {
            if *layer_needed == vk_to_string(&layer.layer_name) {
                is_layer_found = true;
                break;
            }
        }

        if !is_layer_found {
            return Err(UrnError::Generic("Validation layer not found."));
        }
    }

    Ok(())
}