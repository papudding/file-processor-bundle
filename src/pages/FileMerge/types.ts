export interface FileMergeParam {
  filePaths: string,
  outputDir: string,
  outputFileName: string,
  parallel: boolean,
}
export interface MergeProgressUpdateEvent {
  progress: number,
}