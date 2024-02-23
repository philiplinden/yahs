use std::{fs, path::PathBuf};
fn init_log_file(outpath: PathBuf) -> csv::Writer<fs::File> {
    let mut writer = csv::Writer::from_path(outpath).unwrap();
    writer
        .write_record(&[
            "time_s",
            "altitude_m",
            "ascent_rate_m_s",
            "acceleration_m_s2",
            "atmo_temp_K",
            "atmo_pres_Pa",
            "balloon_pres_Pa",
            "balloon_radius_m",
            "balloon_stress_Pa",
            "balloon_strain_pct",
            "balloon_thickness_m",
            "drogue_parachute_area_m2",
            "main_parachute_area_m2",
        ])
        .unwrap();
    writer
}

pub fn log_to_file(sim_output: &SimOutput, writer: &mut csv::Writer<fs::File>) {
    writer
        .write_record(&[
            sim_output.time_s.to_string(),
            sim_output.altitude.to_string(),
            sim_output.ascent_rate.to_string(),
            sim_output.acceleration.to_string(),
            sim_output.atmo_temp.to_string(),
            sim_output.atmo_pres.to_string(),
            sim_output.balloon_pres.to_string(),
            sim_output.balloon_radius.to_string(),
            sim_output.balloon_stress.to_string(),
            sim_output.balloon_strain.to_string(),
            sim_output.balloon_thickness.to_string(),
            sim_output.drogue_parachute_area.to_string(),
            sim_output.main_parachute_area.to_string(),
        ])
        .unwrap();
    writer.flush().unwrap();
}

#[derive(Default, Copy, Clone)]
pub struct SimOutput {
    pub time_s: f32,
    pub altitude: f32,
    pub ascent_rate: f32,
    pub acceleration: f32,
    pub atmo_temp: f32,
    pub atmo_pres: f32,
    pub balloon_pres: f32,
    pub balloon_radius: f32,
    pub balloon_stress: f32,
    pub balloon_strain: f32,
    pub balloon_thickness: f32,
    pub drogue_parachute_area: f32,
    pub main_parachute_area: f32,
}

pub struct SimCommands {
    //TODO: add ability to inject commands to logic controllers
}
