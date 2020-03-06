pub mod vec3;
pub mod image;
pub mod ray;
pub mod scene;




#[cfg(test)]
mod tests {

    use super::vec3::Vec3;

    #[test]
    fn add_and_sub_vec3() {
        let vec1 = Vec3(1.0, 1.0, 1.0);
        let vec2 = Vec3(0.5, 0.0, 2.0);
        assert_eq!(
            vec1+vec2, 
            Vec3(1.5, 1.0, 3.0)
        );
        assert_eq!(
            vec1-vec2, 
            Vec3(0.5, 1.0, -1.0)
        );
    }
}