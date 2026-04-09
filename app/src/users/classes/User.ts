import { NyxLoader } from 'nyx-kit/classes'
import { ServerRole } from '@/shared/types/enums'

const SERVER_ROLES = Object.values(ServerRole)

export default class User {
  id!: string
  email!: string
  display_name: string = 'Anonymous User'
  role: ServerRole = ServerRole.User

  constructor(data: unknown) {
    if (!data) throw new Error('Data is required')

    this.id = NyxLoader.loadString(data, 'id')
    this.email = NyxLoader.loadString(data, 'email')
    this.display_name = NyxLoader.loadString(data, 'display_name', this.display_name)
    this.role = NyxLoader.loadEnum(data, 'role', ServerRole.User, SERVER_ROLES)
  }
}
