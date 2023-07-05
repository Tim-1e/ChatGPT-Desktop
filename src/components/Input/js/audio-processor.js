class AudioProcessor extends AudioWorkletProcessor {
    constructor() {
      super();
    }
    process(inputs, outputs, parameters) {
      const input = inputs[0];
      const output = outputs[0];
  
      for (let channel = 0; channel < input.length; ++channel) {
        const inputChannel = input[channel];
        const outputChannel = output[channel];
        for (let i = 0; i < inputChannel.length; ++i) {
          outputChannel[i] = inputChannel[i];
        }
      }
      
      if (inputs[0] && inputs[0][0]) {
        this.port.postMessage(inputs[0][0]);
      }

      return true;
    }
  }
  
  registerProcessor('audio-processor', AudioProcessor);
  