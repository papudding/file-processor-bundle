<script setup lang="ts">
import { ref, onMounted, onUnmounted } from 'vue'
import FileProcessLayout from '../../components/FileProcessLayout.vue'
import { Activity } from '../../components/types'
import ParamForm from './ParamForm.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'
import { FileMergeParam, MergeProgressUpdateEvent } from './types'

var unlisten: any
onMounted(() => {
  unlisten = listen<MergeProgressUpdateEvent>('file_merge_progress_update', (event) => {
    const processedFile = event.payload.progress
    if (fileMergeForm.value) {
      let files = fileMergeForm.value.filePaths.split(',')
      progressPercentage.value = Math.round(processedFile / files.length * 100)
    }
  })

  fileMergeForm.value = {
    filePaths: '',
    outputDir: '',
    outputFileName: '',
    parallel: false,
  }
})
onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})


const steps = ref<string[]>(['选择要合并的文件', '确认合并后的输出路径', '开始合并'])
const progressPercentage = ref(0)
const activities = ref<Activity[]>([])
const stepActive = ref(0)
const changeStepActive = (step: number) => {
  stepActive.value = step
}
const fileMergeForm = ref<FileMergeParam>({
  filePaths: '',
  outputDir: '',
  outputFileName: '',
  parallel: false,
})

const mergeFile = async ()=> {
  activities.value = []
  stepActive.value = 3
  // 获取文件行数
  activities.value.push({
    content: '开始合并',
    timestamp: new Date().toLocaleString(),
  })
  try {
    await invoke('file_merge', {
      filePaths: fileMergeForm.value.filePaths,
      outputDir: fileMergeForm.value.outputDir,
      outputFileName: fileMergeForm.value.outputFileName,
      parallel: fileMergeForm.value.parallel,
    })
    activities.value.push({
      content: '合并完成',
      timestamp: new Date().toLocaleString(),
    })
  } catch (err) {
    activities.value.push({
      content: `合并文件失败：${err}`,
      timestamp: new Date().toLocaleString(),
    })
  }
}
</script>
<template>
  <FileProcessLayout title="文件合并" processTitle="合并进度" :steps="steps" :stepActive="stepActive"
    :progressPercentage="progressPercentage" :activities="activities">
    <template #tip>
      <p>此功能用于将用户选择的【多个文件】合并成一个文件</p>
      <p>操作步骤：</p>
      <p>1. 选择要合并的文件</p>
      <p>2. 确认合并后的输出路径</p>
      <p>3. 点击【开始合并】按钮开始执行合并</p>
    </template>

    <template #param>
      <ParamForm v-model:fileMergeForm="fileMergeForm" @changeStepActive="changeStepActive" />
    </template>

    <template #processButton>
      <el-button v-show="fileMergeForm.filePaths !== ''" type="primary" @click="mergeFile">开始合并</el-button>
    </template>
  </FileProcessLayout>
</template>