export interface FileMergeParam {
  filePaths: string,
  outputDir: string,
  outPutFileName: string,
  parallel: boolean,
}
export interface MergeProgressUpdateEvent {
  progress: number,
}