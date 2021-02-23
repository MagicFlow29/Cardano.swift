use super::data::CData;
use super::ptr::Ptr;
use std::convert::{TryInto, TryFrom};
use super::error::CError;
use super::panic::*;
use cardano_serialization_lib::{AssetName as RAssetName};

#[repr(C)]
#[derive(Copy, Clone)]
pub struct AssetName {
  bytes: [u8; 32],
  len: u8
}

impl TryFrom<RAssetName> for AssetName {
  type Error = CError;

  fn try_from(asset: RAssetName) -> Result<Self> {
    let name = asset.name();
    let len = name.len();
    let bytes: [u8; 32] = name.try_into().map_err(|_| CError::DataLengthMismatch)?;
    Ok(Self { bytes: bytes, len: len as u8 })
  }
}

impl TryFrom<AssetName> for RAssetName {
  type Error = CError;

  fn try_from(asset: AssetName) -> Result<Self> {
    let mut bytes = Vec::from(asset.bytes);
    bytes.truncate(asset.len.into());
    RAssetName::new(bytes).into_result()
  }
}

#[no_mangle]
pub unsafe extern "C" fn cardano_asset_name_to_bytes(
  asset_name: AssetName, result: &mut CData, error: &mut CError
) -> bool {
  handle_exception_result(|| {
    asset_name.try_into().map(|name: RAssetName| name.to_bytes().into())
  }).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn cardano_asset_name_from_bytes(
  data: CData, result: &mut AssetName, error: &mut CError
) -> bool {
  handle_exception_result(|| {
    data.unowned()
      .and_then(|bytes| RAssetName::from_bytes(bytes.into()).into_result())
      .and_then(|asset| asset.try_into())
  }).response(result, error)
}

#[no_mangle]
pub unsafe extern "C" fn cardano_asset_name_new(
  data: CData, result: &mut AssetName, error: &mut CError
) -> bool {
  handle_exception_result(|| {
    data.unowned()
      .and_then(|bytes| RAssetName::new(bytes.into()).into_result())
      .and_then(|asset| asset.try_into())
  })
  .response(result, error)
}