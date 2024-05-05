#define_import_path bevaders::common

const PI: f32  = 3.14159265359;
const CENTER: vec2<f32> = vec2f(0.5, 0.5);

fn palette(t: f32) -> vec3<f32> {
    let a = vec3<f32>(0.5, 0.5, 0.5);
    let b = vec3<f32>(0.5, 0.5, 0.5);
    let c = vec3<f32>(1.0, 1.0, 1.0);
    let d = vec3<f32>(0.263, 0.416, 0.557);
    return a + b*cos(6.28318*(c*t+d));
}

fn hash(value: u32) -> u32 {
    var state = value;
    state = state ^ 2747636419u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    state = state ^ state >> 16u;
    state = state * 2654435769u;
    return state;
}

fn random_float(value: u32) -> f32 {
    return f32(hash(value)) / 4294967295.0;
}

fn zoom(uv: vec2f, factor: f32) -> vec2f {
    var zoomed_uv = uv;
    zoomed_uv -= CENTER;
    zoomed_uv /= factor;
    zoomed_uv += CENTER;
    return zoomed_uv;
}

/// Rotate clockwise by `theta`
fn rotate(uv: vec2f, theta: f32) -> vec2<f32> {
    let c = cos(theta);
    let s = sin(theta);
    let rotation_matrix = mat2x2<f32>(c, s, -s, c);
    return uv * rotation_matrix;
}


// fn gradient_noise(position: vec2<f32>) {
//     // Gradient Noise (http://en.wikipedia.org/wiki/Gradient_noise)
//     // Implementation from  Inigo Quilez
    
//     let i: vec2<i32> = floor(position);
//     let f: vec2<f32> = fract(position);

//     let u = f * f * (3.0 - 2.0 * f); // feel free to replace by a quintic smoothstep instead

//     return mix( 
//         mix( 
//             dot(grad(i + ivec2(0,0)), f - vec2(0.0,0.0)), 
//             dot(grad(i + ivec2(1,0)), f - vec2(1.0,0.0)), 
//             u.x
//         ),
//         mix( 
//             dot(grad(i + ivec2(0,1)), f - vec2(0.0,1.0)), 
//             dot(grad(i + ivec2(1,1)), f - vec2(1.0,1.0)), 
//             u.x
//         ), 
//         u.y
//     );
// }
