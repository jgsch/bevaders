#import bevy_sprite::mesh2d_view_bindings::globals 
#import bevy_pbr::forward_io::VertexOutput;
#import bevaders::common::{PI, zoom, rotate};

@group(2) @binding(1) var texture: texture_2d<f32>;
@group(2) @binding(2) var texture_sampler: sampler;

fn kaleidoscope(uv: vec2<f32>) -> vec2<f32> {
    let REPETITION_1 = 2.;
    let REPETITION_2 = 2.;

    var kal = uv;
    
    kal = rotate(kal - 0.5, 0.15 * globals.time) + 0.5;
    kal = fract(REPETITION_1 * kal);

    kal = rotate(abs(kal - 0.5), 0.25 * globals.time) + 0.5;
    kal = fract(REPETITION_2 * kal);

    kal = rotate(abs(kal - 0.5),  globals.time) + 0.5;
    kal = rotate(abs(kal - 0.5),  1.) + 0.5;
    // kal = zoom(kal, pow(distance(uv, rotate(vec2f(0.5, 0.5), globals.time * 2.) + 0.5), 2.));

    return kal;
}

@fragment
fn fragment(in: VertexOutput) -> @location(0) vec4<f32> {
    return textureSample(texture, texture_sampler, kaleidoscope(in.uv)); 
}    
    
