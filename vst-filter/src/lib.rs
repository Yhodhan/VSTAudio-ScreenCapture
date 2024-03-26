use nih_plug::prelude::*;
use std::sync::Arc;

// Representation of the filter object
struct DistortionFilter {
    params: Arc<DistortionFilterParams>,
    sample_rate: f32,
}

#[derive(Params)]
struct DistortionFilterParams {
    #[id = "output"]
    output_gain: FloatParam,
}

impl Default for DistortionFilter {
    fn default() -> Self {
        Self {
            params: Arc::new(DistortionFilterParams::default()),
            sample_rate: 1.0,
        }
    }
}

// The default creation sets the params available for the plugin.
impl Default for DistortionFilterParams {
    fn default() -> Self {
        Self {
            output_gain: FloatParam::new(
                "Distortion gain",
                util::db_to_gain(-24.0),
                FloatRange::Linear {
                    min: util::db_to_gain(-24.0),
                    max: util::db_to_gain(0.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(10.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for DistortionFilter {
    const NAME: &'static str = "Sound distortion";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const VENDOR: &'static str = "MMSC";
    const VERSION: &'static str = env!("CARGO_PKG_VERSION");
    const EMAIL: &'static str = "info@example.com";

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(2),
            main_output_channels: NonZeroU32::new(2),
            ..AudioIOLayout::const_default()
        },
        AudioIOLayout {
            main_input_channels: NonZeroU32::new(1),
            main_output_channels: NonZeroU32::new(1),
            ..AudioIOLayout::const_default()
        },
    ];

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.sample_rate = buffer_config.sample_rate;
        true
    }

    // The function responsable for altering the samples to produce the distortion.
    // The effect is achieved by increasing the gain of the samples.
    // In this case if increase by one but more can be used producing a more fuzzy sound.
    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for mut samples in buffer.iter_samples() {
            let output_gain = self.params.output_gain.smoothed.next();

            for sample in samples.iter_mut() {
                *sample = if *sample >= 0.0 { 1.0 } else { -1.0 } * output_gain;
            }
        }

        ProcessStatus::Normal
    }
}

impl Vst3Plugin for DistortionFilter {
    const VST3_CLASS_ID: [u8; 16] = *b"DistorsionFlRvdH";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Dynamics,
        Vst3SubCategory::Distortion,
        Vst3SubCategory::Custom("Distorwave"),
    ];
}

// The macro produces the VST file.
nih_export_vst3!(DistortionFilter);
