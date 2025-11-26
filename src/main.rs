mod color;
mod ray;
mod vec3;

use color::{write_color, Color};
use ray::Ray;
use vec3::{Point3, Vec3};

pub fn ray_color(r: &Ray) -> Color {
    let unit_direction: Vec3 = vec3::unit_vector(r.direction());
    let a = 0.5 * (unit_direction.y() + 1.);
    (1. - a) * Color::new(1., 1., 1.) + a * Color::new(0.5, 0.7, 1.0)
}

fn main() {
    //Image
    const ASPECT_RATIO: f64 = 16. / 9.;
    const IMAGE_WIDTH: i32 = 400;

    //Calculate the image height and ensure that it's atleast 1.
    //TODO: Ensure that it is atleast 1.

    const IMAGE_HEIGHT: i32 = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as i32;

    // Camera
    let focal_length = 1.;
    let viewport_height = 2.;
    let viewport_width = viewport_height * (IMAGE_WIDTH / IMAGE_HEIGHT) as f64;
    let camera_center = Point3::new(0., 0., 0.);

    // Calculate the vector across the horizontal and down the vertical viewport edtes.
    let viewport_u = Vec3::new(viewport_width, 0., 0.);
    let viewport_v = Vec3::new(0., viewport_height, 0.);

    // Calculate the vertical and horizontal delta i.e. distance from pixel to pixel.
    let pixel_delta_u = viewport_u / IMAGE_WIDTH as f64;
    let pixel_delta_v = viewport_v / IMAGE_HEIGHT as f64;

    //Calcualte the location of upper left pixel
    let viewport_upper_left =
        camera_center - Vec3::new(0., 0., focal_length) - viewport_u / 2. - viewport_v / 2.;
    let pixel00_loc = viewport_upper_left + 0.5 * (pixel_delta_u + pixel_delta_v);

    // Render

    print!("P3\n{} {}\n255\n", IMAGE_WIDTH, IMAGE_HEIGHT);

    for j in (0..IMAGE_HEIGHT).rev() {
        eprint!("\rScanlines remaining: {} ", j);
        for i in 0..IMAGE_WIDTH {
            let pixel_center =
                pixel00_loc + (i as f64 * pixel_delta_u) + (j as f64 * pixel_delta_v);

            // ray dir is not a unit vector as this makes for a slightly faster code.
            let ray_direction = pixel_center - camera_center;
            let r = Ray::new(camera_center, ray_direction);

            let pixel_color = ray_color(&r);
            write_color(&mut std::io::stdout(), pixel_color);
        }
    }
    eprintln!("Done");
}
