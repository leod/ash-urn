#[allow(dead_code)]
#[derive(Debug)]
pub enum UrnError {
    GenericDynamic(String),
    Generic(&'static str),
    AshError(ash::vk::Result),
    AshInstanceError(ash::InstanceError),
    NulError(std::ffi::NulError),
}

impl From<std::ffi::NulError> for UrnError {
    fn from(e: std::ffi::NulError) -> UrnError {
        UrnError::NulError(e)
    }
}

impl From<ash::vk::Result> for UrnError {
    fn from(e: ash::vk::Result) -> UrnError {
        UrnError::AshError(e)
    }
}

impl From<ash::InstanceError> for UrnError {
    fn from(e: ash::InstanceError) -> UrnError {
        UrnError::AshInstanceError(e)
    }
}
