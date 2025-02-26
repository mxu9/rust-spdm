// Copyright (c) 2021 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

#![allow(dead_code)]
#![allow(unused_variables)]
use codec::u24;
use codec::Codec;
use codec::Writer;
use spdmlib::config;
use spdmlib::crypto::hash;
use spdmlib::message::*;
use spdmlib::protocol::*;
use spdmlib::protocol::{
    SpdmBaseHashAlgo, SpdmDigestStruct, SpdmHKDFKeyStruct, SpdmMeasurementRecordStructure,
    SpdmMeasurementSpecification, SpdmMeasurementSummaryHashType, SpdmReqAsymAlgo,
    SpdmSignatureStruct,
};
use spdmlib::secret::*;

pub static SECRET_IMPL_INSTANCE: SpdmSecret = SpdmSecret {
    spdm_measurement_collection_cb: spdm_measurement_collection_impl,
    spdm_generate_measurement_summary_hash_cb: spdm_generate_measurement_summary_hash_impl,
    spdm_requester_data_sign_cb: spdm_requester_data_sign_impl,
    spdm_responder_data_sign_cb: spdm_responder_data_sign_impl,
    spdm_psk_handshake_secret_hkdf_expand_cb: spdm_psk_handshake_secret_hkdf_expand_impl,
    spdm_psk_master_secret_hkdf_expand_cb: spdm_psk_master_secret_hkdf_expand_impl,
};

#[allow(clippy::field_reassign_with_default)]
fn spdm_measurement_collection_impl(
    spdm_version: SpdmVersion,
    measurement_specification: SpdmMeasurementSpecification,
    measurement_hash_algo: SpdmBaseHashAlgo,
    measurement_index: usize,
) -> Option<SpdmMeasurementRecordStructure> {
    if measurement_specification != SpdmMeasurementSpecification::DMTF {
        None
    } else {
        let hashsize = SpdmBaseHashAlgo::get_size(&measurement_hash_algo);
        if measurement_index
            == SpdmMeasurementOperation::SpdmMeasurementQueryTotalNumber.get_u8() as usize
        {
            let mut dummy_spdm_measurement_record_structure =
                SpdmMeasurementRecordStructure::default();
            dummy_spdm_measurement_record_structure.number_of_blocks = 10;
            Some(dummy_spdm_measurement_record_structure)
        } else if measurement_index
            == SpdmMeasurementOperation::SpdmMeasurementRequestAll.get_u8() as usize
        {
            let mut firmware1: [u8; 8] = [0; 8];
            let mut firmware2: [u8; 8] = [0; 8];
            let mut firmware3: [u8; 8] = [0; 8];
            let mut firmware4: [u8; 8] = [0; 8];
            let mut firmware5: [u8; 8] = [0; 8];
            let mut firmware6: [u8; 8] = [0; 8];
            let mut firmware7: [u8; 8] = [0; 8];
            let mut firmware8: [u8; 8] = [0; 8];
            let mut firmware9: [u8; 8] = [0; 8];
            let mut firmware10: [u8; 8] = [0; 8];
            firmware1.copy_from_slice("deadbeef".as_bytes());
            firmware2.copy_from_slice("eadbeefd".as_bytes());
            firmware3.copy_from_slice("adbeefde".as_bytes());
            firmware4.copy_from_slice("dbeefdea".as_bytes());
            firmware5.copy_from_slice("beefdead".as_bytes());
            firmware6.copy_from_slice("deadbeef".as_bytes());
            firmware7.copy_from_slice("eadbeefd".as_bytes());
            firmware8.copy_from_slice("adbeefde".as_bytes());
            firmware9.copy_from_slice("dbeefdea".as_bytes());
            firmware10.copy_from_slice("beefdead".as_bytes());
            let digest1 =
                hash::hash_all(measurement_hash_algo, &firmware1).expect("hash_all failed!");
            let digest2 =
                hash::hash_all(measurement_hash_algo, &firmware2).expect("hash_all failed!");
            let digest3 =
                hash::hash_all(measurement_hash_algo, &firmware3).expect("hash_all failed!");
            let digest4 =
                hash::hash_all(measurement_hash_algo, &firmware4).expect("hash_all failed!");
            let digest5 =
                hash::hash_all(measurement_hash_algo, &firmware5).expect("hash_all failed!");
            let digest6 =
                hash::hash_all(measurement_hash_algo, &firmware6).expect("hash_all failed!");
            let digest7 =
                hash::hash_all(measurement_hash_algo, &firmware7).expect("hash_all failed!");
            let digest8 =
                hash::hash_all(measurement_hash_algo, &firmware8).expect("hash_all failed!");
            let digest9 =
                hash::hash_all(measurement_hash_algo, &firmware9).expect("hash_all failed!");
            let digest10 =
                hash::hash_all(measurement_hash_algo, &firmware10).expect("hash_all failed!");
            let mut digest_value1: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value2: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value3: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value4: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value5: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value6: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value7: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value8: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value9: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut digest_value10: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            digest_value1[..64].copy_from_slice(digest1.data.as_ref());
            digest_value2[..64].copy_from_slice(digest2.data.as_ref());
            digest_value3[..64].copy_from_slice(digest3.data.as_ref());
            digest_value4[..64].copy_from_slice(digest4.data.as_ref());
            digest_value5[..64].copy_from_slice(digest5.data.as_ref());
            digest_value6[..64].copy_from_slice(digest6.data.as_ref());
            digest_value7[..64].copy_from_slice(digest7.data.as_ref());
            digest_value8[..64].copy_from_slice(digest8.data.as_ref());
            digest_value9[..64].copy_from_slice(digest9.data.as_ref());
            digest_value10[..64].copy_from_slice(digest10.data.as_ref());

            let spdm_measurement_block_structure = SpdmMeasurementBlockStructure {
                index: measurement_index as u8,
                measurement_specification,
                measurement_size: digest1.data_size + 3,
                measurement: SpdmDmtfMeasurementStructure {
                    r#type: SpdmDmtfMeasurementType::SpdmDmtfMeasurementFirmware,
                    representation: SpdmDmtfMeasurementRepresentation::SpdmDmtfMeasurementDigest,
                    value_size: digest1.data_size,
                    value: digest_value1,
                },
            };

            let mut measurement_record_data = [0u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut writer = Writer::init(&mut measurement_record_data);
            for i in 0..10 {
                spdm_measurement_block_structure.encode(&mut writer);
            }

            Some(SpdmMeasurementRecordStructure {
                number_of_blocks: 10,
                measurement_record_length: u24::new(writer.used() as u32),
                measurement_record_data,
            })
        } else if measurement_index > 10 {
            None
        } else {
            let mut firmware: [u8; 8] = [0; 8];
            firmware.copy_from_slice("deadbeef".as_bytes());

            let digest = hash::hash_all(measurement_hash_algo, &firmware)?;

            let mut digest_value: [u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN] =
                [0; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            digest_value[(measurement_index) * SPDM_MAX_HASH_SIZE
                ..(measurement_index + 1) * SPDM_MAX_HASH_SIZE]
                .copy_from_slice(digest.data.as_ref());

            let spdm_measurement_block_structure = SpdmMeasurementBlockStructure {
                index: measurement_index as u8,
                measurement_specification,
                measurement_size: digest.data_size + 3,
                measurement: SpdmDmtfMeasurementStructure {
                    r#type: SpdmDmtfMeasurementType::SpdmDmtfMeasurementFirmware,
                    representation: SpdmDmtfMeasurementRepresentation::SpdmDmtfMeasurementDigest,
                    value_size: digest.data_size,
                    value: digest_value,
                },
            };

            let mut measurement_record_data = [0u8; config::MAX_SPDM_MEASUREMENT_VALUE_LEN];
            let mut writer = Writer::init(&mut measurement_record_data);
            spdm_measurement_block_structure.encode(&mut writer);

            Some(SpdmMeasurementRecordStructure {
                number_of_blocks: 1,
                measurement_record_length: u24::new(writer.used() as u32),
                measurement_record_data,
            })
        }
    }
}

fn spdm_generate_measurement_summary_hash_impl(
    spdm_version: SpdmVersion,
    base_hash_algo: SpdmBaseHashAlgo,
    measurement_specification: SpdmMeasurementSpecification,
    measurement_hash_algo: SpdmBaseHashAlgo,
    measurement_summary_hash_type: SpdmMeasurementSummaryHashType,
) -> Option<SpdmDigestStruct> {
    Some(SpdmDigestStruct::default())
}

fn spdm_requester_data_sign_impl(
    spdm_version: SpdmVersion,
    op_code: u8,
    req_base_asym_alg: SpdmReqAsymAlgo,
    base_hash_algo: SpdmBaseHashAlgo,
    is_data_hash: bool,
    message: &[u8],
    message_size: u8,
) -> Option<SpdmSignatureStruct> {
    Some(SpdmSignatureStruct::default())
}

fn spdm_responder_data_sign_impl(
    spdm_version: SpdmVersion,
    op_code: u8,
    req_base_asym_alg: SpdmReqAsymAlgo,
    base_hash_algo: SpdmBaseHashAlgo,
    is_data_hash: bool,
    message: &[u8],
    message_size: u8,
) -> Option<SpdmSignatureStruct> {
    Some(SpdmSignatureStruct::default())
}

fn spdm_psk_handshake_secret_hkdf_expand_impl(
    spdm_version: SpdmVersion,
    base_hash_algo: SpdmBaseHashAlgo,
    psk_hint: &[u8],
    psk_hint_size: Option<usize>,
    info: Option<&[u8]>,
    info_size: Option<usize>,
) -> Option<SpdmHKDFKeyStruct> {
    Some(SpdmHKDFKeyStruct::default())
}

fn spdm_psk_master_secret_hkdf_expand_impl(
    spdm_version: SpdmVersion,
    base_hash_algo: SpdmBaseHashAlgo,
    psk_hint: &[u8],
    psk_hint_size: Option<usize>,
    info: Option<&[u8]>,
    info_size: Option<usize>,
) -> Option<SpdmHKDFKeyStruct> {
    Some(SpdmHKDFKeyStruct::default())
}

#[cfg(all(test,))]
mod tests {
    use super::SECRET_IMPL_INSTANCE;
    use codec::Codec;
    use spdmlib::protocol::{
        SpdmBaseHashAlgo, SpdmMeasurementBlockStructure, SpdmMeasurementSpecification, SpdmVersion,
    };
    use spdmlib::secret::*;

    #[test]
    fn test_case0_spdm_measurement_collection() {
        let reg_result = register(SECRET_IMPL_INSTANCE.clone());
        assert_eq!(reg_result, true);

        let records = spdm_measurement_collection(
            SpdmVersion::SpdmVersion11,
            SpdmMeasurementSpecification::DMTF,
            SpdmBaseHashAlgo::TPM_ALG_SHA_512,
            1,
        );
        let deadbeefsha512 = [
            17, 58, 59, 199, 131, 216, 81, 252, 3, 115, 33, 75, 25, 234, 123, 233, 250, 61, 229,
            65, 236, 185, 254, 2, 109, 82, 198, 3, 232, 234, 25, 193, 116, 204, 14, 151, 5, 248,
            185, 13, 49, 34, 18, 192, 195, 166, 216, 69, 61, 223, 179, 227, 20, 20, 9, 207, 75,
            237, 200, 239, 3, 53, 144, 180,
        ];

        match records {
            Some(v) => {
                let spdm_measurement_block_structure =
                    SpdmMeasurementBlockStructure::read_bytes(&v.measurement_record_data).unwrap();
                assert_eq!(
                    deadbeefsha512,
                    &spdm_measurement_block_structure.measurement.value[0..64]
                );
            }
            None => {
                assert!(false)
            }
        }
    }
}
