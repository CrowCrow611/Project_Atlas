use raylib::prelude::*;

pub struct PlanetLocs {
    pub light_pos:  i32,
    pub camera_pos: i32,
    pub base_color: i32,
    pub wg_pos:     i32,
}

pub struct BhLocs {
    pub camera_pos: i32,
}

pub struct WgLocs {
    pub light_pos:  i32,
    pub camera_pos: i32,
    pub time:       i32,
}

pub struct Pipeline {
    pub planet:     Shader,
    pub pl_locs:    PlanetLocs,
    pub blackhole:  Shader,
    pub bh_locs:    BhLocs,
    pub watergiant: Shader,
    pub wg_locs:    WgLocs,
}

impl Pipeline {
    pub fn load(rl: &mut RaylibHandle, thread: &RaylibThread) -> Self {
        let planet = rl.load_shader(thread,
            None,
            Some("shaders/planet.frag"),
        );
        let pl_locs = PlanetLocs {
            light_pos:  planet.get_shader_location("lightPos"),
            camera_pos: planet.get_shader_location("cameraPos"),
            base_color: planet.get_shader_location("baseColor"),
            wg_pos:     planet.get_shader_location("wgPos"),
        };

        let blackhole = rl.load_shader(thread,
            None,
            Some("shaders/blackhole.frag"),
        );
        let bh_locs = BhLocs {
            camera_pos: blackhole.get_shader_location("cameraPos"),
        };

        let watergiant = rl.load_shader(thread,
            None,
            Some("shaders/watergiant.frag"),
        );
        let wg_locs = WgLocs {
            light_pos:  watergiant.get_shader_location("lightPos"),
            camera_pos: watergiant.get_shader_location("cameraPos"),
            time:       watergiant.get_shader_location("time"),
        };

        Pipeline { planet, pl_locs, blackhole, bh_locs, watergiant, wg_locs }
    }
}