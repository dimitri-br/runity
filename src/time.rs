/// # Time
///
/// This contains all time related variables and functions - including things such as deltatime and fixedtime.

/// # Time
///
/// Stores all time values. Look at unity docs for what is modifiable and what is read-only.
#[repr(C)]
#[derive(Debug, Clone, Copy)]
pub struct Time{
    pub delta_time: f32,
    pub fixed_delta_time: f32,
    pub fixed_time: f32,
    pub fixed_unscaled_delta_time: f32,
    pub fixed_unscaled_time: f32,
    pub frame_count: f32,
    pub maximum_delta_time: f32,
    pub maximum_particle_delta_time: f32,
    pub real_time_since_startup: f32,
    pub smooth_delta_time: f32,
    pub time: f32,
    pub time_scale: f32,
    pub time_since_level_load: f32,
    pub unscaled_delta_time: f32,
    pub unscaled_time: f32
}

impl Default for Time{
    fn default() -> Self{
        Self{
            delta_time: 0.0,
            fixed_delta_time: 0.0,
            fixed_time: 0.0,
            fixed_unscaled_delta_time: 0.0,
            fixed_unscaled_time: 0.0,
            frame_count: 0.0,
            maximum_delta_time: 0.0,
            maximum_particle_delta_time: 0.0,
            real_time_since_startup: 0.0,
            smooth_delta_time: 0.0,
            time: 0.0,
            time_scale: 0.0,
            time_since_level_load: 0.0,
            unscaled_delta_time: 0.0,
            unscaled_time: 0.0
        }
    }
}