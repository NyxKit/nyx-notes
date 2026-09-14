import NyxGateClass from './classes/NyxGate'
import { NyxGateDomain, MergeOption } from './types'

const NyxGate = new NyxGateClass({
  baseUrl: '',
  token: null,
  headers: {},
})

export { NyxGateDomain, MergeOption }
export default NyxGate
