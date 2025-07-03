<script setup lang="ts">
import { ref, watch, onMounted, onUnmounted } from 'vue'
import FileProcessLayout from '../../components/FileProcessLayout.vue'
import { Activity } from '../../components/types'
import { FileContentProcessParam, ContentProcessProgressUpdateEvent } from './types'
import ParamForm from './ParamForm.vue'
import { invoke } from '@tauri-apps/api/core'
import { listen } from '@tauri-apps/api/event'

var unlisten: any
onMounted(() => {
  unlisten = listen<ContentProcessProgressUpdateEvent>('file_content_process_progress_update', (event) => {
    const processedFile = event.payload.progress
    if (fileContentProcessForm.value) {
      let files = fileContentProcessForm.value.filePaths.split(',')
      progressPercentage.value = Math.round(processedFile / files.length * 100)
    }
  })

  fileContentProcessForm.value = {
    filePaths: '',
    outputDir: '',
    searchString: '',
    regex: false,
    isReplace: false,
    replaceString: '',
    parallel: false,
  }
})
onUnmounted(() => {
  if (unlisten) {
    unlisten()
  }
})

const steps = ref<string[]>(['选择要搜索的文件', '确认搜索结果的输出路径', '输入要搜索的字符串', '开始搜索'])
const progressPercentage = ref(0)
const activities = ref<Activity[]>([])
const stepActive = ref(0)
const changeStepActive = (step: number) => {
  stepActive.value = step
}
const fileContentProcessForm = ref<FileContentProcessParam>({
  filePaths: '',
  outputDir: '',
  searchString: '',
  regex: false,
  isReplace: false,
  replaceString: '',
  parallel: false,
})

watch(() => fileContentProcessForm.value.isReplace, (newVal) => {
  steps.value = newVal
    ? ['选择要替换的文件', '确认替换结果的输出路径', '输入要替换的字符串', '开始替换']
    : ['选择要搜索的文件', '确认搜索结果的输出路径', '输入要搜索的字符串', '开始搜索']
})
watch(() => fileContentProcessForm.value.searchString, (newVal) => {
  if (newVal) {
    stepActive.value = 3
  }
})

const searchOrReplaceFile = async () => {
  stepActive.value = 4
  if (fileContentProcessForm.value.isReplace) {
    await replaceFile()
  } else {
    await searchFile()
  }
}
const searchFile = async () => {
  activities.value = []
  activities.value.push({
    content: '开始搜索',
    timestamp: new Date().toLocaleString(),
  })
  try {
    await invoke('file_search', {
      filePaths: fileContentProcessForm.value.filePaths,
      outputDir: fileContentProcessForm.value.outputDir,
      searchString: fileContentProcessForm.value.searchString,
      regex: fileContentProcessForm.value.regex,
      parallel: fileContentProcessForm.value.parallel,
    })
    activities.value.push({
      content: '搜索完成',
      timestamp: new Date().toLocaleString(),
    })
  } catch (err) {
    activities.value.push({
      content: `搜索文件失败：${err}`,
      timestamp: new Date().toLocaleString(),
    })
  }
}
const replaceFile = async () => {
  activities.value.push({
    content: '开始替换',
    timestamp: new Date().toLocaleString(),
  })
  try {
    await invoke('file_replace', {
      filePaths: fileContentProcessForm.value.filePaths,
      outputDir: fileContentProcessForm.value.outputDir,
      searchString: fileContentProcessForm.value.searchString,
      regex: fileContentProcessForm.value.regex,
      replaceString: fileContentProcessForm.value.replaceString,
    })
    activities.value.push({
      content: '替换完成',
      timestamp: new Date().toLocaleString(),
    })
  } catch (err) {
    activities.value.push({
      content: `替换文件失败：${err}`,
      timestamp: new Date().toLocaleString(),
    })
  }
}
</script>
<template>
  <FileProcessLayout title="文件内容处理" processTitle="处理进度" :steps="steps" :stepActive="stepActive"
    :progressPercentage="progressPercentage" :activities="activities">
    <template #tip>
      <p>此功能用于在文件内容中搜索指定字符串{{ fileContentProcessForm.isReplace ? '并进行替换' : '' }}</p>
      <p>注意：此功能不会修改原始文件，只会将搜索到的内容输出到新文件中</p>
      <p>操作步骤：</p>
      <p>1. 选择要{{ fileContentProcessForm.isReplace ? '替换' : '搜索' }}的文件(可多选)</p>
      <p>2. 确认{{ fileContentProcessForm.isReplace ? '替换' : '搜索' }}结果的输出路径</p>
      <p>3. 输入要{{ fileContentProcessForm.isReplace ? '替换' : '搜索' }}的字符串{{ fileContentProcessForm.isReplace ? "" : "(支持多个值)" }}</p>
      <p>4. 点击【开始{{ fileContentProcessForm.isReplace ? '替换' : '搜索' }}】按钮开始处理</p>
    </template>

    <template #param>
      <ParamForm v-model:fileContentProcessForm="fileContentProcessForm" @changeStepActive="changeStepActive" />
    </template>

    <template #processButton>
      <el-button v-show="fileContentProcessForm.filePaths !== '' && fileContentProcessForm.searchString !== ''" 
        type="primary" @click="searchOrReplaceFile">{{ fileContentProcessForm.isReplace ? '开始替换' : '开始搜索' }}</el-button>
    </template>
  </FileProcessLayout>
	<div>
    <!-- todo 展示搜索到的内容 分文件名tab -->
  </div>
</template>
