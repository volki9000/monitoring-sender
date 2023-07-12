// Monitoring sender : Sends stereo channel to different outputs at different levels
// Copyright (C) 2023 Volkmar Kobelt
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.

use nih_plug::prelude::*;

pub struct MonitoringSender {
    params: std::sync::Arc<MonitoringSenderParams>,
    buffer_config: BufferConfig,
}

#[derive(Params)]
struct MonitoringSenderParams {
    #[id = "FOH"]
    main_gain: FloatParam,
    #[id = "Axel"]
    ax_gain: FloatParam,
    #[id = "Sebi"]
    sb_gain: FloatParam,
    #[id = "Volki"]
    vk_gain: FloatParam
}

impl Default for MonitoringSenderParams {
    fn default() -> Self {
        Self {
            main_gain: FloatParam::new(
                "FOH",
                util::db_to_gain(0.00),
                FloatRange::Skewed {
                    min: util::db_to_gain(-144.0),
                    max: util::db_to_gain(12.0),
                    factor: FloatRange::gain_skew_factor(-24.0, 12.0),
                },
            )
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            ax_gain: FloatParam::new(
                "Axel",
                util::db_to_gain(0.00),
                FloatRange::Skewed {
                    min: util::db_to_gain(-144.0),
                    max: util::db_to_gain(12.0),
                    factor: FloatRange::gain_skew_factor(-24.0, 12.0),
                },
            )
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            sb_gain: FloatParam::new(
                "Sebi",
                util::db_to_gain(0.00),
                FloatRange::Skewed {
                    min: util::db_to_gain(-144.0),
                    max: util::db_to_gain(12.0),
                    factor: FloatRange::gain_skew_factor(-24.0, 12.0),
                },
            )
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
            vk_gain: FloatParam::new(
                "Volki",
                util::db_to_gain(0.00),
                FloatRange::Skewed {
                    min: util::db_to_gain(-144.0),
                    max: util::db_to_gain(12.0),
                    factor: FloatRange::gain_skew_factor(-24.0, 12.0),
                },
            )
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Default for MonitoringSender {
    fn default() -> Self {
        Self {
            params: std::sync::Arc::new(MonitoringSenderParams::default()),
            buffer_config: BufferConfig {
                sample_rate: 1.0,
                min_buffer_size: None,
                max_buffer_size: 0,
                process_mode: ProcessMode::Realtime,
            }
        }
    }
}

impl Plugin for MonitoringSender {
    const NAME: &'static str = "Monitoring Sender";
    const VENDOR: &'static str = "volki9000";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "https://github.com/volki9000";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[new_nonzero_u32(2); 4],

        names: PortNames {
            layout: Some("4 stereo send channels"),
            main_input: None,
            // We won't output any sound here
            main_output: Some("Same as input"),
            aux_inputs: &[""],
            aux_outputs: &["FOH", "Axel", "Sebi", "Volki"],
        },
    }];

type BackgroundTask = ();
type SysExMessage = ();

fn params(&self) -> std::sync::Arc<dyn Params> {
    self.params.clone()
}

fn initialize(
    &mut self,
    _layout: &AudioIOLayout,
    buffer_config: &BufferConfig,
    _context: &mut impl InitContext<Self>
) -> bool {
    self.buffer_config = *buffer_config;
    true
}

fn reset(&mut self) {
}

fn process(
    &mut self,
    buffer: &mut Buffer,
    aux: &mut AuxiliaryBuffers,
    _context: &mut impl ProcessContext<Self>,
) -> ProcessStatus {
    // Don't do anything when bouncing
    if self.buffer_config.process_mode == ProcessMode::Offline {
        return ProcessStatus::Normal;
    }
    let gains = [self.params.main_gain.value(),
                self.params.ax_gain.value(),
                self.params.sb_gain.value(),
                self.params.vk_gain.value()];
    for send_index in 0..3
    {
        let mut send_1_buffer = aux.outputs[send_index].iter_samples().into_iter();

        for (in1, out1) in buffer.iter_samples().into_iter()
        .map(
            |x| { (x, send_1_buffer.next()) }
            )
        {
            for (ch_in, ch_out) in in1.into_iter().zip(out1.unwrap())
            {
                *ch_out = *ch_in * gains[send_index];
            }
        }
    }

    ProcessStatus::Normal
}
}

impl MonitoringSender {

}

impl ClapPlugin for MonitoringSender {
    const CLAP_ID: &'static str = "volki9000.monitoring-sender";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Distribute audio over multiple channels at different gains");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;
    const CLAP_FEATURES: &'static [ClapFeature] = &[
        ClapFeature::AudioEffect,
        ClapFeature::Stereo,
        ClapFeature::Mono,
        ClapFeature::Utility,
    ];
}

impl Vst3Plugin for MonitoringSender {
    const VST3_CLASS_ID: [u8; 16] = *b"MntgSendPlugV9k.";
    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
        Vst3SubCategory::Fx,
        Vst3SubCategory::Tools];
}

nih_export_clap!(MonitoringSender);
nih_export_vst3!(MonitoringSender);
