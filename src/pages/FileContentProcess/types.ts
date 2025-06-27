export interface FileContentProcessParam {
  filePaths: string,
  outputDir: string,
  searchString: string,
  regex: boolean,
  isReplace: boolean,
  replaceString: string,
}
export interface ContentProcessProgressUpdateEvent {
  progress: number,
}