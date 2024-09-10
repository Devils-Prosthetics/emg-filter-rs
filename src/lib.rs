const LPF_NUMERATOR_COEFFICIENT: [[f32; 3]; 2] =
    [[0.3913, 0.7827, 0.3913], [0.1311, 0.2622, 0.1311]];

const LPF_DENOMINATOR_COEFFICIENT: [[f32; 3]; 2] =
    [[1.0000, 0.3695, 0.1958], [1.0000, -0.7478, 0.2722]];

const HPF_NUMERATOR_COEFFICIENT: [[f32; 3]; 2] =
    [[0.8371, -1.6742, 0.8371], [0.9150, -1.8299, 0.9150]];

const HPF_DENOMINATOR_COEFFICIENT: [[f32; 3]; 2] =
    [[1.0000, -1.6475, 0.7009], [1.0000, -1.8227, 0.8372]];

const AHF_NUMERATOR_COEF_50HZ: [[f32; 6]; 2] = [
    [0.9522, -1.5407, 0.9522, 0.8158, -0.8045, 0.0855],
    [0.5869, -1.1146, 0.5869, 1.0499, -2.0000, 1.0499],
];
const AHF_DENOMINATOR_COEF_50HZ: [[f32; 6]; 2] = [
    [1.0000, -1.5395, 0.9056, 1.0000, -1.1187, 0.3129],
    [1.0000, -1.8844, 0.9893, 1.0000, -1.8991, 0.9892],
];
const AHF_OUTPUT_GAIN_COEF_50HZ: [f32; 2] = [1.3422, 1.4399];
// coef[sampleFreqInd][order] for 60Hz
const AHF_NUMERATOR_COEF_60HZ: [[f32; 6]; 2] = [
    [0.9528, -1.3891, 0.9528, 0.8272, -0.7225, 0.0264],
    [0.5824, -1.0810, 0.5824, 1.0736, -2.0000, 1.0736],
];
const AHF_DENOMINATOR_COEF_60HZ: [[f32; 6]; 2] = [
    [1.0000, -1.3880, 0.9066, 1.0000, -0.9739, 0.2371],
    [1.0000, -1.8407, 0.9894, 1.0000, -1.8584, 0.9891],
];
const AHF_OUTPUT_GAIN_COEF_60HZ: [f32; 2] = [1.3430, 1.4206];

#[derive(PartialEq, Clone, Copy)]
pub enum NotchFrequency {
    Hz50 = 50,
    Hz60 = 60,
}

#[derive(PartialEq, Clone, Copy)]
pub enum SampleFrequency {
    Hz500 = 500,
    Hz1000 = 1000,
}

enum FilterType {
    LowPass = 0,
    HighPass,
}

struct Filter2nd {
    states: [f32; 2],
    num: [f32; 3],
    den: [f32; 3],
}

impl Filter2nd {
    fn init(filter_type: FilterType, sample_frequency: SampleFrequency) -> Self {
        let states = [0f32; 2];
        let mut num = [0f32; 3];
        let mut den = [0f32; 3];

        match filter_type {
            FilterType::LowPass => match sample_frequency {
                SampleFrequency::Hz500 => {
                    for i in 0..3 {
                        num[i] = LPF_NUMERATOR_COEFFICIENT[0][i];
                        den[i] = LPF_DENOMINATOR_COEFFICIENT[0][i];
                    }
                }
                SampleFrequency::Hz1000 => {
                    for i in 0..3 {
                        num[i] = LPF_NUMERATOR_COEFFICIENT[1][i];
                        den[i] = LPF_DENOMINATOR_COEFFICIENT[1][i];
                    }
                }
            },
            FilterType::HighPass => match sample_frequency {
                SampleFrequency::Hz500 => {
                    for i in 0..3 {
                        num[i] = HPF_NUMERATOR_COEFFICIENT[0][i];
                        den[i] = HPF_DENOMINATOR_COEFFICIENT[0][i];
                    }
                }
                SampleFrequency::Hz1000 => {
                    for i in 0..3 {
                        num[i] = HPF_NUMERATOR_COEFFICIENT[1][i];
                        den[i] = HPF_DENOMINATOR_COEFFICIENT[1][i];
                    }
                }
            },
        }

        Self { states, num, den }
    }

    fn update(&mut self, input: f32) -> f32 {
        let tmp =
            (input - self.den[1] * self.states[0] - self.den[2] * self.states[1]) / self.den[0];
        let output =
            self.num[0] * tmp + self.num[1] * self.states[0] + self.num[2] * self.states[1];
        // save last states
        self.states[1] = self.states[0];
        self.states[0] = tmp;
        return output;
    }
}

struct Filter4th {
    states: [f32; 4],
    num: [f32; 6],
    den: [f32; 6],
    gain: f32,
}

impl Filter4th {
    fn init(sample_frequency: SampleFrequency, hum_frequency: NotchFrequency) -> Self {
        let states = [0f32; 4];
        let mut num = [0f32; 6];
        let mut den = [0f32; 6];
        let mut gain: f32 = 0.0;

        match hum_frequency {
            NotchFrequency::Hz50 => match sample_frequency {
                SampleFrequency::Hz500 => {
                    for i in 0..6 {
                        num[i] = AHF_NUMERATOR_COEF_50HZ[0][i];
                        den[i] = AHF_DENOMINATOR_COEF_50HZ[0][i];
                    }
                    gain = AHF_OUTPUT_GAIN_COEF_50HZ[0];
                }
                SampleFrequency::Hz1000 => {
                    for i in 0..6 {
                        num[i] = AHF_NUMERATOR_COEF_50HZ[1][i];
                        den[i] = AHF_DENOMINATOR_COEF_50HZ[1][i];
                    }
                    gain = AHF_OUTPUT_GAIN_COEF_50HZ[1];
                }
            },
            NotchFrequency::Hz60 => match sample_frequency {
                SampleFrequency::Hz500 => {
                    for i in 0..6 {
                        num[i] = AHF_NUMERATOR_COEF_60HZ[0][i];
                        den[i] = AHF_DENOMINATOR_COEF_60HZ[0][i];
                    }
                    gain = AHF_OUTPUT_GAIN_COEF_60HZ[0];
                }
                SampleFrequency::Hz1000 => {
                    for i in 0..6 {
                        num[i] = AHF_NUMERATOR_COEF_60HZ[1][i];
                        den[i] = AHF_DENOMINATOR_COEF_60HZ[1][i];
                    }
                    gain = AHF_OUTPUT_GAIN_COEF_60HZ[1];
                }
            },
        }

        Self {
            states,
            num,
            den,
            gain,
        }
    }

    fn update(&mut self, input: f32) -> f32 {
        let output: f32;
        let stage_in: f32;
        let mut stage_out: f32;

        stage_out = self.num[0] * input + self.states[0];
        self.states[0] = (self.num[1] * input + self.states[1]) - self.den[1] * stage_out;
        self.states[1] = self.num[2] * input - self.den[2] * stage_out;
        stage_in = stage_out;
        stage_out = self.num[3] * stage_out + self.states[2];
        self.states[2] = (self.num[4] * stage_in + self.states[3]) - self.den[4] * stage_out;
        self.states[3] = self.num[5] * stage_in - self.den[5] * stage_out;

        output = self.gain * stage_out;
        output
    }
}

pub struct EMGFilters {
    bypass_enabled: bool,
    notch_filter_enabled: bool,
    lowpass_filter_enabled: bool,
    highpass_filter_enabled: bool,
    lpf: Filter2nd,
    hpf: Filter2nd,
    ahf: Filter4th,
}

impl EMGFilters {
    pub fn init(
        sample_frequency: SampleFrequency,
        notch_frequency: NotchFrequency,
        enable_notch_filter: bool,
        enable_lowpass_filter: bool,
        enable_highpass_filter: bool,
    ) -> Self {
        let mut bypass_enabled = true;

        if ((sample_frequency == SampleFrequency::Hz500)
            || (sample_frequency == SampleFrequency::Hz1000))
            && ((notch_frequency == NotchFrequency::Hz50)
                || (notch_frequency == NotchFrequency::Hz60))
        {
            bypass_enabled = false;
        }
        Self {
            bypass_enabled,
            notch_filter_enabled: enable_notch_filter,
            lowpass_filter_enabled: enable_lowpass_filter,
            highpass_filter_enabled: enable_highpass_filter,
            lpf: Filter2nd::init(FilterType::LowPass, sample_frequency),
            hpf: Filter2nd::init(FilterType::HighPass, sample_frequency),
            ahf: Filter4th::init(sample_frequency, notch_frequency),
        }
    }

    pub fn update(&mut self, input_value: f32) -> f32 {
        let mut output: f32 = 0.0;
        if self.bypass_enabled {
            output = input_value;
        }

        // first notch filter
        if self.notch_filter_enabled {
            // output = NTF.update(inputValue);
            output = self.ahf.update(input_value);
        } else {
            // notch filter bypass
            output = input_value;
        }

        // second low pass filter
        if self.lowpass_filter_enabled {
            output = self.lpf.update(output);
        }

        // third high pass filter
        if self.highpass_filter_enabled {
            output = self.hpf.update(output);
        }
        output
    }
}
