<script setup lang="ts">
import { onMounted, onUnmounted, ref } from 'vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

import FileProcessLayout from '../../components/FileProcessLayout.vue'
import { type Activity } from '../../components/types'
import { FileSplitParam, type SplitProgressUpdateEvent } from './types.ts'
import ParamForm from './ParamForm.vue'

var unlisten: any
onMounted(() => {
  unlisten = listen<SplitProgressUpdateEvent>('file_split_progress_update', (event) => {
    const currentLine = event.payload.progress
    if (fileLines.value) {
      progressPercentage.value = Math.round(currentLine / fileLines.value * 100)
    }
  })

  fileForm.value = {
    filePath: '',
    outputDir: '',
    prefix: '',
    chunkSize: 0,
  }
})
onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})

const fileForm = ref<FileSplitParam>({
  filePath: '',
  outputDir: '',
  prefix: '',
  chunkSize: 0,
})

const fileLines = ref(0)
const stepActive = ref(0)
const changeStepActive = (step: number) => {
  stepActive.value = step
}
const steps = ref<string[]>(['选择要拆分的文件', '确认拆分的最大行数', '开始拆分'])
const progressPercentage = ref(0)
const activities = ref<Activity[]>([])


// 拆分文件
const splitFile = async () => {
  activities.value = []
  stepActive.value = 3
  // 获取文件行数
  activities.value.push({
    content: '开始获取文件行数',
    timestamp: new Date().toLocaleString(),
  })
  try {
    const tmpFileLines = await invoke('get_line_count', { filePath: fileForm.value.filePath }) || 0
    fileLines.value = parseInt(String(tmpFileLines))
    activities.value.push({
      content: `获取完成，行数：${tmpFileLines}`,
      timestamp: new Date().toLocaleString(),
    })
    // 处理 fileForm.value.chunkSize 可能为 undefined 的情况
    if (parseInt(String(tmpFileLines)) === 0 || (fileForm.value.chunkSize !== undefined && parseInt(String(tmpFileLines)) < fileForm.value.chunkSize)) {
      activities.value.push({
        content: '行数异常，无法拆分',
        timestamp: new Date().toLocaleString(),
      })
      return
    }
  } catch (err) {
    activities.value.push({
      content: `获取文件行数失败：${err}`,
      timestamp: new Date().toLocaleString(),
    })
    return
  }

  // 开始拆分
  activities.value.push({
    content: '开始拆分文件',
    timestamp: new Date().toLocaleString(),
  })

  // 调用Rust
  try {
    await invoke('file_split', {
      filePath: fileForm.value.filePath,
      outputDir: fileForm.value.outputDir,
      prefix: fileForm.value.prefix,
      chunkSize: fileForm.value.chunkSize,
    })
    activities.value.push({
      content: '拆分完成',
      timestamp: new Date().toLocaleString(),
    })
  } catch (err) {
    activities.value.push({
      content: `拆分文件失败：${err}`,
      timestamp: new Date().toLocaleString(),
    })
  }
}

const handleChunkSizeChange = (currentValue: number | undefined) => {
  if (currentValue) {
    stepActive.value = 2
  } else {
    stepActive.value = 1
  }
}

const startButtonControl = (): boolean => {
  return fileForm.value.filePath !== '' && fileForm.value.chunkSize !== undefined && fileForm.value.chunkSize > 0
}

</script>
<template>
  <FileProcessLayout title="文件拆分" processTitle="拆分进度" :steps="steps" :stepActive="stepActive"
    :progressPercentage="progressPercentage" :activities="activities">
    <template #tip>
      <p>此功能用于将【单个文件】按用户指定的【最大行数】拆分为多个文件</p>
      <p>操作步骤：</p>
      <p>1. 选择要拆分的文件</p>
      <p>2. 确认拆分的最大行数</p>
      <p>3. 点击【开始拆分】按钮开始执行拆分</p>
    </template>

    <template #param>
      <ParamForm v-model:fileForm="fileForm" @change-step-active="changeStepActive"
        @handle-chunk-size-change="handleChunkSizeChange" />
    </template>

    <template #processButton>
      <el-button v-show="startButtonControl" type="primary" @click="splitFile">开始拆分</el-button>
    </template>
  </FileProcessLayout>
</template>
