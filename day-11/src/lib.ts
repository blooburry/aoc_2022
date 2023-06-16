export interface Monkey {
  id: number
  items: bigint[]
  operation: (item: bigint) => number
  throwTo: (thrownItem: bigint) => number
  monkeyBussiness: number
}
