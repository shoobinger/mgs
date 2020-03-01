export default class AssetModel {
  constructor(
    readonly id: number,
    readonly user_id: number,
    readonly name: string,
    readonly created_at: number,
    readonly enabled: boolean
  ) {}
}
