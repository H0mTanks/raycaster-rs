#[derive(Clone)]
pub struct Ray {
    ray_angle: f32,
    wall_hit_x: f32,
    wall_hit_y: f32,
    distance: f32,
    was_hit_vertical: bool,
    is_ray_facing_up: bool,
    is_ray_facing_down: bool,
    is_ray_facing_left: bool,
    is_ray_facing_right: bool,
    wall_hit_content: u8,
}

impl Ray {
    pub fn new() -> Self {
        Ray {
            ray_angle: 0.0,
            wall_hit_x: 0.0,
            wall_hit_y: 0.0,
            distance: 0.0,
            was_hit_vertical: false,
            is_ray_facing_up: false,
            is_ray_facing_down: false,
            is_ray_facing_left: false,
            is_ray_facing_right: false,
            wall_hit_content: 0,
        }
    }
}
