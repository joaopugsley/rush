<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/tauri'
import { appWindow } from '@tauri-apps/api/window'

const inputRef = ref<HTMLInputElement | null>(null);

const handleKeyDown = async (event: KeyboardEvent) => {
  if (event.key === 'Escape') {
    await invoke('hide_window');
  }
}

const focusInput = () => {
  if (!inputRef.value) return;
  inputRef.value.focus();
}

const resetInput = () => {
  if (!inputRef.value) return;
  inputRef.value.value = "";
}

onMounted(() => {
  window.addEventListener('keydown', handleKeyDown);

  appWindow.onFocusChanged(({ event, payload: focused }) => {
    if (focused) {
      focusInput();
    } else {
      resetInput();
    }
  })

})

onUnmounted(() => {
  window.removeEventListener('keydown', handleKeyDown);
})
</script>

<template>
  <div class="relative [&_*]:z-10 flex flex-col items-center border-zinc-800 bg-zinc-900 bg-gradient-to-br drop-shadow-md p-3 border rounded-2xl w-[540px] h-[360px] overflow-hidden">
    <input
      ref="inputRef"
      type="text"
      placeholder="Search here"
      class="border-zinc-800 bg-zinc-900 p-2 border rounded-xl w-full h-10 text-zinc-50 placeholder:text-zinc-400/40"
    />
    <ul class="">
      <li></li>
    </ul>
  </div>
</template>