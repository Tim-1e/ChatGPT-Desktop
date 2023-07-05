<script setup lang="ts">
import { getName } from '@tauri-apps/api/app'
import multiAvatar from '@multiavatar/multiavatar'
import { openDB } from 'idb'

const props = defineProps<{
    value?: string;
    enableReplace?: boolean;
  }>();

const appName = ref('')

const defaultAvatar = computed(() => multiAvatar(props.value || appName.value))

const avatar = ref('')

async function getAvatarFromDB(Name: string) {
  const db = await openDB('AvatarDB', 1, {
    upgrade(db) {
      db.createObjectStore('avatars')
    },
  })

  return db.get('avatars', Name)
}

async function saveAvatarToDB(appName: string, avatarDataUrl: string) {
  const db = await openDB('AvatarDB', 1)

  return db.put('avatars', avatarDataUrl, appName)
}

async function UpdateImg() {
  appName.value = await getName()

  const storedAvatar = await getAvatarFromDB(props.value || appName.value)
  if (storedAvatar) {
    avatar.value = `<img src="${storedAvatar}" class="text-0 w-full rounded-full">`
  } else {
    avatar.value = defaultAvatar.value
  }
}

onMounted(UpdateImg)
onUpdated(UpdateImg)

const loadImage = async () => {
  console.log(props.value || appName.value)
  console.log(props.enableReplace)

  if (!props.value)
    return
  const input = document.createElement('input')
  input.type = 'file'
  input.accept = 'image/jpeg,image/png,image/jpg';

  input.addEventListener('change', async (event) => {
    const target = event.target as HTMLInputElement
    const files = target.files

    if (files && files.length > 0) {
      const file = files[0]
      
      // 检查文件大小是否小于 5MB
      const fileSizeMB = file.size / (1024 * 1024);
      if (fileSizeMB > 5) {
        alert('File size must be less than 5MB');
        return;
      }

      const fileReader = new FileReader()
      fileReader.onload = async () => {
        const fileDataUrl = fileReader.result as string
        await saveAvatarToDB(props.value || appName.value, fileDataUrl)
        avatar.value = `<img src="${fileDataUrl}" class="text-0 w-full rounded-full">`
      }
      fileReader.readAsDataURL(file)
    }
  })

  input.click()
}

</script>

<template>
  <div class="text-0 w-full rounded-full" v-html="avatar" 
  @click="props.enableReplace ? loadImage() : undefined">
  </div>
</template>
