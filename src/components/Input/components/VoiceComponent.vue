<script setup lang="ts">
  import { ref, onMounted} from 'vue';
  import Voice from '../js/voice.js';
  
  const isVoiceActive=ref(false)

  const voiceTxt = ref('');
  const times = ref(null);
  const voice = ref(null);
  
  const emit = defineEmits(['update-voice-txt']);

  const startVoiceRecognition = () => {
    voice.value.start();
  };
  
  const stopVoiceRecognition = () => {
    voice.value.stop();
  };
  
  onMounted(() => {
    voice.value = new Voice({
      appId: 'ddbb00c0',
      apiSecret: 'MzhjOTExMWE1ZjE0MmRiZWUzNjU3ZmZi',
      apiKey: '1754b26d2af25b6c4b0fa239d58f0304',
      onWillStatusChange: (oldStatus: any, newStatus: any) => {
        //do nothing at now
      },
      onTextChange: (text: string) => {
        //console.log("we change!");
        voiceTxt.value = text;
        emit('update-voice-txt', text);
        if (text) {
          clearTimeout(times.value);
          times.value = setTimeout(() => {
            voice.value.stop();
            isVoiceActive.value = false;
          }, 3000);
        }
      }
    });
  });

const changeVoiceMode=() => {
  isVoiceActive.value=!isVoiceActive.value
  if(isVoiceActive.value)
    startVoiceRecognition()
  else
    stopVoiceRecognition()
}

</script>

  <template>
    <icon-voice 
    :class="{ 'voice-active': isVoiceActive }"
    class="flex-col" 
    size="40"
    @click="changeVoiceMode"
    />
  </template>

  <style scoped lang="scss">
  .voice-active {
    color: rgb(46, 108, 223); /* 你可以根据需求设置所需的激活颜色 */
  }
  </style>
  