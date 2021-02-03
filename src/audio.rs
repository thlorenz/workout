/*
var audioCtx = new(window.AudioContext || window.webkitAudioContext)();

function playNote(frequency, duration) {
  // create Oscillator node
  var oscillator = audioCtx.createOscillator();

  oscillator.type = 'square';
  oscillator.frequency.value = frequency; // value in hertz
  oscillator.connect(audioCtx.destination);
  oscillator.start();

  setTimeout(
    function() {
      oscillator.stop();
      playMelody();
    }, duration);
}
*/

use wasm_bindgen::JsValue;
use web_sys::{AudioContext, AudioContextState, GainNode, OscillatorNode, OscillatorType};

pub struct Audio {
    ctx: AudioContext,
    node: OscillatorNode,
    gain: GainNode,
}

impl Drop for Audio {
    fn drop(&mut self) {
        let _ = self.ctx.close();
    }
}

impl Audio {
    pub fn new() -> Result<Audio, JsValue> {
        let ctx = web_sys::AudioContext::new()?;

        let gain = ctx.create_gain()?;
        gain.gain().set_value(0.0);

        let node = ctx.create_oscillator()?;
        node.set_type(OscillatorType::Square);

        node.connect_with_audio_node(&gain)?;
        gain.connect_with_audio_node(&ctx.destination())?;

        node.start()?;

        Ok(Audio { ctx, node, gain })
    }

    pub fn play(&self, freq: f32) {
        if self.ctx.state() == AudioContextState::Suspended {
            let _ = self.ctx.resume();
        }
        self.node.frequency().set_value(freq);
        self.gain.gain().set_value(1.0);
    }

    pub fn stop(&self) {
        // Cannot stop and restart node, so we use gain to turn it on/off
        self.gain.gain().set_value(0.0);
    }
}
