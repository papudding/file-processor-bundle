export interface SplitProgressUpdateEvent {
  progress: number,
}

export interface FileSplitParam {
  filePath: string,
  outputDir: string,
  prefix: string,
  chunkSize: number,
}