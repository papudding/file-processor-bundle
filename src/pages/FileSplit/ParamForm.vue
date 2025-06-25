<script setup lang="ts">
import { ref } from 'vue'
import { type FileSplitParam } from './types.ts'
import { open } from '@tauri-apps/plugin-dialog'

const emit = defineEmits<{
  (e: 'changeStepActive', step: number): void
  (e: 'handleChunkSizeChange', chunkSize: number): void
}>()

const fileForm = defineModel<FileSplitParam>('fileForm', {
  default: () => ({
    filePath: '',
    outputDir: '',
    prefix: '',
    chunkSize: 0,
  })
})

const fileName = ref<string>('')

const fileFormRules = ref({
  filePath: [
    { required: true, message: '请选择文件', trigger: 'blur' },
  ],
})

// 处理选择文件
const selectFile = async () => {
  // Open a dialog
  const file = await open({
    multiple: false,
    directory: false,
  })
  // 处理 file 可能为 null 的情况，若为 null 则赋值为空字符串
  fileForm.value.filePath = file || ''
  
  if (file) {
    fileName.value = file.split(/[\\/]/).pop() || ''
    emit('changeStepActive', 1)
  }
}

// 选择目录
const selectDir = async () => {
  const file = await open({
    multiple: false,
    directory: true,
  })
  // 处理 file 可能为 null 的情况，若为 null 则赋值为空字符串
  fileForm.value.outputDir = file || ''
}
</script>
<template>
  <el-form label-position="top" :model="fileForm" :rules="fileFormRules">
    <el-form-item label="待拆分文件" prop="filePath">
      <el-row>
        <el-col :span="19">
          <el-input readonly v-model="fileName" placeholder="点击按钮选择文件" />
          <el-input v-show="false" hidden readonly v-model="fileForm.filePath" />
        </el-col>
        <el-col :span="4" :offset="1">
          <el-button type="primary" @click="selectFile">选择文件</el-button>
        </el-col>
      </el-row>
    </el-form-item>
    <el-form-item prop="outputDir">
      <template #label>
        <span>拆分后的文件路径
          <el-icon>
            <el-tooltip placement="right" content="未指定则默认为待拆分文件同目录">
              <InfoFilled />
            </el-tooltip>
          </el-icon>
        </span>
      </template>
      <el-row>
        <el-col :span="19">
          <el-input readonly v-model="fileForm.outputDir" placeholder="点击按钮选择输出路径" />
        </el-col>
        <el-col :span="4" :offset="1">
          <el-button type="primary" @click="selectDir">选择输出</el-button>
        </el-col>
      </el-row>
    </el-form-item>
    <el-row>
      <el-col :span="12">
        <el-form-item prop="prefix">
          <template #label>
            <span>拆分后的文件名前缀
              <el-icon>
                <el-tooltip placement="right" content="未指定则默认为“chunk”">
                  <InfoFilled />
                </el-tooltip>
              </el-icon>
            </span>
          </template>
          <el-input v-model="fileForm.prefix" placeholder="请输入文件名前缀" />
        </el-form-item>
      </el-col>
      <el-col :span="11" :offset="1">
        <el-form-item label="拆分的最大行数" prop="chunkSize">
          <el-input-number :disabled="fileForm.filePath === undefined || fileForm.filePath === ''"
            v-model="fileForm.chunkSize" @change="emit('handleChunkSizeChange', $event)" placeholder="指定最大行数" :step="1" />
        </el-form-item>
      </el-col>
    </el-row>
  </el-form>
</template>