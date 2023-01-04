use mixr::AudioFormat;

pub struct Sample {
    pub data: Vec<u8>,
    pub format: AudioFormat,
    pub multiplier: f64,

    pub looping: bool,
    pub loop_start: i32,
    pub loop_end: i32,

    pub global_volume: u8
}

impl Sample {
    pub fn new(data: &[u8], format: AudioFormat, looping: bool, loop_start: i32, loop_end: i32, global_volume: u8) -> Self {
        let multiplier = format.sample_rate as f64 / (crate::track_player::calculate_speed(crate::PianoKey::C, 5, 1.0) * format.sample_rate as f64);

        let mut d_vec = data.to_vec();
        fix_sample(&mut d_vec, &format);

        Self { 
            data: d_vec, 
            format, 
            multiplier,
            looping,
            loop_start,
            loop_end,

            global_volume
        }
    }
}

fn fix_sample(data: &mut Vec<u8>, format: &AudioFormat) {
    if format.bits_per_sample == 8 {
        for i in 0..data.len() {
            data[i] = (data[i] as i32 - 128) as u8;
        }
    }

    if format.channels == 2 {
        let old_data = data.clone();
        data.clear();

        // i did want to do this the nice math way but it didnt work so screw it i'll just use if statements
        let mut left_i = 0;
        let mut right_i = old_data.len() / 2;
        let mut is_right = false;

        println!("{}, {right_i}", old_data.len());

        let alignment = format.bits_per_sample / 8;

        while data.len() != old_data.len() {
            for _ in 0..alignment {
                if is_right {
                    data.push(old_data[right_i]);
                    right_i += 1;
                } else {
                    data.push(old_data[left_i]);
                    left_i += 1;
                }
            }

            is_right = !is_right;
        }
    }
}