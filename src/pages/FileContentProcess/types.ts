export interface FileContentProcessParam {
  filePaths: string,
  outputDir: string,
  searchString: string,
  regex: boolean,
  isReplace: boolean,
  replaceString: string,
  // 仅【搜索】生效
  parallel: boolean,
}
export interface ContentProcessProgressUpdateEvent {
  progress: number,
}