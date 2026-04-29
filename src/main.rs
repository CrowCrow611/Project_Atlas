mod math;
mod sim;
mod render;
mod pipeline;

use raylib::prelude::*;
use sim::Sim;

struct Star {
    pos:  Vector3,
    size: f32,
    r: u8,
    g: u8,
    b: u8,
    lum: u8,
    flicker_speed: f32,
    flicker_phase: f32,
}

fn gen_stars(n: usize) -> Vec<Star> {
    let mut v = Vec::with_capacity(n);
    let mut s: u64 = 0xCAFEBABEDEADBEEF;
    let step = |x: u64| x.wrapping_mul(6364136223846793005).wrapping_add(1442695040888963407);
    let frac = |x: u64| (x >> 33) as f32 / u32::MAX as f32;

    for _ in 0..n {
        // spherical distribution — stars in a shell around the origin
        s = step(s); let theta = frac(s) * std::f32::consts::TAU;
        s = step(s); let phi   = (frac(s) * 2.0 - 1.0).acos();
        s = step(s); let rad   = 600.0 + frac(s) * 300.0; // shell 600..900 units out
        let x = rad * phi.sin() * theta.cos();
        let y = rad * phi.cos();
        let z = rad * phi.sin() * theta.sin();

        s = step(s); let t   = frac(s);
        s = step(s); let sz  = 0.8 + frac(s) * 3.0;
        s = step(s); let lum = (120. + frac(s) * 135.) as u8;
        s = step(s); let fs  = 0.4 + frac(s) * 3.0;
        s = step(s); let fp  = frac(s) * std::f32::consts::TAU;

        let (r, g, b): (u8, u8, u8) = if t < 0.10 {
            (255, 180, 100)
        } else if t < 0.20 {
            (255, 210, 140)
        } else if t < 0.45 {
            (255, 248, 220)
        } else if t < 0.65 {
            (240, 245, 255)
        } else if t < 0.82 {
            (200, 220, 255)
        } else if t < 0.93 {
            (160, 195, 255)
        } else {
            (130, 170, 255)
        };

        v.push(Star { pos: Vector3::new(x, y, z), size: sz, r, g, b, lum, flicker_speed: fs, flicker_phase: fp });
    }
    v
}


fn main() {
    #[cfg(target_os = "windows")]
    if let Ok(exe_path) = std::env::current_exe() {
        if let Some(exe_dir) = exe_path.parent() {
            let _ = std::env::set_current_dir(exe_dir);
        }
    }
    let (mut rl, thread) = raylib::init()
        .size(1280, 720)
        .title("Project Atlas")
        .build();
    rl.set_target_fps(60);

    let mut pipe = pipeline::Pipeline::load(&mut rl, &thread);

    let mut cam = Camera3D::perspective(
        Vector3::new(0., 205., 490.),
        Vector3::new(0.,   0.,   0.),
        Vector3::new(0.,   1.,   0.),
        45.,
    );

    let mut sim   = Sim::new();
    let stars = gen_stars(3000);
    let mut cp = Vector3::new(0., 205., 490.);
    let mut yaw   = -1.57_f32;
    let mut pitch = -0.30_f32;

    while !rl.window_should_close() {
        let dt = rl.get_frame_time();

        if rl.is_key_pressed(KeyboardKey::KEY_SPACE) {
            sim.paused = !sim.paused;
        }
        if rl.is_key_pressed(KeyboardKey::KEY_RIGHT_BRACKET) {
            sim.timescale = (sim.timescale * 2.).min(64.);
        }
        if rl.is_key_pressed(KeyboardKey::KEY_LEFT_BRACKET) {
            sim.timescale = (sim.timescale * 0.5).max(0.125);
        }

        if rl.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            let md = rl.get_mouse_delta();
            yaw   += md.x * 0.003;
            pitch  = (pitch - md.y * 0.003).clamp(-1.4, 1.4);
        }

        let fwd   = Vector3::new(pitch.cos()*yaw.cos(), pitch.sin(), pitch.cos()*yaw.sin());
        let right = Vector3::new(-yaw.sin(), 0., yaw.cos());
        let spd   = if rl.is_key_down(KeyboardKey::KEY_LEFT_SHIFT) { 420. } else { 105. };

        if rl.is_key_down(KeyboardKey::KEY_W) { cp.x += fwd.x*spd*dt; cp.y += fwd.y*spd*dt; cp.z += fwd.z*spd*dt; }
        if rl.is_key_down(KeyboardKey::KEY_S) { cp.x -= fwd.x*spd*dt; cp.y -= fwd.y*spd*dt; cp.z -= fwd.z*spd*dt; }
        if rl.is_key_down(KeyboardKey::KEY_A) { cp.x -= right.x*spd*dt; cp.z -= right.z*spd*dt; }
        if rl.is_key_down(KeyboardKey::KEY_D) { cp.x += right.x*spd*dt; cp.z += right.z*spd*dt; }
        if rl.is_key_down(KeyboardKey::KEY_Q) { cp.y -= spd * dt; }
        if rl.is_key_down(KeyboardKey::KEY_E) { cp.y += spd * dt; }

        let scroll = rl.get_mouse_wheel_move();
        cp.x += fwd.x * scroll * 25.;
        cp.y += fwd.y * scroll * 25.;
        cp.z += fwd.z * scroll * 25.;

        cam.position = cp;
        cam.target   = Vector3::new(cp.x + fwd.x, cp.y + fwd.y, cp.z + fwd.z);

        sim.tick(0.004);

        let bh_pos = sim.bodies[0].pos.to_rl();
        let bh_r = sim.bodies[0].radius as f32;
        let wg_pos = sim.bodies[1].pos.to_rl();
        let wg_r = sim.bodies[1].radius as f32;
        let disc_rot = sim.disc_rot;
        let wg_rot = sim.wg_rot;
        let sim_time = sim.time as f32;
        let body_count = sim.bodies.len();

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);

        {
            let mut d3 = d.begin_mode3D(cam);

            // stars with flicker
            for star in &stars {
                let flicker = (sim_time * star.flicker_speed + star.flicker_phase).sin() * 0.18 + 0.82;
                let lum = (star.lum as f32 * flicker).min(255.) as u8;
                let r   = (star.r   as f32 * flicker).min(255.) as u8;
                let g   = (star.g   as f32 * flicker).min(255.) as u8;
                let b   = (star.b   as f32 * flicker).min(255.) as u8;
                let sp = Vector3::new(star.pos.x + cp.x, star.pos.y + cp.y, star.pos.z + cp.z);
                d3.draw_cube(sp, star.size, star.size, star.size, Color::new(r, g, b, lum));

                // bright stars get a small cross flare
                if star.lum > 220 {
                    let arm = star.size * 3.5;
                    d3.draw_line_3D(
              Vector3::new(sp.x - arm, sp.y, sp.z),
                Vector3::new(sp.x + arm, sp.y, sp.z),
                        Color::new(star.r, star.g, star.b, 60),
                    );
                    d3.draw_line_3D(
            Vector3::new(sp.x, sp.y - arm, sp.z),
              Vector3::new(sp.x, sp.y + arm, sp.z),
                       Color::new(star.r, star.g, star.b, 60),
                    );
                }
            }

            render::draw_black_hole(&mut d3, &mut pipe, bh_pos, bh_r, disc_rot, cp);
            sim.trails[1].draw(&mut d3, Color::new(38, 108, 228, 255));
            render::draw_water_giant(&mut d3, &mut pipe, wg_pos, wg_r, wg_rot, bh_pos, cp, sim_time);

            for i in 2..body_count {
                let p = sim.bodies[i].pos.to_rl();
                let r = sim.bodies[i].radius as f32;
                let c = sim.bodies[i].color;
                sim.trails[i].draw(&mut d3, c);
                render::draw_planet(&mut d3, &mut pipe, p, r, c, wg_pos, bh_pos, cp);
            }
        }
    }
}