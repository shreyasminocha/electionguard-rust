// Copyright (C) Microsoft Corporation. All rights reserved.

#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]
#![deny(clippy::panic)]
#![deny(clippy::manual_assert)]

use lazy_static::lazy_static;
use num_bigint::BigUint;
use num_traits::Num;

use util::prime::BigUintPrime;

use crate::fixed_parameters::{FixedParameterGenerationParameters, FixedParameters, NumsNumber};

lazy_static! {
    /// Standard parameters, ElectionGuard latest (currently v2.0).
    pub static ref STANDARD_PARAMETERS: FixedParameters = make_standard_parameters_v2_0();
}

/// Standard parameters, ElectionGuard v2.0.
pub fn make_standard_parameters_v2_0() -> FixedParameters {
    FixedParameters {
        opt_version: Some([2, 0]),
        generation_parameters: FixedParameterGenerationParameters {
            q_bits_total: 256,
            p_bits_total: 4096,
            p_bits_msb_fixed_1: 256,
            p_middle_bits_source: NumsNumber::Ln2,
            p_bits_lsb_fixed_1: 256,
        },
        p: BigUintPrime::new_unchecked_the_caller_guarantees_that_this_number_is_prime(
            hex_to_biguint(
                "
                FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF
                B17217F7 D1CF79AB C9E3B398 03F2F6AF 40F34326 7298B62D 8A0D175B 8BAAFA2B
                E7B87620 6DEBAC98 559552FB 4AFA1B10 ED2EAE35 C1382144 27573B29 1169B825
                3E96CA16 224AE8C5 1ACBDA11 317C387E B9EA9BC3 B136603B 256FA0EC 7657F74B
                72CE87B1 9D6548CA F5DFA6BD 38303248 655FA187 2F20E3A2 DA2D97C5 0F3FD5C6
                07F4CA11 FB5BFB90 610D30F8 8FE551A2 EE569D6D FC1EFA15 7D2E23DE 1400B396
                17460775 DB8990E5 C943E732 B479CD33 CCCC4E65 9393514C 4C1A1E0B D1D6095D
                25669B33 3564A337 6A9C7F8A 5E148E82 074DB601 5CFE7AA3 0C480A54 17350D2C
                955D5179 B1E17B9D AE313CDB 6C606CB1 078F735D 1B2DB31B 5F50B518 5064C18B
                4D162DB3 B365853D 7598A195 1AE273EE 5570B6C6 8F969834 96D4E6D3 30AF889B
                44A02554 731CDC8E A17293D1 228A4EF9 8D6F5177 FBCF0755 268A5C1F 9538B982
                61AFFD44 6B1CA3CF 5E9222B8 8C66D3C5 422183ED C9942109 0BBB16FA F3D949F2
                36E02B20 CEE886B9 05C128D5 3D0BD2F9 62136319 6AF50302 0060E499 08391A0C
                57339BA2 BEBA7D05 2AC5B61C C4E9207C EF2F0CE2 D7373958 D7622658 90445744
                FB5F2DA4 B7510058 92D35689 0DEFE9CA D9B9D4B7 13E06162 A2D8FDD0 DF2FD608
                FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF",
            ),
        ),
        q: BigUintPrime::new_unchecked_the_caller_guarantees_that_this_number_is_prime(
            hex_to_biguint(
                "
                FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFF43",
            ),
        ),
        r: hex_to_biguint(
            "
                1
                00000000 00000000 00000000 00000000 00000000 00000000 00000000 000000BC
                B17217F7 D1CF79AB C9E3B398 03F2F6AF 40F34326 7298B62D 8A0D175B 8BAB857A
                E8F42816 5418806C 62B0EA36 355A3A73 E0C74198 5BF6A0E3 130179BF 2F0B43E3
                3AD86292 3861B8C9 F768C416 9519600B AD06093F 964B27E0 2D868312 31A9160D
                E48F4DA5 3D8AB5E6 9E386B69 4BEC1AE7 22D47579 249D5424 767C5C33 B9151E07
                C5C11D10 6AC446D3 30B47DB5 9D352E47 A53157DE 04461900 F6FE360D B897DF53
                16D87C94 AE71DAD0 BE84B647 C4BCF818 C23A2D4E BB53C702 A5C8062D 19F5E9B5
                033A94F7 FF732F54 12971286 9D97B8C9 6C412921 A9D86797 70F499A0 41C297CF
                F79D4C91 49EB6CAF 67B9EA3D C563D965 F3AAD137 7FF22DE9 C3E62068 DD0ED615
                1C37B4F7 4634C2BD 09DA912F D599F433 3A8D2CC0 05627DCA 37BAD43E 64A39631
                19C0BFE3 4810A21E E7CFC421 D53398CB C7A95B3B F585E5A0 4B790E2F E1FE9BC2
                64FDA810 9F6454A0 82F5EFB2 F37EA237 AA29DF32 0D6EA860 C41A9054 CCD24876
                C6253F66 7BFB0139 B5531FF3 01899612 02FD2B0D 55A75272 C7FD7334 3F7899BC
                A0B36A4C 470A64A0 09244C84 E77CEBC9 2417D5BB 13BF1816 7D8033EB 6C4DD787
                9FD4A7F5 29FD4A7F 529FD4A7 F529FD4A 7F529FD4 A7F529FD 4A7F529F D4A7F52A",
        ),
        g: hex_to_biguint(
            "
                36036FED 214F3B50 DC566D3A 312FE413 1FEE1C2B CE6D02EA 39B477AC 05F7F885
                F38CFE77 A7E45ACF 4029114C 4D7A9BFE 058BF2F9 95D2479D 3DDA618F FD910D3C
                4236AB2C FDD783A5 016F7465 CF59BBF4 5D24A22F 130F2D04 FE93B2D5 8BB9C1D1
                D27FC9A1 7D2AF49A 779F3FFB DCA22900 C14202EE 6C996160 34BE35CB CDD3E7BB
                7996ADFE 534B63CC A41E21FF 5DC778EB B1B86C53 BFBE9998 7D7AEA07 56237FB4
                0922139F 90A62F2A A8D9AD34 DFF799E3 3C857A64 68D001AC F3B681DB 87DC4242
                755E2AC5 A5027DB8 1984F033 C4D17837 1F273DBB 4FCEA1E6 28C23E52 759BC776
                5728035C EA26B44C 49A65666 889820A4 5C33DD37 EA4A1D00 CB62305C D541BE1E
                8A92685A 07012B1A 20A746C3 591A2DB3 815000D2 AACCFE43 DC49E828 C1ED7387
                466AFD8E 4BF19355 93B2A442 EEC271C5 0AD39F73 3797A1EA 11802A25 57916534
                662A6B7E 9A9E449A 24C8CFF8 09E79A4D 806EB681 119330E6 C57985E3 9B200B48
                93639FDF DEA49F76 AD1ACD99 7EBA1365 7541E79E C57437E5 04EDA9DD 01106151
                6C643FB3 0D6D58AF CCD28B73 FEDA29EC 12B01A5E B86399A5 93A9D5F4 50DE39CB
                92962C5E C6925348 DB54D128 FD99C14B 457F883E C20112A7 5A6A0581 D3D80A3B
                4EF09EC8 6F9552FF DA1653F1 33AA2534 983A6F31 B0EE4697 935A6B1E A2F75B85
                E7EBA151 BA486094 D68722B0 54633FEC 51CA3F29 B31E77E3 17B178B6 B9D8AE0F",
        ),
    }
}

/// Standard parameters, ElectionGuard v1.54.
pub fn make_standard_parameters_v1_54() -> FixedParameters {
    FixedParameters {
        opt_version: Some([1, 54]),
        generation_parameters: FixedParameterGenerationParameters {
            q_bits_total: 256,
            p_bits_total: 4096,
            p_bits_msb_fixed_1: 256,
            p_middle_bits_source: NumsNumber::EulerMascheroniConstant,
            p_bits_lsb_fixed_1: 256,
        },
        p: BigUintPrime::new_unchecked_the_caller_guarantees_that_this_number_is_prime(
            hex_to_biguint(
                "
                FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF
                93C467E3 7DB0C7A4 D1BE3F81 0152CB56 A1CECC3A F65CC019 0C03DF34 709AFFBD
                8E4B59FA 03A9F0EE D0649CCB 621057D1 1056AE91 32135A08 E43B4673 D74BAFEA
                58DEB878 CC86D733 DBE7BF38 154B36CF 8A96D156 7899AAAE 0C09D4C8 B6B7B86F
                D2A1EA1D E62FF864 3EC7C271 82797722 5E6AC2F0 BD61C746 961542A3 CE3BEA5D
                B54FE70E 63E6D09F 8FC28658 E80567A4 7CFDE60E E741E5D8 5A7BD469 31CED822
                03655949 64B83989 6FCAABCC C9B31959 C083F22A D3EE591C 32FAB2C7 448F2A05
                7DB2DB49 EE52E018 2741E538 65F004CC 8E704B7C 5C40BF30 4C4D8C4F 13EDF604
                7C555302 D2238D8C E11DF242 4F1B66C2 C5D238D0 744DB679 AF289048 7031F9C0
                AEA1C4BB 6FE9554E E528FDF1 B05E5B25 6223B2F0 9215F371 9F9C7CCC 69DDF172
                D0D62342 17FCC003 7F18B93E F5389130 B7A661E5 C26E5421 4068BBCA FEA32A67
                818BD307 5AD1F5C7 E9CC3D17 37FB2817 1BAF84DB B6612B78 81C1A48E 439CD03A
                92BF5222 5A2B38E6 542E9F72 2BCE15A3 81B5753E A8427633 81CCAE83 512B3051
                1B32E5E8 D8036214 9AD030AA BA5F3A57 98BB22AA 7EC1B6D0 F17903F4 E22D8407
                34AA8597 3F79A93F FB82A75C 47C03D43 D2F9CA02 D03199BA CEDDD453 3A52566A
                FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF",
            ),
        ),
        q: BigUintPrime::new_unchecked_the_caller_guarantees_that_this_number_is_prime(
            hex_to_biguint(
                "
                FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFFFF FFFFFF43",
            ),
        ),
        r: hex_to_biguint(
            "
            1
            00000000 00000000 00000000 00000000 00000000 00000000 00000000 000000BC
            93C467E3 7DB0C7A4 D1BE3F81 0152CB56 A1CECC3A F65CC019 0C03DF34 709B8AF6
            A64C0CED CF2D559D A9D97F09 5C3076C6 86037619 148D2C86 C317102A FA214803
            1F04440A C0FF0C9A 417A8921 2512E760 7B2501DA A4D38A2C 1410C483 6149E2BD
            B8C8260E 627C4646 963EFFE9 E16E495D 48BD215C 6D8EC9D1 667657A2 A1C8506F
            2113FFAD 19A6B2BC 7C457604 56719183 309F874B C9ACE570 FFDA877A A2B23A2D
            6F291C15 54CA2EB1 2F12CD00 9B8B8734 A64AD51E B893BD89 1750B851 62241D90
            8F0C9709 879758E7 E8233EAB 3BF2D6AB 53AFA32A A153AD66 82E5A064 8897C9BE
            18A0D50B ECE030C3 432336AD 9163E33F 8E7DAF49 8F14BB28 52AFFA81 4841EB18
            DD5F0E89 516D5577 76285C16 071D2111 94EE1C3F 34642036 AB886E3E C28882CE
            4003DEA3 35B4D935 BAE4B582 35B9FB2B AB713C8F 705A1C7D E4222020 9D6BBCAC
            C4673186 01565272 E4A63E38 E2499754 AE493AC1 A8E83469 EEF35CA2 7C271BC7
            92EEE211 56E617B9 22EA8F71 3C22CF28 2DC5D638 5BB12868 EB781278 FA0AB2A8
            958FCCB5 FFE2E5C3 61FC1744 20122B01 63CA4A46 308C8C46 C91EA745 7C136A7D
            9FD4A7F5 29FD4A7F 529FD4A7 F529FD4A 7F529FD4 A7F529FD 4A7F529F D4A7F52A",
        ),
        g: hex_to_biguint(
            "
            1D41E49C 477E15EA EEF0C5E4 AC08D4A4 6C268CD3 424FC01D 13769BDB 43673218
            587BC86C 4C1448D0 06A03699 F3ABAE5F EB19E296 F5D143CC 5E4A3FC8 9088C9F4
            523D166E E3AE9D5F B03C0BDD 77ADD5C0 17F6C55E 2EC92C22 6FEF5C6C 1DF2E7C3
            6D90E7EA ADE09824 1D340998 3BCCD2B5 379E9391 FBC62F9F 8D939D12 08B16036
            7C134264 12218959 5EC85C8C DBE5F9D3 07F46912 C04932F8 C16815A7 6B4682BD
            6BDC0ED5 2B00D8D3 0F59C731 D5A7FFAE 8165D53C F96649AA C2B743DA 56F14F19
            DACC5236 F29B1AB9 F9BEFC69 697293D5 DEAD8B5B F5DE9BAB 6DE67C45 719E5634
            4A3CBDF3 609824B1 B578E34E AEB6DD31 90AB3571 D6D671C5 12282C1D A7BD36B4
            251D2584 FADEA80B 9E141423 074DD9B5 FB83ACBD EAD4C87A 58FFF517 F977A830
            80370A3B 0CF98A1B C2978C47 AAC29611 FD6C40E2 F9875C35 D50443A9 AA3F4961
            1DCD3A0D 6FF3CB3F ACF31471 BDB61860 B92C594D 4E46569B B39FEEAD FF1FD64C
            836A6D6D B85C6BA7 241766B7 AB56BF73 9633B054 147F7170 921412E9 48D9E474
            02D15BB1 C2573186 12C121C3 6B80EB84 33C08E7D 0B7149E3 AB0A8735 A92EDCE8
            FF943E28 A2DCEACF CC69EC31 8909CB04 7BE1C585 8844B5AD 44F22EEB 289E4CC5
            54F7A5E2 F3DEA026 877FF928 51816071 CE028EB8 68D965CC B2D2295A 8C55BD1C
            070B39B0 9AE06B37 D29343B9 D8997DC2 44C468B9 80970731 736EE018 BBADB987",
        ),
    }
}

fn hex_to_biguint(s: &str) -> BigUint {
    let s = s.chars().filter(|c| !c.is_whitespace()).collect::<String>();

    // `unwrap()` is justified here because `s` is fixed at compile time.
    #[allow(clippy::unwrap_used)]
    BigUint::from_str_radix(s.as_str(), 16).unwrap()
}

#[cfg(test)]
mod test {
    use super::*;

    /// Validate the standard parameters v1.54.
    #[cfg(not(debug_assertions))] // This test is too slow without optimizations.
    #[test]
    fn standard_parameters_v1_54() {
        let mut csprng = util::csprng::Csprng::new(b"test::standard_parameters_v1_54");

        let standard_parameters_v1_54 = make_standard_parameters_v1_54();
        assert_eq!(standard_parameters_v1_54.opt_version, Some([1, 54]));
        assert!(standard_parameters_v1_54.validate(&mut csprng).is_ok());
    }

    /// Validate the standard parameters v2.0.
    #[cfg(not(debug_assertions))] // This test is too slow without optimizations.
    #[test]
    fn standard_parameters_v2_0() {
        let mut csprng = util::csprng::Csprng::new(b"test::standard_parameters_v2_0");

        let standard_parameters_v2_0 = make_standard_parameters_v2_0();
        assert_eq!(standard_parameters_v2_0.opt_version, Some([2, 0]));
        assert!(standard_parameters_v2_0.validate(&mut csprng).is_ok());
    }

    /// Verify that `pub static STANDARD_PARAMETERS` reflect the latest version (currently v2.0).
    #[test]
    fn standard_parameters_pub_static() {
        // Latest standard parameters are v2.0.
        assert_eq!(&*STANDARD_PARAMETERS, &make_standard_parameters_v2_0());
    }
}
