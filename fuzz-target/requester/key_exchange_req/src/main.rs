// Copyright (c) 2020 Intel Corporation
//
// SPDX-License-Identifier: BSD-2-Clause-Patent

use fuzzlib::*;
use spdmlib::protocol::*;

fn fuzz_send_receive_spdm_key_exchange(fuzzdata: &[u8]) {
    let (rsp_config_info, rsp_provision_info) = rsp_create_info();
    let (req_config_info, req_provision_info) = req_create_info();
    let (rsp_config_info1, rsp_provision_info1) = rsp_create_info();
    let (req_config_info1, req_provision_info1) = req_create_info();
    {
        let shared_buffer = SharedBuffer::new();
        let mut device_io_responder = FakeSpdmDeviceIoReceve::new(&shared_buffer);

        let pcidoe_transport_encap = &mut PciDoeTransportEncap {};

        spdmlib::crypto::asym_sign::register(ASYM_SIGN_IMPL.clone());

        let message_m = &[
            0x11, 0xe0, 0x00, 0x00, 0x11, 0x60, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut responder = responder::ResponderContext::new(
            &mut device_io_responder,
            pcidoe_transport_encap,
            rsp_config_info,
            rsp_provision_info,
        );

        responder.common.provision_info.my_cert_chain = Some(SpdmCertChainData {
            data_size: 512u16,
            data: [0u8; config::MAX_SPDM_CERT_CHAIN_DATA_SIZE],
        });

        responder.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
        responder.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_128_GCM;
        responder.common.negotiate_info.dhe_sel = SpdmDheAlgo::SECP_384_R1;
        responder.common.negotiate_info.base_asym_sel =
            SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;

        responder.common.reset_runtime_info();

        #[cfg(feature = "hashed-transcript-data")]
        responder.common.runtime_info.digest_context_m1m2.is_some();
        // responder.common.peer_info.peer_cert_chain.cert_chain = REQ_CERT_CHAIN_DATA;

        let pcidoe_transport_encap2 = &mut PciDoeTransportEncap {};
        let mut device_io_requester =
            fake_device_io::FakeSpdmDeviceIo::new(&shared_buffer, &mut responder);

        let mut requester = requester::RequesterContext::new(
            &mut device_io_requester,
            pcidoe_transport_encap2,
            req_config_info,
            req_provision_info,
        );

        requester.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
        requester.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_128_GCM;
        requester.common.negotiate_info.dhe_sel = SpdmDheAlgo::SECP_384_R1;
        requester.common.negotiate_info.base_asym_sel =
            SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;

        requester.common.reset_runtime_info();

        #[cfg(feature = "hashed-transcript-data")]
        requester.common.runtime_info.digest_context_m1m2.is_some();
        // requester.common.peer_info.peer_cert_chain.cert_chain = REQ_CERT_CHAIN_DATA;

        let _ = requester.send_receive_spdm_key_exchange(
            0,
            SpdmMeasurementSummaryHashType::SpdmMeasurementSummaryHashTypeNone,
        );
    }
    {
        let shared_buffer = SharedBuffer::new();
        let mut device_io_responder = FuzzSpdmDeviceIoReceve::new(&shared_buffer, fuzzdata);

        let pcidoe_transport_encap = &mut PciDoeTransportEncap {};

        spdmlib::crypto::asym_sign::register(ASYM_SIGN_IMPL.clone());

        let message_m = &[
            0x11, 0xe0, 0x00, 0x00, 0x11, 0x60, 0x00, 0x01, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        ];
        let mut responder = responder::ResponderContext::new(
            &mut device_io_responder,
            pcidoe_transport_encap,
            rsp_config_info1,
            rsp_provision_info1,
        );

        responder.common.provision_info.my_cert_chain = Some(SpdmCertChainData {
            data_size: 512u16,
            data: [0u8; config::MAX_SPDM_CERT_CHAIN_DATA_SIZE],
        });

        responder.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
        responder.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_128_GCM;
        responder.common.negotiate_info.dhe_sel = SpdmDheAlgo::SECP_384_R1;
        responder.common.negotiate_info.base_asym_sel =
            SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;

        responder.common.reset_runtime_info();

        #[cfg(feature = "hashed-transcript-data")]
        responder.common.runtime_info.digest_context_m1m2.is_some();
        // responder.common.peer_info.peer_cert_chain.cert_chain = REQ_CERT_CHAIN_DATA;

        let pcidoe_transport_encap2 = &mut PciDoeTransportEncap {};
        let mut device_io_requester =
            fake_device_io::FakeSpdmDeviceIo::new(&shared_buffer, &mut responder);

        let mut requester = requester::RequesterContext::new(
            &mut device_io_requester,
            pcidoe_transport_encap2,
            req_config_info1,
            req_provision_info1,
        );

        requester.common.negotiate_info.base_hash_sel = SpdmBaseHashAlgo::TPM_ALG_SHA_384;
        requester.common.negotiate_info.aead_sel = SpdmAeadAlgo::AES_128_GCM;
        requester.common.negotiate_info.dhe_sel = SpdmDheAlgo::SECP_384_R1;
        requester.common.negotiate_info.base_asym_sel =
            SpdmBaseAsymAlgo::TPM_ALG_ECDSA_ECC_NIST_P384;

        requester.common.reset_runtime_info();

        #[cfg(feature = "hashed-transcript-data")]
        requester.common.runtime_info.digest_context_m1m2.is_some();
        // requester.common.peer_info.peer_cert_chain.cert_chain = REQ_CERT_CHAIN_DATA;

        let _ = requester.send_receive_spdm_key_exchange(
            0,
            SpdmMeasurementSummaryHashType::SpdmMeasurementSummaryHashTypeNone,
        );
    }
}

fn main() {
    #[cfg(all(feature = "fuzzlogfile", feature = "fuzz"))]
    flexi_logger::Logger::try_with_str("info")
        .unwrap()
        .log_to_file(
            FileSpec::default()
                .directory("traces")
                .basename("foo")
                .discriminant("Sample4711A")
                .suffix("trc"),
        )
        .print_message()
        .create_symlink("current_run")
        .start()
        .unwrap();

    #[cfg(not(feature = "fuzz"))]
    {
        let args: Vec<String> = std::env::args().collect();
        if args.len() < 2 {
            // Here you can replace the single-step debugging value in the fuzzdata array.
            let fuzzdata = [
                0x1, 0x0, 0x1, 0x0, 0x4d, 0x0, 0x0, 0x0, 0x11, 0x64, 0x0, 0x0, 0xfe, 0xff, 0x0,
                0x0, 0x61, 0x10, 0xf0, 0x2c, 0x73, 0x72, 0xb8, 0x4e, 0x45, 0x2d, 0x68, 0x5c, 0xe1,
                0x23, 0xd2, 0x10, 0x4b, 0x74, 0x13, 0x7d, 0xd7, 0xfa, 0xa2, 0x95, 0xab, 0x14, 0x45,
                0x26, 0x6, 0xbf, 0xdb, 0x2a, 0xac, 0x52, 0xc3, 0x2f, 0x5d, 0x9, 0x81, 0x19, 0xfb,
                0x2, 0xf9, 0x7b, 0xfc, 0xa6, 0xfb, 0x72, 0xc6, 0x1b, 0xc5, 0xc4, 0xcb, 0x59, 0x81,
                0xd4, 0x35, 0xb3, 0xd2, 0x7d, 0x87, 0xb, 0x3d, 0x72, 0x43, 0x68, 0x3d, 0xc0, 0x49,
                0xbd, 0x41, 0xd3, 0xa8, 0xbd, 0xad, 0xf, 0x46, 0x1d, 0xb8, 0x50, 0x83, 0xe2, 0xb6,
                0xe8, 0x43, 0x4b, 0x8c, 0x98, 0x22, 0xb, 0x82, 0x40, 0xf8, 0xb9, 0x44, 0xda, 0x91,
                0x7c, 0xf3, 0xa4, 0x3e, 0x6f, 0xa7, 0x92, 0xd9, 0x2f, 0x5d, 0x3c, 0x35, 0xa3, 0xd,
                0x7e, 0xbf, 0x8f, 0x43, 0x1, 0xf8, 0xe, 0x65, 0x9d, 0x20, 0xc2, 0xf5, 0xfb, 0x4c,
                0x83, 0xa5, 0x78, 0x10, 0x0, 0x46, 0x54, 0x4d, 0x44, 0x1, 0x1, 0x0, 0x0, 0x0, 0x0,
                0x4, 0x0, 0x1, 0x0, 0x0, 0x11, 0xce, 0x78, 0xc5, 0xb6, 0xa9, 0xcb, 0x5b, 0xe5,
                0x57, 0xe6, 0xb6, 0x71, 0xf5, 0xeb, 0xa, 0x6, 0x18, 0x43, 0x97, 0x87, 0x92, 0x11,
                0x9c, 0x41, 0x1b, 0xb, 0xf6, 0xfd, 0x3c, 0x74, 0xad, 0x34, 0xaf, 0xf5, 0x8a, 0x31,
                0x2, 0x20, 0x58, 0x57, 0x17, 0x33, 0x13, 0x6b, 0x5, 0x5, 0x67, 0x7f, 0xee, 0x2a,
                0xd0, 0x4a, 0x47, 0xe3, 0xd9, 0x20, 0x58, 0xcd, 0x6e, 0x6a, 0xe6, 0x24, 0x51, 0x77,
                0x22, 0xab, 0x7, 0x2c, 0x9b, 0x4a, 0xe5, 0x2c, 0x55, 0x7f, 0x8c, 0x5b, 0x4a, 0x54,
                0x65, 0xd8, 0xd, 0xb, 0xcd, 0xec, 0x9b, 0xa5, 0xac, 0xee, 0x31, 0x77, 0x57, 0xa3,
                0x8a, 0x79, 0x34, 0xd, 0xd1, 0xbe, 0xf, 0x15, 0x81, 0x2a, 0xe4, 0x7a, 0xd, 0xdc,
                0xf9, 0x62, 0x52, 0xf3, 0x1a, 0x9f, 0x30, 0x29, 0xb4, 0x8e, 0x12, 0x68, 0x10, 0xd1,
                0xd1, 0x40, 0x8f, 0x9c, 0x98, 0xc5, 0xd2, 0x9f, 0x3e, 0x6f, 0x9b, 0xf8, 0x10, 0x21,
                0xfd, 0x34, 0x29, 0x15, 0x76, 0x72, 0x9c, 0xf7, 0x5f, 0x96, 0x0, 0x0,
            ];
            fuzz_send_receive_spdm_key_exchange(&fuzzdata);
        } else {
            let path = &args[1];
            let data = std::fs::read(path).expect("read crash file fail");
            fuzz_send_receive_spdm_key_exchange(data.as_slice());
        }
    }
    #[cfg(feature = "fuzz")]
    afl::fuzz!(|data: &[u8]| {
        fuzz_send_receive_spdm_key_exchange(data);
    });
}
