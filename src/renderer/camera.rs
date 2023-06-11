use bevy::prelude::*;
use std::ptr;


const YAW: f32 = -90.0;
const PITCH: f32 = 0.0;
const SPEED: f32 = 10.0;
const ROT_SPEED: f32 = 50.0;
const ZOOM: f32 = 45.0;

pub enum CameraMoveDirection {
    None,
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Component)]
pub struct Camera {
    pub position: Vec3,
    pub forward: Vec3,
    pub up: Vec3,
    pub right: Vec3,
    pub world_up: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub zoom: f32,
}

#[derive(Component)]
pub struct CameraMovement {
    pub direction: CameraMoveDirection,
    pub speed: f32,
    pub constrain_pitch: bool,
    pub rotation_speed: f32,
}

#[derive(Bundle)]
pub struct CameraBundle {
    pub camera: Camera,
    pub movement: CameraMovement,
}

impl Default for Camera {
    fn default() -> Self {
        let mut camera = Self {
            position: Vec3::new(0.0, 0.0, 0.0),
            forward: Vec3::NEG_Z,
            up: Vec3::Y,
            right: Vec3::X,
            world_up: Vec3::Y,
            yaw: YAW,
            pitch: PITCH,
            zoom: ZOOM,
        };
        camera.update_vectors();
        camera
    }
}

impl Camera {
    pub fn get_view_mat(&self) -> Mat4 {
        Mat4::look_at_rh(self.position, self.position + self.forward, self.up)
    }

    pub fn from_position(x: f32, y: f32, z: f32) -> Self {
        Camera {
            position: Vec3::new(x, y, z),
            ..default()
        }
    }
    
    // this function doesn't work yet because zoom is not used for anything yet
    pub fn process_mouse_scroll(&mut self, mouse_scroll: f32) {
        if self.zoom >= 1.0 && self.zoom <= 45.0 {
            self.zoom -= mouse_scroll;
        } else if self.zoom <= 1.0 {
            self.zoom = 1.0;
        } else if self.zoom >= 45.0 {
            self.zoom = 45.0;
        }
    }
    
    pub fn process_mouse_movement(&mut self,
        mouse_pos: Vec2,
        prev_mouse_pos: Vec2,
        cam_move: &CameraMovement,
        time: &Time,
    ) {
        let mut x_offset = mouse_pos.x - prev_mouse_pos.x;
        let mut y_offset = -(mouse_pos.y - prev_mouse_pos.y);
        x_offset *= cam_move.rotation_speed * time.delta_seconds();
        y_offset *= cam_move.rotation_speed * time.delta_seconds();
        
        self.yaw += x_offset;
        self.pitch += y_offset;
        
        if cam_move.constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            } else if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }
        
        self.update_vectors();
    }
    
    pub fn process_movement(&mut self,
        local_move_dir: Vec3,
        movement: &CameraMovement,
        time: &Time
    ) {
        let dir_vector: Vec3 = if local_move_dir.x != 0.0 || local_move_dir.z != 0.0 {
            (self.forward * -local_move_dir.z + self.right * local_move_dir.x).normalize()
        } else {
            Vec3::ZERO
        };

        let delta_pos = movement.speed * time.delta_seconds();
        self.position += dir_vector * delta_pos;
    }

    fn update_vectors(&mut self) {
        let forward = Vec3::new(
            self.yaw.to_radians().cos() * self.pitch.to_radians().cos(),
            self.pitch.to_radians().sin(),
            self.yaw.to_radians().sin() * self.pitch.to_radians().cos(),
        );
        self.forward = forward.normalize();
        self.right = self.forward.cross(self.world_up).normalize();
        self.up = self.right.cross(self.forward).normalize();
    }
    
    pub fn process_rotation(&mut self,
        pitch: f32,
        yaw: f32, movement:
        &CameraMovement,
        time: &Time
    ) {
        let delta_pitch = pitch * movement.rotation_speed * time.delta_seconds();
        let delta_yaw = yaw * movement.rotation_speed * time.delta_seconds();

        self.pitch += delta_pitch;
        self.yaw += delta_yaw;

        if movement.constrain_pitch {
            if self.pitch > 89.0 {
                self.pitch = 89.0;
            } else if self.pitch < -89.0 {
                self.pitch = -89.0;
            }
        }
        
        self.update_vectors();
    }

}


impl Default for CameraMovement {
    fn default() -> Self {
        Self {
            direction: CameraMoveDirection::None,
            speed: SPEED,
            constrain_pitch: false,
            rotation_speed: ROT_SPEED,
        }
    }
}

impl Default for CameraBundle {
    fn default() -> Self {
        Self { camera: Camera::default(), movement: CameraMovement::default() }
    }
}