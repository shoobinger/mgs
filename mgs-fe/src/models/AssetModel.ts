export default class AssetModel {
  constructor(
    readonly id: number,
    readonly name: string,
    readonly createdAt: number,
    readonly enabled: boolean
  ) {}
}
