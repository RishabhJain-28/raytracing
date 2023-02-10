# Ray Tracing in one weekend

This is an implementation of [Peter Shirley's Ray Tracing in One Weekend](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust.

## Preview of the final image converted to PNG

![final](https://user-images.githubusercontent.com/56471206/216060466-9b381429-37e0-4cd0-b98c-2aec125559cf.png)

## To recreate the output image run:

`$ cargo run > img.ppm`

### Scenes:

The code contains few prebuilt scenes.
You can render them by calling the appropriate function in main.

## Extra improvemnts added beyond the book:

- Added rayon for parallel execution

Todo : add jpeg write
add default skybox in case of no bg color
make rotateY generic or sdd rotateX,Z
