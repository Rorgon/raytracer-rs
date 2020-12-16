use criterion::{black_box, criterion_group, criterion_main, Criterion};
use raytracer::{
    image::png,
    vec3::Vec3,
    scene::{
        Scene, 
        sphere::Sphere, 
        camera::Camera,
    },
    ray::material::{ 
        Lambertian , 
        Metallic
    },
};
use std::{
    sync::Arc, 
    error::Error
};
use core::f64::consts::PI;


pub fn scene(){
    let height: usize = 100;
    let width: usize = 200;
    let samples_per_pixel : u64 = 50;
    let max_depth : u64 = 50;


    let vfov: f64 = PI/2.0;
    let aspect: f64 = (width as f64)/(height as f64);
    let look_from: Vec3 = Vec3(0.0,0.0,0.0);
    let look_to: Vec3 = Vec3(0.0,0.0,-1.0);
    let look_up: Vec3 = Vec3(0.0,1.0,0.0);

    let mut scene = Scene::new(Camera::new(look_from,look_to,look_up,vfov,aspect));
    scene.add(Box::new(Sphere::new(Vec3(0.0,0.0,-1.0),0.5,
                                   Arc::new(Lambertian::new(Vec3(0.0,0.0,1.0))))));
    scene.add(Box::new(Sphere::new(Vec3(0.0,-100.5,-1.0),100.0,
                                   Arc::new(Lambertian::new(Vec3(0.8,0.8,0.0))))));

    scene.add(Box::new(Sphere::new(Vec3(1.0,0.0,-1.0),0.5,
                                   Arc::new(Metallic::new(Vec3(0.8,0.6,0.2),0.0)))));
    scene.add(Box::new(Sphere::new(Vec3(-1.0,0.0,-1.0),0.5,
                                   Arc::new(Metallic::new(Vec3(0.8,0.6,0.8),0.5)))));

    let image = scene.render(width, height, samples_per_pixel, max_depth);
    black_box(image);
}

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("scene", |b| b.iter(|| scene()));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
