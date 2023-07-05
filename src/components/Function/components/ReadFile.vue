<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/tauri';

const disabled = ref(false); // define disabled as a reactive variable

const openDocument = async () => {
  disabled.value = true; // disable the button while the document is being opened
  try {
    const documentContent = await invoke('open_document');
    // 成功打开文档后，显示成功消息
    Message.success('成功复制内容到剪贴板');
  } 
  catch (error) {
    // 如果在打开文档的过程中出现错误，显示错误消息
      Message.error(`啊哦,文件打开失败了:${error}`);
  }
  finally {
    disabled.value = false; // re-enable the button after the document is opened
  }
};

</script>

<template>
    <div>
    <a-button type="text" :disabled="disabled" @click="openDocument">
        <template #icon>
          <icon-file />
        </template>
    </a-button>
    </div>
</template>
