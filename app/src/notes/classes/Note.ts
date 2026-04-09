import { NyxLoader } from 'nyx-kit/classes'
import NoteMeta from './NoteMeta'

export default class Note {
  meta: NoteMeta
  content: string = ''

  constructor(data: unknown) {
    if (!data) throw new Error('Data is required')

    this.meta = new NoteMeta(NyxLoader.loadObject(data, 'meta', {}))
    this.content = NyxLoader.loadString(data, 'content')
  }
}
