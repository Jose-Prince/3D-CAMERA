use nalgebra_glm::{Vec3, normalize, cross, magnitude};
use std::f32::consts::PI;

pub struct Camera {
    pub eye: Vec3, // Camera position in world space
    pub center: Vec3, // Point the camera is looking at
    pub up: Vec3, // Up vector
}

impl Camera {
    pub fn basis_change(&self, vector: &Vec3) -> Vec3 {
        let forward = (self.center - self.eye).normalize();
        let right = forward.cross(&self.up).normalize();
        let up = right.cross(&forward).normalize();

        let rotated = 
        vector.x * right + 
        vector.y * up -
        vector.z * forward;

        rotated.normalize()
    }

    pub fn orbit(&mut self, delta_yaw: f32, delta_pitch: f32) {
        // Calcular el vector desde el centro hasta el ojo (vector de radio) y medir la distancia
        let radius_vector = self.eye - self.center;
        let radius = radius_vector.magnitude();

        // Calcular yaw actual (rotación alrededor del eje Y)
        let current_yaw = radius_vector.z.atan2(radius_vector.x);

        // Calcular pitch actual (rotación alrededor del eje X)
        let radius_xz = (radius_vector.x * radius_vector.x + radius_vector.z * radius_vector.z).sqrt();
        let current_pitch = (-radius_vector.y).atan2(radius_xz);

        // Aplicar rotaciones delta
        let new_yaw = (current_yaw + delta_yaw) % (2.0 * PI);
        let new_pitch = (current_pitch + delta_pitch).clamp(-PI / 2.0 + 0.1, PI / 2.0 - 0.1);

        // Calcular la nueva posición del ojo
        let new_eye = self.center + Vec3::new(
            radius * new_yaw.cos() * new_pitch.cos(),
            -radius * new_pitch.sin(),
            radius * new_yaw.sin() * new_pitch.cos()
        );

        self.eye = new_eye;
    }
}
