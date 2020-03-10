enum AssetType {
  CURRENCY = 1,
  ETF = 2,
  CRYPTO = 3
}

//TODO evaluate dynamically
const assetTypes: string[] = Object.values(AssetType).filter(
  value => typeof value === "string"
) as string[];

export { AssetType, assetTypes };
