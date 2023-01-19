pub type V2 = cgmath::Vector2<f32>;
pub type V3 = cgmath::Vector3<f32>;
pub type V4 = cgmath::Vector4<f32>;

pub fn v2_rotate_about_v2(point: &V2, origin: &V2, s: f32, c: f32) -> V2 {
    let mut result = V2::new(point.x, point.y);
    result = result - origin;

    let xnew = (result.x * c) - (result.y * s);
    let ynew = (result.x * s) + (result.y * c);

    result.x = xnew + origin.x;
    result.y = ynew + origin.y;

    return result;
}