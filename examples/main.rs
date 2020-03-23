mod sdl;

use ash_urn::base::{Entry, Instance, LogicalDevice, PhysicalDevice, Validation};

fn main() {
    let sdl = sdl::SDL::new(sdl::WindowSettings {
        title: "Test",
        w: 800,
        h: 800,
        maximized: false,
    })
    .unwrap();

    let mut instance_extension_names = sdl.required_extension_names().unwrap();
    instance_extension_names.push(ash::extensions::ext::DebugUtils::name().to_str().unwrap());

    let entry = Entry::new().unwrap();
    let instance = Instance::new("Test", 1, 2, 131, &instance_extension_names, &entry.0).unwrap();
}