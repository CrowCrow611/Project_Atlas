use crate::pipeline::Pipeline;
use raylib::prelude::*;
use raylib::ffi;
use std::f32::consts::TAU;

macro_rules! shaded {
    ($shader:expr, $block:block) => {{
        unsafe { ffi::BeginShaderMode(*$shader.as_ref()); }
        $block
        unsafe { ffi::EndShaderMode(); }
    }};
}

pub fn draw_black_hole(
    d:     &mut RaylibMode3D<RaylibDrawHandle>,
    pipe:  &mut Pipeline,
    pos:   Vector3,
    r:     f32,
    angle: f32,
    cam:   Vector3,
) {
    let bands: &[(f32, u8, u8, u8, u8)] = &[
        (r*1.55, 255, 245,  95, 230),
        (r*2.40, 255, 210,  70, 195),
        (r*3.60, 255, 160,  30, 155),
        (r*5.20, 220, 100,  12, 110),
        (r*7.20, 160,  55,   5,  60),
        (r*9.50, 100,  30,   2,  25),
    ];
    let segs = 128i32;
    for (bi, (br, cr, cg, cb, a)) in bands.iter().enumerate() {
        let rot  = angle * (1.0 + bi as f32 * 0.14);
        let warp = bi as f32 * 0.045 + 0.015;
        for s in 0..segs {
            let a1 = (s   as f32 / segs as f32) * TAU + rot;
            let a2 = ((s+1) as f32 / segs as f32) * TAU + rot;
            d.draw_line_3D(
                Vector3::new(pos.x + a1.cos()*br, pos.y + a1.sin()*warp*br, pos.z + a1.sin()*br),
                Vector3::new(pos.x + a2.cos()*br, pos.y + a2.sin()*warp*br, pos.z + a2.sin()*br),
                Color::new(*cr, *cg, *cb, *a),
            );
            d.draw_line_3D(
                Vector3::new(pos.x + a1.cos()*br*0.97, pos.y + a1.sin()*warp*br + 0.5, pos.z + a1.sin()*br*0.97),
                Vector3::new(pos.x + a2.cos()*br*0.97, pos.y + a2.sin()*warp*br + 0.5, pos.z + a2.sin()*br*0.97),
                Color::new(255, 255, 200, ((*a as u32) * 55 / 215) as u8),
            );
        }
    }

    let pr   = r * 1.5;
    let segs = 256i32;
    for s in 0..segs {
        let a1 = (s   as f32 / segs as f32) * TAU;
        let a2 = ((s+1) as f32 / segs as f32) * TAU;
        for (off, alpha) in [(0.0_f32,255u8),(0.5,160),(-0.5,160),(1.0,80),(-1.0,80)] {
            d.draw_line_3D(
                Vector3::new(pos.x + a1.cos()*pr, pos.y + off, pos.z + a1.sin()*pr),
                Vector3::new(pos.x + a2.cos()*pr, pos.y + off, pos.z + a2.sin()*pr),
                Color::new(255, 200, 80, alpha),
            );
        }
    }

    for si in 0..8i32 {
        let ang   = (si as f32 / 8.0) * TAU + angle * 0.03;
        let outer = r * (6.0 + (si % 2) as f32 * 3.5);
        d.draw_line_3D(
            Vector3::new(pos.x + ang.cos()*r*2.2, pos.y, pos.z + ang.sin()*r*2.2),
            Vector3::new(pos.x + ang.cos()*outer, pos.y, pos.z + ang.sin()*outer),
            Color::new(255, 180, 40, if si % 2 == 0 { 50 } else { 20 }),
        );
    }

    d.draw_sphere(pos, r, Color::new(10, 4, 20, 255));
    pipe.blackhole.set_shader_value(pipe.bh_locs.camera_pos, [cam.x, cam.y, cam.z]);
    shaded!(&pipe.blackhole, {
        d.draw_sphere(pos, r, Color::new(10, 4, 20, 255));
    });
}

pub fn draw_water_giant(
    d:      &mut RaylibMode3D<RaylibDrawHandle>,
    pipe:   &mut Pipeline,
    pos:    Vector3,
    r:      f32,
    rot:    f32,
    bh_pos: Vector3,
    cam:    Vector3,
    time:   f32,
) {
    // fallback sphere so it's visible even if shader fails
    d.draw_sphere(pos, r, Color::new(6, 22, 108, 255));

    pipe.watergiant.set_shader_value(pipe.wg_locs.light_pos,  [bh_pos.x, bh_pos.y, bh_pos.z]);
    pipe.watergiant.set_shader_value(pipe.wg_locs.camera_pos, [cam.x, cam.y, cam.z]);
    pipe.watergiant.set_shader_value(pipe.wg_locs.time,       time);
    shaded!(&pipe.watergiant, {
        d.draw_sphere(pos, r, Color::WHITE);
    });

    // ice rings
    let tilt  = 0.10_f32;
    let rsegs = 148i32;
    for (rr, a) in [
        (r*1.62,75u8),(r*1.80,60),(r*2.02,46),(r*2.28,32),(r*2.60,18),(r*3.00,9)
    ] {
        for s in 0..rsegs {
            let a1 = (s   as f32 / rsegs as f32) * TAU;
            let a2 = ((s+1) as f32 / rsegs as f32) * TAU;
            d.draw_line_3D(
                Vector3::new(pos.x + a1.cos()*rr, pos.y + a1.sin()*rr*tilt, pos.z + a1.sin()*rr),
                Vector3::new(pos.x + a2.cos()*rr, pos.y + a2.sin()*rr*tilt, pos.z + a2.sin()*rr),
                Color::new(190, 228, 255, a),
            );
        }
    }

    // polar auroras
    let asegs = 56i32;
    for pole in [-1.0_f32, 1.0] {
        let py = pos.y + pole * r * 1.04;
        let ar = r * 0.24;
        for s in 0..asegs {
            let a1 = (s   as f32 / asegs as f32) * TAU + rot * 0.28;
            let a2 = ((s+1) as f32 / asegs as f32) * TAU + rot * 0.28;
            d.draw_line_3D(
                Vector3::new(pos.x + a1.cos()*ar, py, pos.z + a1.sin()*ar),
                Vector3::new(pos.x + a2.cos()*ar, py, pos.z + a2.sin()*ar),
                Color::new(90, 215, 255, 95),
            );
        }
        d.draw_sphere(Vector3::new(pos.x, py, pos.z), r*0.18, Color::new(55,175,255,75));
    }
}

pub fn draw_planet(
    d: &mut RaylibMode3D<RaylibDrawHandle>,
    pipe: &mut Pipeline,
    pos: Vector3,
    r: f32,
    col: Color,
    wg_pos: Vector3,
    bh_pos: Vector3,
    cam: Vector3,
) {
    // fallback sphere
    d.draw_sphere(pos, r, col);

    let base = [col.r as f32/255.0, col.g as f32/255.0, col.b as f32/255.0];
    pipe.planet.set_shader_value(pipe.pl_locs.light_pos,  [bh_pos.x, bh_pos.y, bh_pos.z]);
    pipe.planet.set_shader_value(pipe.pl_locs.camera_pos, [cam.x, cam.y, cam.z]);
    pipe.planet.set_shader_value(pipe.pl_locs.base_color, base);
    pipe.planet.set_shader_value(pipe.pl_locs.wg_pos,     [wg_pos.x, wg_pos.y, wg_pos.z]);
    shaded!(&pipe.planet, {
        d.draw_sphere(pos, r, Color::WHITE);
    });

    // atmospheric shells
    let dx   = wg_pos.x - pos.x;
    let dz   = wg_pos.z - pos.z;
    let wdist = (dx*dx + dz*dz).sqrt();
    let fall  = (1.0 - (wdist / 160.0).min(1.0)).powf(0.55).max(0.07);

    for (ar, bf, aa) in [(r*1.14,0.52_f32,42u8),(r*1.28,0.32,23),(r*1.50,0.16,11)] {
        d.draw_sphere_wires(pos, ar, 8, 8, Color::new(
            (col.r as f32 * fall * bf + 12.0).min(255.0) as u8,
            (col.g as f32 * fall * bf + 14.0).min(255.0) as u8,
            (col.b as f32 * fall * bf + 28.0).min(255.0) as u8,
            aa,
        ));
    }
}