<script setup lang="ts">
import { ref } from 'vue'
import { FileContentProcessParam } from './types'
import { open } from '@tauri-apps/plugin-dialog'
// import { ElNotification } from 'element-plus'

const emit = defineEmits<{
  (e: 'changeStepActive', step: number): void
}>()

const fileContentProcessForm = defineModel<FileContentProcessParam>('fileContentProcessForm', {
  default: () => ({
    filePaths: '',
    outputDir: '',
    searchString: '',
    regex: false,
    isReplace: false,
    replaceString: '',
  })
})
const fileNames = ref<string>('')
const fileContentProcessFormRules = ref({
  filePaths: [
    { required: true, message: '请选择文件', trigger: 'blur' },
  ],
  outputDir: [
    { required: true, message: '请选择输出目录', trigger: 'blur' },
  ],
})

// 处理选择文件
const selectFile = async () => {
  // Open a dialog
  const files = await open({
    multiple: true,
    directory: false,
  })

  if (files && files.length) {
    fileNames.value = files.map((file) => file.split(/[\\/]/).pop() || '').join('\n')
    fileContentProcessForm.value.filePaths = files.join(',')
    emit('changeStepActive', 2)
  }
}
// 选择目录
const selectDir = async () => {
  // const file = await open({
  //   multiple: false,
  //   directory: true,
  // })
  // 处理 file 可能为 null 的情况，若为 null 则赋值为空字符串
  // fileContentProcessForm.value.outputDir = file || ''
}
</script>
<template>
  <el-form style="min-width: 330px;" label-position="top" :model="fileContentProcessForm"
    :rules="fileContentProcessFormRules">
    <el-form-item :label="fileContentProcessForm.isReplace ? '待替换文件' : '待搜索文件'" prop="filePaths">
      <el-row>
        <el-col :span="19">
          <el-input :autosize="{ minRows: 2, maxRows: 5 }" type="textarea" readonly v-model="fileNames"
            placeholder="点击按钮选择文件" />
          <el-input v-show="false" hidden readonly v-model="fileContentProcessForm.filePaths" />
        </el-col>
        <el-col :span="4" :offset="1">
          <el-button type="primary" @click="selectFile">选择文件</el-button>
        </el-col>
      </el-row>
    </el-form-item>
    <el-form-item :label="fileContentProcessForm.isReplace ? '替换后输出的文件路径' : '搜索结果保存的文件路径'" prop="outputDir">
      <el-row>
        <el-col :span="19">
          <el-input readonly v-model="fileContentProcessForm.outputDir" placeholder="点击按钮选择输出路径" />
        </el-col>
        <el-col :span="4" :offset="1">
          <el-button type="primary" @click="selectDir">选择输出</el-button>
        </el-col>
      </el-row>
    </el-form-item>
    <el-row>
      <el-col :span="12">
        <el-form-item :label="fileContentProcessForm.isReplace ? '要替换的字符串' : '要搜索的字符串'" prop="searchString">
          <el-input v-model="fileContentProcessForm.searchString" placeholder="请输入合并后的文件名" />
        </el-form-item>
      </el-col>
      <el-col :span="11" :offset="1">
        <div class="control-checks">
          <div class="control-check">
            <el-form-item prop="isReplace">
              <el-checkbox v-model="fileContentProcessForm.isReplace" label="替换模式" />
              <el-icon>
                <el-tooltip placement="bottom" content="【替换模式】会输出替换后的完整内容">
                  <InfoFilled />
                </el-tooltip>
              </el-icon>
            </el-form-item>
          </div>
          <div class="control-check">
            <el-form-item prop="regex">
              <el-checkbox v-model="fileContentProcessForm.regex" label="正则匹配" />
              <el-icon>
                <el-tooltip placement="bottom"
                  :content="'勾选后，【要' + (fileContentProcessForm.isReplace ? '替换' : '搜索') + '的字符串】为正则表达式'">
                  <InfoFilled />
                </el-tooltip>
              </el-icon>
            </el-form-item>
          </div>
        </div>
      </el-col>
    </el-row>
    <el-form-item v-if="fileContentProcessForm.isReplace" label="替换内容" prop="replaceString">
      <el-input v-model="fileContentProcessForm.replaceString" placeholder="请输入替换后的字符串" />
    </el-form-item>
  </el-form>
</template>
<style scoped>
.control-checks {
  display: flex;
  flex-direction: column;
  gap: 1px;
}
.control-check {
  height: 30px;
}
</style>