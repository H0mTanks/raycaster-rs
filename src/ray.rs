use crate::prelude::*;

#[derive(Clone)]
pub struct Ray {
    pub ray_angle: f32,
    pub wall_hit_x: f32,
    pub wall_hit_y: f32,
    pub distance: f32,
    pub was_hit_vertical: bool,
    pub is_ray_facing_up: bool,
    pub is_ray_facing_down: bool,
    pub is_ray_facing_left: bool,
    pub is_ray_facing_right: bool,
    pub wall_hit_content: u8,
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

    pub fn new_with(
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
    ) -> Self {
        Ray {
            ray_angle,
            wall_hit_x,
            wall_hit_y,
            distance,
            was_hit_vertical,
            is_ray_facing_up,
            is_ray_facing_down,
            is_ray_facing_left,
            is_ray_facing_right,
            wall_hit_content,
        }
    }

    fn normalize_angle(mut angle: f32) -> f32 {
        angle = angle % (2.0 * PI);
        if angle < 0.0 {
            angle = 2.0 * PI + angle;
        }

        angle
    }

    fn distance_between_points(x1: f32, y1: f32, x2: f32, y2: f32) -> f32 {
        f32::sqrt((x2 - x1) * (x2 - x1) + (y2 - y1) * (y2 - y1))
    }

    fn cast_ray(game_state: &GameState, mut ray_angle: f32) -> Ray {
        ray_angle = Ray::normalize_angle(ray_angle);

        let is_ray_facing_down = ray_angle > 0.0 && ray_angle < PI;
        let is_ray_facing_up = !is_ray_facing_down;
        let is_ray_facing_right = ray_angle < 0.5 * PI || ray_angle > 1.5 * PI;
        let is_ray_facing_left = !is_ray_facing_right;

        //* Horizontal ray grid intersection */
        let mut found_horz_wall_hit = false;
        let mut horz_wall_hit_x: f32 = 0.0;
        let mut horz_wall_hit_y: f32 = 0.0;
        let mut horz_wall_content: u8 = 0;

        //* Find the y-coordinate of the closest horizontal grid intersection
        let mut y_intercept: f32 =
            f32::floor(game_state.player.y / TILE_SIZE as f32) * TILE_SIZE as f32;
        y_intercept += if is_ray_facing_down {
            TILE_SIZE as f32
        } else {
            0.0
        };

        //*  Find the x-coordinate of the closest horizontal grid intersection
        let x_intercept =
            game_state.player.x + (y_intercept - game_state.player.y) / f32::tan(ray_angle);

        //* calculate the increment of xstep and ystep */
        let mut ystep: f32 = TILE_SIZE as f32;
        ystep *= if is_ray_facing_up { -1.0 } else { 1.0 };

        let mut xstep = TILE_SIZE as f32 / f32::tan(ray_angle);
        xstep *= if is_ray_facing_left && xstep > 0.0 {
            -1.0
        } else {
            1.0
        };

        xstep *= if is_ray_facing_right && xstep < 0.0 {
            -1.0
        } else {
            1.0
        };

        let mut next_horz_touch_x = x_intercept;
        let mut next_horz_touch_y = y_intercept;

        //* increment xstep and ystep until we find a wall
        while next_horz_touch_x >= 0.0
            && next_horz_touch_x < WINDOW_WIDTH as f32
            && next_horz_touch_y >= 0.0
            && next_horz_touch_y < WINDOW_HEIGHT as f32
        {
            let x_tocheck: f32 = next_horz_touch_x;
            let y_tocheck: f32 = next_horz_touch_y + if is_ray_facing_up { -1.0 } else { 0.0 };

            if game_state.map.has_wall_at(x_tocheck, y_tocheck) {
                horz_wall_hit_x = next_horz_touch_x;
                horz_wall_hit_y = next_horz_touch_y;
                horz_wall_content = game_state.map.get(
                    (x_tocheck / TILE_SIZE as f32) as u32,
                    (y_tocheck / TILE_SIZE as f32) as u32,
                );
                found_horz_wall_hit = true;
                break;
            } else {
                next_horz_touch_x += xstep;
                next_horz_touch_y += ystep;
            }
        }

        //* Vertical ray grid intersection */
        let mut found_vert_wall_hit = false;
        let mut vert_wall_hit_x: f32 = 0.0;
        let mut vert_wall_hit_y: f32 = 0.0;
        let mut vert_wall_content: u8 = 0;

        //* Find the x-coordinate of the closest vertical grid intersection
        let mut x_intercept: f32 =
            f32::floor(game_state.player.x / TILE_SIZE as f32) * TILE_SIZE as f32;
        x_intercept += if is_ray_facing_right {
            TILE_SIZE as f32
        } else {
            0.0
        };

        //*  Find the y-coordinate of the closest vertical grid intersection
        let y_intercept =
            game_state.player.y + (x_intercept - game_state.player.x) * f32::tan(ray_angle);

        //* calculate the increment of xstep and ystep */
        let mut xstep: f32 = TILE_SIZE as f32;
        xstep *= if is_ray_facing_left { -1.0 } else { 1.0 };

        let mut ystep = TILE_SIZE as f32 * f32::tan(ray_angle);
        ystep *= if is_ray_facing_up && ystep > 0.0 {
            -1.0
        } else {
            1.0
        };

        ystep *= if is_ray_facing_down && ystep < 0.0 {
            -1.0
        } else {
            1.0
        };

        let mut next_vert_touch_x = x_intercept;
        let mut next_vert_touch_y = y_intercept;

        //* increment xstep and ystep until we find a wall
        while next_vert_touch_x >= 0.0
            && next_vert_touch_x < WINDOW_WIDTH as f32
            && next_vert_touch_y >= 0.0
            && next_vert_touch_y < WINDOW_HEIGHT as f32
        {
            let x_tocheck: f32 = next_vert_touch_x + if is_ray_facing_left { -1.0 } else { 0.0 };
            let y_tocheck: f32 = next_vert_touch_y;

            if game_state.map.has_wall_at(x_tocheck, y_tocheck) {
                vert_wall_hit_x = next_vert_touch_x;
                vert_wall_hit_y = next_vert_touch_y;
                vert_wall_content = game_state.map.get(
                    (x_tocheck / TILE_SIZE as f32) as u32,
                    (y_tocheck / TILE_SIZE as f32) as u32,
                );
                found_vert_wall_hit = true;
                break;
            } else {
                next_vert_touch_x += xstep;
                next_vert_touch_y += ystep;
            }
        }

        //* Calculate both horizontal and vertical hit distances and choose the smallest one
        let horz_hit_distance = if found_horz_wall_hit {
            Ray::distance_between_points(
                game_state.player.x,
                game_state.player.y,
                horz_wall_hit_x,
                horz_wall_hit_y,
            )
        } else {
            f32::MAX
        };

        let vert_hit_distance = if found_vert_wall_hit {
            Ray::distance_between_points(
                game_state.player.x,
                game_state.player.y,
                vert_wall_hit_x,
                vert_wall_hit_y,
            )
        } else {
            f32::MAX
        };

        if vert_hit_distance < horz_hit_distance {
            Ray::new_with(
                ray_angle,
                vert_wall_hit_x,
                vert_wall_hit_y,
                vert_hit_distance,
                true,
                is_ray_facing_up,
                is_ray_facing_down,
                is_ray_facing_left,
                is_ray_facing_right,
                vert_wall_content,
            )
        } else {
            Ray::new_with(
                ray_angle,
                horz_wall_hit_x,
                horz_wall_hit_y,
                horz_hit_distance,
                false,
                is_ray_facing_up,
                is_ray_facing_down,
                is_ray_facing_left,
                is_ray_facing_right,
                horz_wall_content,
            )
        }
    }

    pub fn cast_all_rays(game_state: &mut GameState) {
        let mut ray_angle: f32 = game_state.player.rotation_angle - (FOV_ANGLE / 2.0);

        for column_id in 0..NUM_RAYS {
            game_state.rays[column_id as usize] = Ray::cast_ray(game_state, ray_angle); //* remember to clear rays after render */
            ray_angle += FOV_ANGLE / NUM_RAYS as f32;
        }
    }

    pub fn render_rays(app: &mut App, game_state: &mut GameState) {
        app.renderer.set_draw_color(Color::RGBA(255, 0, 0, 255));
        for i in 0..NUM_RAYS {
            app.draw_line(
                (MINIMAP_SCALE_FACTOR * game_state.player.x) as i32,
                (MINIMAP_SCALE_FACTOR * game_state.player.y) as i32,
                (MINIMAP_SCALE_FACTOR * game_state.rays[i as usize].wall_hit_x) as i32,
                (MINIMAP_SCALE_FACTOR * game_state.rays[i as usize].wall_hit_y) as i32,
                0xFF0000FF,
            );
        }
    }
}
