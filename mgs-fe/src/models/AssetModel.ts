export default class AssetModel {
  constructor(
    readonly id: number,
    readonly type: string,
    readonly name: string,
    readonly description: string,
    readonly createdAt: number,
    readonly value: number
  ) {}
}
